use std::str::FromStr;

use crate::derivative::{base_deriv, head_to_tail_deriv, inv_imbalance_deriv, tail_to_head_deriv};
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, OpenOrder, Position, QueryMsg, WrappedGetActionResponse, WrappedOpenOrder, WrappedOrderResponse, WrappedPosition,
};
use crate::spot::{create_new_orders_spot, inv_imbalance_spot};
use crate::state::{config, config_read, State};
use crate::utils::{div_dec, sub_abs, wrap};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Coin, Decimal256 as Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint256,
};
use injective_bindings::{create_subaccount_transfer_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper, SubaccountDepositResponse};

#[entry_point]
pub fn instantiate(deps: DepsMut<InjectiveQueryWrapper>, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<Response, StdError> {
    let state = State {
        market_id: msg.market_id.to_string(),
        manager: info.sender.clone().into(),
        sub_account: msg.sub_account.clone(),
        fee_recipient: msg.fee_recipient.clone(),
        risk_aversion: Decimal::from_str(&msg.risk_aversion.clone()).unwrap(),
        order_density: Uint256::from_str(&msg.order_density.clone()).unwrap(),
        active_capital_perct: Decimal::from_str(&msg.active_capital_perct.clone()).unwrap(),
        decimal_shift: Uint256::from_str(&msg.decimal_shift.clone()).unwrap(),
        max_notional_position: Decimal::from_str(&msg.max_notional_position.clone()).unwrap(),
        min_pnl: Decimal::from_str(&msg.min_pnl.clone()).unwrap(),
        manual_offset_perct: Decimal::from_str(&msg.manual_offset_perct.clone()).unwrap(),
        tail_dist_to_head_bp: Decimal::from_str(&msg.tail_dist_to_head_bp.clone()).unwrap(),
        head_chg_tol_bp: Decimal::from_str(&msg.head_chg_tol_bp.clone()).unwrap(),
        max_dd: Decimal::from_str(&msg.max_dd.clone()).unwrap(),
        leverage: Decimal::from_str(&msg.leverage.clone()).unwrap(),
        base_precision_shift: Uint256::from_str(&msg.base_precision_shift.clone()).unwrap(),
    };

    config(deps.storage).save(&state)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Subscribe { subaccount_id, amount } => subscribe(deps, env, info.sender, subaccount_id, amount),
    }
}

pub fn subscribe(
    _deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    subaccount_id: String,
    amount: Coin,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let contract = env.contract.address;

    let querier = InjectiveQuerier::new(&_deps.querier);
    let res: SubaccountDepositResponse = querier.query_subaccount_deposit(subaccount_id.clone(), amount.denom.clone().into())?;

    // just log the available balance for now
    _deps.api.debug(res.deposits.available_balance.to_string().as_str());

    let msg = create_subaccount_transfer_msg(sender, subaccount_id.into(), contract.into(), amount);

    let res = Response::new().add_message(msg);
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        QueryMsg::GetAction {
            is_deriv,
            open_orders,
            position,
            inv_base_val,
            inv_val,
            std_dev,
            mid_price,
        } => to_binary(&get_action(
            deps,
            is_deriv,
            open_orders,
            position,
            inv_base_val,
            inv_val,
            std_dev,
            mid_price,
        )?),
    }
}

