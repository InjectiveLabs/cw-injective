use std::str::FromStr;

use crate::derivative::{create_new_orders_deriv, inv_imbalance_deriv};
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, OpenOrder, Position, QueryMsg, WrappedGetActionResponse, WrappedOpenOrder, WrappedOrderResponse, WrappedPosition,
};
use crate::risk_management::{check_tail_dist, get_alloc_bal_new_orders, safe_varience};
use crate::spot::{create_new_orders_spot, inv_imbalance_spot};
use crate::state::{config, config_read, State};
use crate::utils::{bp_to_perct, div_dec, sub_abs, wrap};
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
        manual_offset_perct: Decimal::from_str(&msg.manual_offset_perct.clone()).unwrap(),
        min_tail_dist_perct: bp_to_perct(Decimal::from_str(&msg.min_tail_dist_bp.clone()).unwrap()),
        tail_dist_from_mid_perct: bp_to_perct(Decimal::from_str(&msg.tail_dist_from_mid_bp.clone()).unwrap()),
        head_chg_tol_perct: bp_to_perct(Decimal::from_str(&msg.head_chg_tol_bp.clone()).unwrap()),
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
    let varience = safe_varience(std_dev);

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
    if head_chg_is_gt_tol(&open_buys, new_buy_head, state.head_chg_tol_perct)
        && head_chg_is_gt_tol(&open_sells, new_sell_head, state.head_chg_tol_perct)
    {
        // Get new tails
        let (new_buy_tail, new_sell_tail) = new_tail_prices(
            new_buy_head,
            new_sell_head,
            mid_price,
            state.tail_dist_from_mid_perct,
            state.min_tail_dist_perct,
        );

        // Cancel all open buy/sell from the preceeding block
        let buy_hashes_to_cancel = open_buys.iter().map(|o| o.order_hash.clone()).collect();
        let sell_hashes_to_cancel = open_sells.iter().map(|o| o.order_hash.clone()).collect();

        // Get new buy/sell orders
        let buy_orders_to_open = create_orders(new_buy_head, new_buy_tail, inv_val, position.clone(), is_deriv, true, &state);
        let sell_orders_to_open = create_orders(new_sell_head, new_sell_tail, inv_val, position, is_deriv, false, &state);

        Ok(WrappedGetActionResponse {
            buy_hashes_to_cancel,
            buy_orders_to_open,
            sell_hashes_to_cancel,
            sell_orders_to_open,
        })
    } else {
        Ok(WrappedGetActionResponse {
            buy_hashes_to_cancel: Vec::new(),
            buy_orders_to_open: Vec::new(),
            sell_hashes_to_cancel: Vec::new(),
            sell_orders_to_open: Vec::new(),
        })
    }
}

fn create_orders(
    new_head: Decimal,
    new_tail: Decimal,
    inv_val: Decimal,
    position: Option<WrappedPosition>,
    is_deriv: bool,
    is_buy: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    if is_deriv {
        let (position_qty, position_margin) = match position {
            Some(position) => {
                if position.is_long == is_buy {
                    (Decimal::zero(), position.margin)
                } else {
                    (position.quantity, Decimal::zero())
                }
            }
            None => (Decimal::zero(), Decimal::zero()),
        };
        let alloc_val_for_new_orders = get_alloc_bal_new_orders(inv_val, position_margin, state.active_capital_perct);
        create_new_orders_deriv(new_head, new_tail, alloc_val_for_new_orders, position_qty, is_buy, &state).0
    } else {
        let alloc_val_for_new_orders = get_alloc_bal_new_orders(inv_val, Decimal::zero(), state.active_capital_perct);
        create_new_orders_spot(new_head, new_tail, alloc_val_for_new_orders, is_buy, &state)
    }
}

/// Uses the inventory imbalance to
/// # Arguments
/// * `varience` - The square of the the standard deviation that gets passed in as a param to get action
/// * `reservation_price` - The a price that is shifted from the mid price depending on the inventory imbalance
/// * `risk_aversion` - Risk aversion param configured upon intialization
/// # Returns
/// * `buy_head` - The new buy head
/// * `sell_head` - The new sell head
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

/// Uses the reservation price and variation to calculate where the buy/sell heads should be. Both buy and
/// sell heads will be equi-distant from the reservation price.
/// # Arguments
/// * `varience` - The square of the the standard deviation that gets passed in as a param to get action
/// * `reservation_price` - The a price that is shifted from the mid price depending on the inventory imbalance
/// * `risk_aversion` - Risk aversion param configured upon intialization
/// # Returns
/// * `buy_head` - The new buy head
/// * `sell_head` - The new sell head
fn new_head_prices(varience: Decimal, reservation_price: Decimal, risk_aversion: Decimal) -> (Decimal, Decimal) {
    let dist_from_reservation_price = div_dec(varience * risk_aversion, Decimal::from_str("2").unwrap());
    (
        reservation_price - dist_from_reservation_price,
        reservation_price + dist_from_reservation_price,
    )
}