fn get_action(
    deps: Deps<InjectiveQueryWrapper>,
    is_deriv: bool,
    open_orders: Vec<OpenOrder>,
    position: Option<Position>,
    inv_base_val: String,
    inv_val: String,
    std_dev: String,
    mid_price: String,
) -> StdResult<WrappedGetActionResponse> {
    // Wrap everything
    let open_orders: Vec<WrappedOpenOrder> = open_orders.into_iter().map(|o| o.wrap(deps).unwrap()).collect();
    let position = match position {
        None => None,
        Some(p) => Some(p.wrap(deps).unwrap()),
    };
    let inv_base_val = wrap(&inv_base_val, deps);
    let inv_val = wrap(&inv_val, deps);
    let std_dev = wrap(&std_dev, deps);
    let mid_price = wrap(&mid_price, deps);
    let varience = std_dev * std_dev;

    // Load state
    let state = config_read(deps.storage).load().unwrap();

    // Calculate inventory imbalance parameter
    let (inv_imbal, imbal_is_long) = if is_deriv {
        inv_imbalance_deriv(&position, inv_val)
    } else {
        inv_imbalance_spot(inv_base_val, inv_val)
    };

    // Calculate reservation price
    let reservation_price = reservation_price(
        mid_price,
        inv_imbal,
        varience,
        state.risk_aversion,
        state.manual_offset_perct,
        imbal_is_long,
    );

    // Calculate the new head prices
    let (new_buy_head, new_sell_head) = new_head_prices(varience, reservation_price, state.risk_aversion);

    // Split open orders
    let (open_buys, open_sells) = split_open_orders(open_orders);

    // Ensure that the heads have changed enough that we are willing to make an action
    if head_chg_gt_tol(&open_buys, new_buy_head, state.head_chg_tol_bp) && head_chg_gt_tol(&open_sells, new_sell_head, state.head_chg_tol_bp) {
        // Get new tails
        let (new_buy_tail, new_sell_tail) = new_tail_prices(new_buy_head, new_sell_head, state.tail_dist_to_head_bp);

        // Get information for buy order creation/cancellation
        let (mut buy_hashes_to_cancel, buy_orders_to_keep, buy_orders_remaining_val, buy_append_to_new_head) =
            orders_to_cancel(open_buys, new_buy_head, new_buy_tail, true);

        // Get information for sell order creation/cancellation
        let (mut sell_hashes_to_cancel, sell_orders_to_keep, sell_orders_remaining_val, sell_append_to_new_head) =
            orders_to_cancel(open_sells, new_sell_head, new_sell_tail, false);

        // Get new buy orders
        let new_buy_orders = create_orders(
            new_buy_head,
            new_buy_tail,
            inv_val,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            position.clone(),
            buy_append_to_new_head,
            &mut buy_hashes_to_cancel,
            is_deriv,
            true,
            &state,
        );

        // Get new sell orders
        let mut new_sell_orders = create_orders(
            new_sell_head,
            new_sell_tail,
            inv_val,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            position,
            sell_append_to_new_head,
            &mut sell_hashes_to_cancel,
            is_deriv,
            false,
            &state,
        );

        let mut hashes_to_cancel = buy_hashes_to_cancel;
        hashes_to_cancel.append(&mut sell_hashes_to_cancel);

        let mut orders_to_open = new_buy_orders;
        orders_to_open.append(&mut new_sell_orders);

        Ok(WrappedGetActionResponse {
            hashes_to_cancel,
            orders_to_open,
        })
    } else {
        Ok(WrappedGetActionResponse {
            hashes_to_cancel: Vec::new(),
            orders_to_open: Vec::new(),
        })
    }
}

/// Uses the new head/tail to determine if there are any orders that need to be cancelled.
/// # Arguments
/// * `open_orders` - A list of open orders from the last block
/// * `new_head` - The new head (closest to the reservation price)
/// * `new_tail` - The new tail (farthest from the reservation price)
/// * `is_buy` - True if all open_orders are buy. False if they are all sell
/// # Returns
/// * `hashes_to_cancel` - A list of order hashes that we would like to cancel
/// * `orders_to_keep` - A list of open orders that we are going to keep on the book
/// * `orders_remaining_val` - An aggregation of the total notional value of orders_to_keep
/// * `append_to_new_head` - An indication of whether we should append new orders to the new head
/// or to the back of the orders_to_keep block
pub fn orders_to_cancel(
    open_orders: Vec<WrappedOpenOrder>,
    new_head: Decimal,
    new_tail: Decimal,
    is_buy: bool,
) -> (Vec<String>, Vec<WrappedOpenOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut hashes_to_cancel: Vec<String> = Vec::new();
    // If there are any open orders, we need to check them to see if we should cancel
    if open_orders.len() > 0 {
        // Use the new tail/head to filter out the orders to cancel
        let orders_to_keep: Vec<WrappedOpenOrder> = open_orders
            .into_iter()
            .filter(|o| {
                let keep_if_buy = o.price <= new_head && o.price >= new_tail;
                let keep_if_sell = o.price >= new_head && o.price <= new_tail;
                let keep = (keep_if_buy && is_buy) || (keep_if_sell && !is_buy);
                if keep {
                    orders_remaining_val = orders_remaining_val + (o.price * o.qty);
                } else {
                    hashes_to_cancel.push(o.order_hash.clone());
                }
                keep
            })
            .collect();
        // Determine if we need to append to new orders to the new head or if we need to
        // append to the end of the block of orders we will be keeping
        let append_to_new_head = sub_abs(new_head, orders_to_keep.first().unwrap().price) > sub_abs(orders_to_keep.last().unwrap().price, new_tail);
        (hashes_to_cancel, orders_to_keep, orders_remaining_val, append_to_new_head)
    } else {
        (hashes_to_cancel, Vec::new(), orders_remaining_val, true)
    }
}

fn create_orders(
    new_head: Decimal,
    new_tail: Decimal,
    inv_val: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal,
    position: Option<WrappedPosition>,
    append_to_new_head: bool,
    hashes_to_cancel: &mut Vec<String>,
    is_deriv: bool,
    is_buy: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - orders_remaining_val;
    if is_deriv {
        let position_qty = match position {
            Some(position) => {
                if !position.is_long {
                    Decimal::zero()
                } else {
                    position.quantity
                }
            }
            None => Decimal::zero(),
        };
        if orders_to_keep.len() == 0 {
            let (new_orders, _) = base_deriv(
                new_head,
                new_tail,
                alloc_val_for_new_orders,
                orders_to_keep,
                position_qty,
                true,
                is_buy,
                &state,
            );
            new_orders
        } else if append_to_new_head {
            let (new_orders, mut additional_hashes_to_cancel) =
                tail_to_head_deriv(new_head, alloc_val_for_new_orders, orders_to_keep, position_qty, is_buy, &state);
            hashes_to_cancel.append(&mut additional_hashes_to_cancel);
            new_orders
        } else {
            let (new_orders, mut additional_hashes_to_cancel) =
                head_to_tail_deriv(new_tail, alloc_val_for_new_orders, orders_to_keep, position_qty, is_buy, &state);
            hashes_to_cancel.append(&mut additional_hashes_to_cancel);
            new_orders
        }
    } else {
        create_new_orders_spot(
            new_head,
            new_tail,
            alloc_val_for_new_orders,
            orders_to_keep,
            append_to_new_head,
            is_buy,
            &state,
        )
    }
}

fn reservation_price(
    mid_price: Decimal,
    inv_imbal: Decimal,
    varience: Decimal,
    risk_aversion: Decimal,
    manual_offset_perct: Decimal,
    imbal_is_long: bool,
) -> Decimal {
    if inv_imbal == Decimal::zero() {
        mid_price
    } else {
        if imbal_is_long {
            (mid_price - (inv_imbal * risk_aversion * varience)) * (Decimal::one() - manual_offset_perct)
        } else {
            (mid_price + (inv_imbal * risk_aversion * varience)) * (Decimal::one() + manual_offset_perct)
        }
    }
}

fn new_head_prices(varience: Decimal, reservation_price: Decimal, risk_aversion: Decimal) -> (Decimal, Decimal) {
    let dist_from_reservation_price = div_dec(varience * risk_aversion, Decimal::from_str("2").unwrap());
    (
        reservation_price - dist_from_reservation_price,
        reservation_price + dist_from_reservation_price,
    )
}

fn head_chg_gt_tol(open_orders: &Vec<WrappedOpenOrder>, new_head: Decimal, head_chg_tol_bp: Decimal) -> bool {
    if open_orders.len() == 0 {
        true
    } else {
        let old_head = open_orders.first().unwrap().price;
        div_dec(sub_abs(old_head, new_head), old_head) * Decimal::from_str("10000").unwrap() > head_chg_tol_bp
    }
}