/// Determines whether the new head for the next block will be different enough than that of the previous to warrent
/// order cancellation and the creation of new orders.
/// # Arguments
/// * `open_orders` - The buy or sell side orders from the previous block
/// * `new_head` - The new proposed buy or sell head
/// * `head_chg_tol_perct` - Our tolerance to change in the head price
/// # Returns
/// * `should_change` - Whether we should cancel and place new orders at the new head
fn head_chg_is_gt_tol(open_orders: &Vec<WrappedOpenOrder>, new_head: Decimal, head_chg_tol_perct: Decimal) -> bool {
    if open_orders.len() == 0 {
        true
    } else {
        let old_head = open_orders.first().unwrap().price;
        div_dec(sub_abs(old_head, new_head), old_head) > head_chg_tol_perct
    }
}

/// Calculates the correct tail prices for buy and sell sides based off of the mid price. If either of
/// the distances between the head and tail fall below the minimum spread defined at initialization, risk
/// manager returns a tail that meets the minimum spread constraint.
/// # Arguments
/// * `buy_head` - The new buy head
/// * `sell_head` - The new sell head
/// * `mid_price` - The mid price (or oracle price passed in with the query)
/// * `tail_dist_from_mid_perct` - The distance from the mid price, in either direction, that we want tails to be located
/// * `min_tail_dist_perct` - The min distance that can exist between any head and tail
/// # Returns
/// * `buy_tail` - The new buy tail
/// * `sell_tail` - The new sell tail
pub fn new_tail_prices(
    buy_head: Decimal,
    sell_head: Decimal,
    mid_price: Decimal,
    tail_dist_from_mid_perct: Decimal,
    min_tail_dist_perct: Decimal,
) -> (Decimal, Decimal) {
    let proposed_buy_tail = mid_price * (Decimal::one() - tail_dist_from_mid_perct);
    let proposed_sell_tail = mid_price * (Decimal::one() + tail_dist_from_mid_perct);
    check_tail_dist(buy_head, sell_head, proposed_buy_tail, proposed_sell_tail, min_tail_dist_perct)
}

/// Splits the vec of orders to buyside and sellside orders. Sorts them so that the head from the previous block is at index == 0. Buyside
/// orders are sorted desc. Sellside are sorted asc.
/// # Arguments
/// * `open_orders` - The open orders from both sides that were on the book as of the preceeding block
/// # Returns
/// * `buy_orders` - The sorted buyside orders
/// * `sell_orders` - The sorted sellside orders
fn split_open_orders(open_orders: Vec<WrappedOpenOrder>) -> (Vec<WrappedOpenOrder>, Vec<WrappedOpenOrder>) {
    let mut buy_orders: Vec<WrappedOpenOrder> = Vec::new();
    let mut sell_orders: Vec<WrappedOpenOrder> = open_orders
        .into_iter()
        .filter(|o| {
            if o.is_buy {
                buy_orders.push(o.clone());
            }
            !o.is_buy
        })
        .collect();

    // Sort both so the head is at index 0
    buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    (buy_orders, sell_orders)
}

#[cfg(test)]
mod tests {
    use super::{head_chg_is_gt_tol, new_tail_prices, split_open_orders};
    use crate::msg::WrappedOpenOrder;
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;
    #[test]
    fn head_chg_is_gt_tol_test() {
        let mut open_orders: Vec<WrappedOpenOrder> = Vec::new();
        let new_head = Decimal::from_str("1").unwrap();
        let head_chg_tol_perct = Decimal::from_str("0.01").unwrap();
        let should_change = head_chg_is_gt_tol(&open_orders, new_head, head_chg_tol_perct);
        assert!(should_change);

        open_orders.push(WrappedOpenOrder {
            is_buy: true,
            order_hash: String::from(""),
            qty: Decimal::zero(),
            price: Decimal::from_str("1").unwrap(),
            is_reduce_only: false,
        });

        let should_change = head_chg_is_gt_tol(&open_orders, new_head, head_chg_tol_perct);
        assert!(!should_change);

        open_orders.pop();
        open_orders.push(WrappedOpenOrder {
            is_buy: true,
            order_hash: String::from(""),
            qty: Decimal::zero(),
            price: Decimal::from_str("1.011").unwrap(),
            is_reduce_only: false,
        });

        let should_change = head_chg_is_gt_tol(&open_orders, new_head, head_chg_tol_perct);
        assert!(should_change);
    }

    #[test]
    fn new_tail_prices_test() {
        let buy_head = Decimal::from_str("3999").unwrap();
        let mid_price = Decimal::from_str("4000").unwrap();
        let sell_head = Decimal::from_str("4001").unwrap();
        let tail_dist_from_mid_perct = Decimal::from_str("0.05").unwrap();
        let min_tail_dist_perct = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = new_tail_prices(buy_head, sell_head, mid_price, tail_dist_from_mid_perct, min_tail_dist_perct);
        assert_eq!(buy_tail, mid_price * (Decimal::one() - tail_dist_from_mid_perct));
        assert_eq!(sell_tail, mid_price * (Decimal::one() + tail_dist_from_mid_perct));

        let tail_dist_from_mid_perct = Decimal::from_str("0.001").unwrap();
        let min_tail_dist_perct = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = new_tail_prices(buy_head, sell_head, mid_price, tail_dist_from_mid_perct, min_tail_dist_perct);
        assert_eq!(buy_tail, buy_head * (Decimal::one() - min_tail_dist_perct));
        assert_eq!(sell_tail, sell_head * (Decimal::one() + min_tail_dist_perct));
    }

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
}