fn new_tail_prices(new_buy_head: Decimal, new_sell_head: Decimal, tail_dist_to_head_bp: Decimal) -> (Decimal, Decimal) {
    (
        new_buy_head * (Decimal::one() - (tail_dist_to_head_bp * Decimal::from_str("10000").unwrap())),
        new_sell_head * (Decimal::one() + (tail_dist_to_head_bp * Decimal::from_str("10000").unwrap())),
    )
}

fn split_open_orders(open_orders: Vec<WrappedOpenOrder>) -> (Vec<WrappedOpenOrder>, Vec<WrappedOpenOrder>) {
    let mut buy_orders: Vec<WrappedOpenOrder> = open_orders.clone().into_iter().filter(|o| o.is_buy).collect();
    let mut sell_orders: Vec<WrappedOpenOrder> = open_orders.into_iter().filter(|o| !o.is_buy).collect();

    // Sort both so the head is at index 0
    buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    (buy_orders, sell_orders)
}

#[cfg(test)]
mod tests {
    use super::split_open_orders;
    use crate::{contract::orders_to_cancel, msg::WrappedOpenOrder};
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;

    #[test]
    fn split_open_orders_test() {
        let mut open_orders: Vec<WrappedOpenOrder> = Vec::new();
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: true,
            qty: Decimal::zero(),
            price: Decimal::one(),
            is_reduce_only: false,
        };
        let mut buy = order.clone();
        let mut sell = order.clone();
        for _ in 0..25 {
            sell.is_buy = false;
            buy.price = buy.price + Decimal::one();
            sell.price = sell.price + Decimal::one();
            open_orders.push(buy.clone());
            open_orders.push(sell.clone());
        }
        let (buy_orders, sell_orders) = split_open_orders(open_orders);
        for i in 1..25 {
            assert!(buy_orders[i].price < buy_orders[i - 1].price); // These should be decreasing
            assert!(sell_orders[i].price > sell_orders[i - 1].price); // These should be increasing
        }
    }

    #[test]
    fn orders_to_cancel_for_buy_test() {
        let mut open_buy_orders: Vec<WrappedOpenOrder> = Vec::new();
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: true,
            qty: Decimal::one(),
            price: Decimal::zero(),
            is_reduce_only: false,
        };

        let old_tail_price = 10;
        let order_density = 10;

        let new_buy_head_a = Decimal::from_str(&(old_tail_price + order_density + 1).to_string()).unwrap();
        let new_buy_tail_a = Decimal::from_str(&(old_tail_price + 1).to_string()).unwrap();
        let mut buy_orders_remaining_val_a = Decimal::zero();

        let new_buy_head_b = Decimal::from_str(&(old_tail_price + order_density - 1).to_string()).unwrap();
        let new_buy_tail_b = Decimal::from_str(&(old_tail_price + 1).to_string()).unwrap();
        let mut buy_orders_remaining_val_b = Decimal::zero();

        let mut buy = order.clone();
        for i in (old_tail_price..(old_tail_price + order_density + 1)).into_iter().rev() {
            let price = Decimal::from_str(&i.to_string()).unwrap();
            buy.price = price;
            buy.is_reduce_only = i % 2 == 0;
            if price <= new_buy_head_a && price >= new_buy_tail_a {
                buy_orders_remaining_val_a = buy_orders_remaining_val_a + price;
            }
            if price <= new_buy_head_b && price >= new_buy_tail_b {
                buy_orders_remaining_val_b = buy_orders_remaining_val_b + price;
            }
            open_buy_orders.push(buy.clone());
        }

        // Check case where we need to cancel orders by the tail because the new head > old head
        let (buy_hashes_to_cancel, buy_orders_to_keep, buy_orders_remaining_val, buy_append_new_to_head) =
            orders_to_cancel(open_buy_orders.clone(), new_buy_head_a, new_buy_tail_a, true);
        assert!(buy_append_new_to_head);
        assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_a);
        assert_eq!(open_buy_orders.len() - buy_orders_to_keep.len(), buy_hashes_to_cancel.len());

        // Check case where we need to cancel orders by the head because the new head < old head
        let (buy_hashes_to_cancel, buy_orders_to_keep, buy_orders_remaining_val, buy_append_new_to_head) =
            orders_to_cancel(open_buy_orders.clone(), new_buy_head_b, new_buy_tail_b, true);
        assert!(!buy_append_new_to_head);
        assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_b);
        assert_eq!(open_buy_orders.len() - buy_orders_to_keep.len(), buy_hashes_to_cancel.len());

        // Check case where there were no open orders at all
        let (buy_hashes_to_cancel, _, _, buy_append_new_to_head) = orders_to_cancel(Vec::new(), new_buy_head_a, new_buy_tail_a, true);
        assert!(buy_append_new_to_head);
        assert_eq!(0, buy_hashes_to_cancel.len());
    }
    #[test]
    fn orders_to_cancel_for_sell_test() {
        let mut open_sell_orders: Vec<WrappedOpenOrder> = Vec::new();
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: false,
            qty: Decimal::one(),
            price: Decimal::zero(),
            is_reduce_only: false,
        };

        let old_head_price = 10;
        let order_density = 10;

        let new_sell_head_a = Decimal::from_str(&(old_head_price + 1).to_string()).unwrap();
        let new_sell_tail_a = Decimal::from_str(&(old_head_price + order_density + 1).to_string()).unwrap();
        let mut sell_orders_remaining_val_a = Decimal::zero();

        let new_sell_head_b = Decimal::from_str(&(old_head_price - 1).to_string()).unwrap();
        let new_sell_tail_b = Decimal::from_str(&(old_head_price + order_density - 1).to_string()).unwrap();
        let mut sell_orders_remaining_val_b = Decimal::zero();

        let mut sell = order.clone();
        for i in old_head_price..(old_head_price + order_density + 1) {
            let price = Decimal::from_str(&i.to_string()).unwrap();
            sell.price = price;
            sell.is_reduce_only = i % 2 == 0;
            if price >= new_sell_head_a && price <= new_sell_tail_a {
                sell_orders_remaining_val_a = sell_orders_remaining_val_a + price;
            }
            if price >= new_sell_head_b && price <= new_sell_tail_b {
                sell_orders_remaining_val_b = sell_orders_remaining_val_b + price;
            }
            open_sell_orders.push(sell.clone());
        }

        // Check case where we need to cancel orders by the tail because the new head > old head
        let (sell_hashes_to_cancel, sell_orders_to_keep, sell_orders_remaining_val, sell_append_new_to_head) =
            orders_to_cancel(open_sell_orders.clone(), new_sell_head_a, new_sell_tail_a, false);
        assert!(!sell_append_new_to_head);
        assert_eq!(sell_orders_remaining_val, sell_orders_remaining_val_a);
        assert_eq!(open_sell_orders.len() - sell_orders_to_keep.len(), sell_hashes_to_cancel.len());

        // Check case where we need to cancel orders by the tail because the new head > old head
        let (sell_hashes_to_cancel, sell_orders_to_keep, sell_orders_remaining_val, sell_append_new_to_head) =
            orders_to_cancel(open_sell_orders.clone(), new_sell_head_b, new_sell_tail_b, false);
        assert!(sell_append_new_to_head);
        assert_eq!(sell_orders_remaining_val, sell_orders_remaining_val_b);
        assert_eq!(open_sell_orders.len() - sell_orders_to_keep.len(), sell_hashes_to_cancel.len());

        // Check case where there were no open orders at all
        let (sell_hashes_to_cancel, _, _, sell_append_new_to_head) = orders_to_cancel(Vec::new(), new_sell_head_a, new_sell_tail_a, false);
        assert!(sell_append_new_to_head);
        assert_eq!(0, sell_hashes_to_cancel.len());
    }
}
