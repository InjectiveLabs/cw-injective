use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{
    wrap, ExecuteMsg, InstantiateMsg, QueryMsg, WrappedGetActionResponse, WrappedOrderResponse,
    WrappedPosition, div_dec,
};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, Decimal256 as Decimal,
    StdError, StdResult, Uint256,
};
use injective_bindings::{
    create_subaccount_transfer_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper,
    SubaccountDepositResponse,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        market_id: msg.market_id.to_string(),
        manager: info.sender.clone().into(),
        sub_account: msg.sub_account.clone(),
        fee_recipient: msg.fee_recipient.clone(),
        risk_aversion: msg.risk_aversion.clone(),
        price_distribution_rate: msg.price_distribution_rate.clone(),
        slices_per_spread_bp: msg.slices_per_spread_bp.clone(),
        ratio_active_capital: msg.ratio_active_capital.clone(),
        leverage: msg.leverage.clone(),
        decimal_shift: msg.decimal_shift.clone(),
    };

    config(deps.storage).save(&state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("market_id", msg.market_id)
        .add_attribute("sub_account", msg.sub_account)
        .add_attribute("fee_recipient", msg.fee_recipient)
        .add_attribute("risk_aversion", msg.risk_aversion)
        .add_attribute("price_distribution_rate", msg.price_distribution_rate)
        .add_attribute("slices_per_spread_bp", msg.slices_per_spread_bp)
        .add_attribute("ratio_active_capital", msg.ratio_active_capital)
        .add_attribute("leverage", msg.leverage.to_string())
        .add_attribute("decimal_shift", msg.decimal_shift.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Subscribe {
            subaccount_id,
            amount,
        } => subscribe(deps, env, info.sender, subaccount_id, amount),
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
    let res: SubaccountDepositResponse =
        querier.query_subaccount_deposit(subaccount_id.clone(), amount.denom.clone().into())?;

    // just log the available balance for now
    _deps
        .api
        .debug(res.deposits.available_balance.to_string().as_str());

    let msg = create_subaccount_transfer_msg(sender, subaccount_id.into(), contract.into(), amount);

    let res = Response::new().add_message(msg);
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        QueryMsg::GetAction {
            position,
            total_notional_balance,
            standard_deviation,
            mid_price,
        } => to_binary(&query_action(
            deps,
            position.wrap(deps).unwrap(),
            wrap(&total_notional_balance, deps),
            wrap(&standard_deviation, deps),
            wrap(&mid_price, deps),
        )?),
    }
}

fn query_action(
    deps: Deps<InjectiveQueryWrapper>,
    position: WrappedPosition,
    total_notional_balance: Decimal,
    std_dev: Decimal,
    mid_price: Decimal,
) -> StdResult<WrappedGetActionResponse> {
    let state = config_read(deps.storage).load().unwrap();
    let leverage = Decimal::from_str(&state.leverage).unwrap();
    let risk_aversion = Decimal::from_str(&state.risk_aversion).unwrap();
    let ratio_active_capital = Decimal::from_str(&state.ratio_active_capital).unwrap();
    let slices_per_spread_bp = Decimal::from_str(&state.slices_per_spread_bp).unwrap();
    let price_distribution_rate = Decimal::from_str(&state.price_distribution_rate).unwrap();
    let decimal_shift = Decimal::from_str(&state.decimal_shift).unwrap();
    let varience = std_dev * std_dev;
    let q = calculate_inventory_dist_from_target(&position, total_notional_balance);
    let reservation_price = calc_reservation_price(mid_price, q, risk_aversion, varience,  position.is_long && position.quantity > Decimal::from_str("0").unwrap());
    let reservation_spread = calc_spread_res(varience, risk_aversion);
    let orders_to_open = create_all_orders(
        &state,
        decimal_shift,
        ratio_active_capital,
        slices_per_spread_bp,
        price_distribution_rate,
        reservation_spread,
        reservation_price,
        risk_aversion,
        varience,
        total_notional_balance,
        &position,
        mid_price,
        leverage,
    );
    Ok(WrappedGetActionResponse { orders_to_open })
}

fn create_all_orders(
    state: &State,
    decimal_shift: Decimal,
    ratio_active_capital: Decimal,
    slices_per_spread_bp: Decimal,
    price_distribution_rate: Decimal,
    reservation_spread: Decimal,
    reservation_price: Decimal,
    risk_aversion: Decimal,
    varience: Decimal,
    total_notional_balance: Decimal,
    position: &WrappedPosition,
    mid_price: Decimal,
    leverage: Decimal,
) -> Vec<WrappedOrderResponse> {
    let alloc_notional_balance =
        (total_notional_balance * ratio_active_capital) / Uint256::from_str("2").unwrap();
    let mid_spread = calc_spread_mid(varience, risk_aversion);
    let min_bid = mid_price - mid_spread;
    let mut buy_orders = create_orders(
        state,
        decimal_shift,
        slices_per_spread_bp,
        price_distribution_rate,
        true,
        reservation_spread,
        reservation_price,
        alloc_notional_balance,
        position,
        min_bid,
        leverage,
    );
    let max_ask = mid_price + mid_spread;
    let mut sell_orders = create_orders(
        state,
        decimal_shift,
        slices_per_spread_bp,
        price_distribution_rate,
        false,
        reservation_spread,
        reservation_price,
        alloc_notional_balance,
        position,
        max_ask,
        leverage,
    );
    buy_orders.append(&mut sell_orders);
    buy_orders
}

fn create_orders(
    state: &State,
    decimal_shift: Decimal,
    slices_per_spread_bp: Decimal,
    price_distribution_rate: Decimal,
    are_bids: bool,
    spread: Decimal,
    reservation_price: Decimal,
    alloc_notional_balance: Decimal,
    position: &WrappedPosition,
    max_price: Decimal,
    leverage: Decimal,
) -> Vec<WrappedOrderResponse> {
    let (mut free_balance_remaining, mut reduce_only_qty_remaining, mut margin_remaining) =
        if are_bids == position.is_long {
            (
                alloc_notional_balance - position.margin,
                Decimal::from_str("0").unwrap(),
                Decimal::from_str("0").unwrap(),
            )
        } else {
            (alloc_notional_balance, position.quantity, position.margin)
        };

    let optimal_price = if are_bids {
        reservation_price - spread
    } else {
        reservation_price + spread
    };

    let mut orders: Vec<WrappedOrderResponse> = Vec::new();
    let bp = if max_price > optimal_price {
        div_dec(max_price - optimal_price, optimal_price) * Decimal::from_str("10000").unwrap()
    } else {
        div_dec(optimal_price - max_price, optimal_price) * Decimal::from_str("10000").unwrap()
    };
    let num_buy_orders = format!("{:.0}", div_dec(bp , slices_per_spread_bp).to_string()).parse::<i32>().unwrap();
    if num_buy_orders > 0 {
        let price_interval =
            div_dec(optimal_price - max_price, Decimal::from_str(&num_buy_orders.to_string()).unwrap());
        for i in 0..num_buy_orders {
            let current_price = max_price + (price_interval * Decimal::from_str(&(i + 1).to_string()).unwrap());
            let (balance_for_price, balance_for_next_price) = if i != num_buy_orders - 1 {
                let balance_for_next = div_dec(free_balance_remaining, price_distribution_rate);
                (free_balance_remaining - balance_for_next, balance_for_next)
            } else {
                (free_balance_remaining, Decimal::from_str("0").unwrap())
            };
            if margin_remaining > balance_for_next_price {
                let reduce_only_balance_for_price = margin_remaining - balance_for_next_price;
                let qty = div_dec(reduce_only_balance_for_price , current_price) * leverage;
                let reduce_order = WrappedOrderResponse::new(
                    state,
                    decimal_shift,
                    current_price,
                    qty,
                    are_bids,
                    true,
                    leverage,
                );
                reduce_only_qty_remaining = reduce_only_qty_remaining - qty;
                let remaining_balance_for_price = balance_for_price - reduce_only_balance_for_price;
                let qty = div_dec(remaining_balance_for_price , current_price) * leverage;
                if qty > Decimal::from_str("0").unwrap() {
                    let order = WrappedOrderResponse::new(
                        state,
                        decimal_shift,
                        current_price,
                        qty,
                        are_bids,
                        false,
                        leverage,
                    );
                    orders.push(order);
                }
                orders.push(reduce_order);
                margin_remaining = margin_remaining - reduce_only_balance_for_price;
            } else {
                let qty = div_dec(balance_for_price , current_price) * leverage;
                let order = WrappedOrderResponse::new(
                    state,
                    decimal_shift,
                    current_price,
                    qty,
                    are_bids,
                    false,
                    leverage,
                );
                orders.push(order);
            }
            free_balance_remaining = free_balance_remaining - balance_for_price;
        }
        if !are_bids {
            orders.reverse();
        }
    }
    orders
}

fn calculate_inventory_dist_from_target(
    position: &WrappedPosition,
    total_notional_balance: Decimal,
) -> Decimal {
    let notional_position_value = position.margin;
    div_dec(notional_position_value, total_notional_balance)
}

fn calc_reservation_price(s: Decimal, q: Decimal, r: Decimal, varience: Decimal, should_sub: bool) -> Decimal {
    if should_sub {
        s + (q * r * varience)
    } else {
        s - (q * r * varience)
    }
}

fn calc_spread_res(varience: Decimal, risk_aversion: Decimal) -> Decimal {
    div_dec(varience * risk_aversion, Decimal::from_str("2").unwrap())
}

fn calc_spread_mid(varience: Decimal, risk_aversion: Decimal) -> Decimal {
    div_dec(risk_aversion * varience * Decimal::from_str("2").unwrap(), Decimal::from_str("2").unwrap())
}

// #[cfg(test)]
// mod tests {
//     use std::marker::PhantomData;

//     use super::*;
//     use cosmwasm_std::testing::{MockStorage, MockApi, MockQuerier, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary, OwnedDeps, QuerierWrapper};
//     use injective_bindings::{InjectiveRoute, InjectiveQuery};

//     #[test]
//     fn reservation_price_test() {
//         let s = Decimal::from_str("4000").unwrap();
//         let q = Decimal::from_str("0.2").unwrap();
//         let r = Decimal::from_str("0.5").unwrap();
//         let std_dev = Decimal::from_str("1").unwrap();
//         let reservation_price = calc_reservation_price(s, q, r, std_dev * std_dev);
//         println!("{}", reservation_price);
//         assert_eq!(Decimal::from_str("3999.90").unwrap(), reservation_price);
//     }

//     #[test]
//     fn inventory_dist_from_target_test() {
//         // first, no position taken
//         let position = WrappedPosition {
//             is_long: true,
//             quantity: Decimal::from_str("0").unwrap(),
//             avg_price: Decimal::from_str("0").unwrap(),
//             margin: Decimal::from_str("0").unwrap(),
//             cum_funding_entry: Decimal::from_str("0").unwrap(),
//         };

//         let q =
//             calculate_inventory_dist_from_target(&position, Decimal::from_str("100000").unwrap());

//         assert_eq!(q, Decimal::from_str("0.0").unwrap());

//         // first, long taken
//         let position = WrappedPosition {
//             is_long: true,
//             quantity: Decimal::from_str("1").unwrap(),
//             avg_price: Decimal::from_str("4000").unwrap(),
//             margin: Decimal::from_str("4000").unwrap(),
//             cum_funding_entry: Decimal::from_str("0").unwrap(),
//         };

//         let q =
//             calculate_inventory_dist_from_target(&position, Decimal::from_str("100000").unwrap());

//         assert_eq!(q, Decimal::from_str("-0.04").unwrap());

//         // first, short taken
//         let position = WrappedPosition {
//             is_long: false,
//             quantity: Decimal::from_str("1").unwrap(),
//             avg_price: Decimal::from_str("4000").unwrap(),
//             margin: Decimal::from_str("4000").unwrap(),
//             cum_funding_entry: Decimal::from_str("0").unwrap(),
//         };

//         let q =
//             calculate_inventory_dist_from_target(&position, Decimal::from_str("100000").unwrap());

//         assert_eq!(q, Decimal::from_str("0.04").unwrap());
//     }

//     #[test]
//     fn create_orders_test() {
//         // Create vars
//         let market_id = String::from("some market");
//         let sub_account = String::from("some account");
//         let fee_recipient = String::from("some recipient");
//         let manager = String::from("some manager");
//         let risk_aversion = Decimal::from_str("0.5").unwrap();
//         let price_distribution_rate = Decimal::from_str("2.5").unwrap();
//         let leverage = Decimal::from_str("2").unwrap();
//         let decimal_shift = Decimal::from_str("1000000").unwrap();
//         let slices_per_spread_bp = Decimal::from_str("0.2").unwrap();
//         let ratio_active_capital = Decimal::from_str("0.2").unwrap();

//         // Get state
//         let query =  InjectiveQueryWrapper {
//              route: InjectiveRoute::Exchange,
//              query_data: InjectiveQuery::SubaccountDeposit{ subaccount_id: String::from(""), denom: String::from("") },
//         };
//         let querier: QuerierWrapper<'_, injective_bindings::InjectiveQueryWrapper > = QuerierWrapper::new(query);
//         let deps: DepsMut<'_, InjectiveQueryWrapper> = DepsMut {
//             storage: &mut MockStorage::default(),
//             api: &MockApi::default(),
//             querier: querier,
//         };
//         let msg = InstantiateMsg {
//             market_id,
//             sub_account,
//             fee_recipient,
//             risk_aversion: risk_aversion.to_string(),
//             price_distribution_rate: price_distribution_rate.to_string(),
//             leverage: leverage.to_string(),
//             decimal_shift: decimal_shift.to_string(),
//             slices_per_spread_bp: slices_per_spread_bp.to_string(),
//             ratio_active_capital: ratio_active_capital.to_string(),
//             manager,
//         };
//         let info = mock_info("creator", &coins(1000, "earth"));
//         instantiate(deps, mock_env(), info.clone(), msg).unwrap();

//         let state = config_read(deps.storage).load().unwrap();

//         // Test sell side orders (No Position)
//         let quantity = Decimal::from_str("0").unwrap();
//         let avg_price = Decimal::from_str("0").unwrap();
//         let margin = quantity * avg_price / leverage;
//         let cum_funding_entry = Decimal::from_str("0").unwrap();
//         let position = WrappedPosition {
//             is_long: true,
//             quantity,
//             avg_price,
//             margin,
//             cum_funding_entry,
//         };
//         let spread = Decimal::from_i32(1).unwrap();
//         let reservation_price = Decimal::from_i64(4000).unwrap();
//         let alloc_notional_balance = Decimal::from_i64(10000).unwrap();
//         let max_price = Decimal::from_i64(4002).unwrap();
//         let orders = create_orders(
//             &state,
//             decimal_shift,
//             slices_per_spread_bp,
//             price_distribution_rate,
//             false,
//             spread,
//             reservation_price,
//             alloc_notional_balance,
//             &position,
//             max_price,
//             leverage,
//         );
//         orders.iter().for_each(|o| println!("{}", o));

//         if orders.len() > 0 {
//             sell_order_test_helper(
//                 orders,
//                 position,
//                 max_price,
//                 decimal_shift,
//                 reservation_price,
//                 alloc_notional_balance,
//                 leverage,
//                 spread,
//             );
//         }

//         // Test sell side orders (Short Position)
//         let quantity = Decimal::from_str("1").unwrap();
//         let avg_price = Decimal::from_str("4001").unwrap();
//         let margin = quantity * avg_price / leverage;
//         let cum_funding_entry = Decimal::from_str("0").unwrap();
//         let position = WrappedPosition {
//             is_long: false,
//             quantity,
//             avg_price,
//             margin,
//             cum_funding_entry,
//         };
//         let spread = Decimal::from_i32(2).unwrap();
//         let reservation_price = Decimal::from_i64(4000).unwrap();
//         let alloc_notional_balance = Decimal::from_i64(10000).unwrap();
//         let max_price = Decimal::from_i64(4004).unwrap();
//         let orders = create_orders(
//             &state,
//             decimal_shift,
//             slices_per_spread_bp,
//             price_distribution_rate,
//             false,
//             spread,
//             reservation_price,
//             alloc_notional_balance,
//             &position,
//             max_price,
//             leverage,
//         );
//         orders.iter().for_each(|o| println!("{}", o));

//         if orders.len() > 0 {
//             sell_order_test_helper(
//                 orders,
//                 position,
//                 max_price,
//                 decimal_shift,
//                 reservation_price,
//                 alloc_notional_balance,
//                 leverage,
//                 spread,
//             );
//         }

//         // Test sell side orders (Long Position)
//         let quantity = Decimal::from_str("1").unwrap();
//         let avg_price = Decimal::from_str("3999").unwrap();
//         let margin = quantity * avg_price / leverage;
//         let cum_funding_entry = Decimal::from_str("0").unwrap();
//         let position = WrappedPosition {
//             is_long: true,
//             quantity,
//             avg_price,
//             margin,
//             cum_funding_entry,
//         };
//         let spread = Decimal::from_i32(2).unwrap();
//         let reservation_price = Decimal::from_i64(4000).unwrap();
//         let alloc_notional_balance = Decimal::from_i64(10000).unwrap();
//         let max_price = Decimal::from_i64(4004).unwrap();
//         let orders = create_orders(
//             &state,
//             decimal_shift,
//             slices_per_spread_bp,
//             price_distribution_rate,
//             false,
//             spread,
//             reservation_price,
//             alloc_notional_balance,
//             &position,
//             max_price,
//             leverage,
//         );
//         orders.iter().for_each(|o| println!("{}", o));

//         if orders.len() > 0 {
//             sell_order_test_helper(
//                 orders,
//                 position,
//                 max_price,
//                 decimal_shift,
//                 reservation_price,
//                 alloc_notional_balance,
//                 leverage,
//                 spread,
//             );
//         }
//     }

//     fn sell_order_test_helper(
//         orders: Vec<WrappedOrderResponse>,
//         position: WrappedPosition,
//         max_price: Decimal,
//         decimal_shift: Decimal,
//         reservation_price: Decimal,
//         alloc_notional_balance: Decimal,
//         leverage: Decimal,
//         spread: Decimal,
//     ) {
//         assert!(
//             max_price * decimal_shift > Decimal::from_str(&orders[orders.len() - 1].price).unwrap()
//         );
//         let min_price = Decimal::from_str(&orders[0].price).unwrap();
//         assert!(min_price >= (reservation_price + spread) * decimal_shift);

//         // If there was a position taking there will be additional things to check
//         if !position.is_long && position.avg_price > Decimal::from_i32(0).unwrap() {
//             // Ensure that our notional balance of sell orders includes the balance of the short position taken
//             let notional_value: Vec<Decimal> = orders
//                 .iter()
//                 .map(|order| {
//                     Decimal::from_str(&order.price).unwrap()
//                         * Decimal::from_str(&order.quantity).unwrap()
//                 })
//                 .collect();
//             let notional_value = notional_value
//                 .into_iter()
//                 .fold(Decimal::from_i32(0).unwrap(), |acc, x| acc + x);
//             println!(
//                 "{} {}",
//                 (alloc_notional_balance - position.margin)
//                     * decimal_shift
//                     * Decimal::from_str("1.00001").unwrap(),
//                 (notional_value / leverage).round()
//             );
//             assert!(
//                 (alloc_notional_balance - position.margin)
//                     * decimal_shift
//                     * Decimal::from_str("1.00001").unwrap()
//                     > (notional_value / leverage).round()
//             );
//         } else if position.is_long && position.avg_price > Decimal::from_i32(0).unwrap() {
//             // Ensure that the total notional value of the orders is equal to the allocated notional balance + some tolerance
//             let notional_value: Vec<Decimal> = orders
//                 .iter()
//                 .map(|order| {
//                     Decimal::from_str(&order.price).unwrap()
//                         * Decimal::from_str(&order.quantity).unwrap()
//                 })
//                 .collect();
//             let notional_value = notional_value
//                 .into_iter()
//                 .fold(Decimal::from_i32(0).unwrap(), |acc, x| acc + x);
//             println!(
//                 "{} {}",
//                 alloc_notional_balance * decimal_shift * Decimal::from_str("1.00001").unwrap(),
//                 (notional_value / leverage).round()
//             );
//             assert!(
//                 alloc_notional_balance * decimal_shift * Decimal::from_str("1.00001").unwrap()
//                     > (notional_value / leverage).round()
//             );

//             // Ensure that the reduce only orders add up to the margin of the position
//             let reduce_only_val: Vec<WrappedOrderResponse> = orders
//                 .into_iter()
//                 .filter(|order| order.is_reduce_only)
//                 .collect();
//             let reduce_only_val: Vec<Decimal> = reduce_only_val
//                 .iter()
//                 .map(|order| {
//                     Decimal::from_str(&order.price).unwrap()
//                         * Decimal::from_str(&order.quantity).unwrap()
//                 })
//                 .collect();
//             let reduce_only_val = reduce_only_val
//                 .into_iter()
//                 .fold(Decimal::from_i32(0).unwrap(), |acc, x| acc + x);
//             println!(
//                 "{} {}",
//                 reduce_only_val / leverage,
//                 position.margin * decimal_shift * Decimal::from_str("0.99999").unwrap()
//             );
//             assert!(
//                 reduce_only_val / leverage
//                     > position.margin * decimal_shift * Decimal::from_str("0.99999").unwrap()
//             );
//         }
//     }

//     #[test]
//     fn spread_test() {
//         let std_dev = Decimal::from_str("5").unwrap();
//         let risk_aversion = Decimal::from_str("0.6").unwrap();
//         let spread_res = calc_spread_res(std_dev * std_dev, risk_aversion);
//         let spread_mid = calc_spread_mid(std_dev * std_dev, risk_aversion);
//         println!("{} {}", spread_res, spread_mid);
//     }

// #[test]
// fn initialization_test() {
//     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//     let msg = InstantiateMsg {
//         market_id: String::from("some market"),
//         sub_account: String::from("some account"),
//         fee_recipient: String::from("some recipient"),
//         risk_aversion: String::from("0.5"),
//         price_distribution_rate: String::from("2.5"),
//         leverage: String::from("2"),
//         decimal_shift: String::from("1000000"),
//         slices_per_spread_bp: String::from("0.2"),
//         ratio_active_capital: String::from("0.2"),
//         manager: String::from("some manager"),
//     };
//     let info = mock_info("creator", &coins(1000, "earth"));

//     // we can just call .unwrap() to assert this was a success
//     let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//     assert_eq!(0, res.messages.len());

//     // it worked, let's query the state
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::CheckState {}).unwrap();
//     let value: State = from_binary(&res).unwrap();
//     let expected = State {
//         market_id: String::from("some market"),
//         sub_account: String::from("some account"),
//         fee_recipient: String::from("some recipient"),
//         risk_aversion: String::from("0.5"),
//         price_distribution_rate: String::from("2.5"),
//         leverage: String::from("2"),
//         decimal_shift: String::from("1000000"),
//         owner: info.sender,
//         slices_per_spread_bp: String::from("0.2"),
//         ratio_active_capital: String::from("0.2"),
//         manager: todo!(),
//     };
//     assert_eq!(expected, value);
// }

// #[test]
// fn create_all_orders_test() {
//     // Define vars
//     let total_notional_balance = String::from("200000000000");
//     let standard_deviation = String::from("2000000");
//     let mid_price = String::from("4000000000");
//     let leverage = String::from("2");

//     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//     let msg = InstantiateMsg {
//         market_id: String::from("some market"),
//         sub_account: String::from("some account"),
//         fee_recipient: String::from("some recipient"),
//         risk_aversion: String::from("0.5"),
//         price_distribution_rate: String::from("2.5"),
//         leverage: leverage.clone(),
//         decimal_shift: String::from("1000000"),
//         slices_per_spread_bp: String::from("0.2"),
//         ratio_active_capital: String::from("0.2"),
//     };
//     let info = mock_info("creator", &coins(1000, "earth"));

//     // we can just call .unwrap() to assert this was a success
//     let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//     assert_eq!(0, res.messages.len());

//     // No position
//     let msg = QueryMsg::GetAction {
//         position: Position {
//             is_long: false,
//             quantity: String::from("0"),
//             avg_price: String::from("0"),
//             margin: String::from("0"),
//             cum_funding_entry: String::from("0"),
//         },
//         total_notional_balance: total_notional_balance.clone(),
//         standard_deviation: standard_deviation.clone(),
//         mid_price: mid_price.clone(),
//     };

//     let res = query(deps.as_ref(), mock_env(), msg).unwrap();
//     let value: WrappedGetActionResponse = from_binary(&res).unwrap();
//     let notional_value: Vec<Decimal> = value
//         .orders_to_open
//         .iter()
//         .map(|order| {
//             Decimal::from_str(&order.price).unwrap()
//                 * Decimal::from_str(&order.quantity).unwrap()
//         })
//         .collect();
//     let notional_value = notional_value
//         .into_iter()
//         .fold(Decimal::from_i32(0).unwrap(), |acc, x| acc + x);
//     // assert_eq!(notional_value  / Decimal::from_str(&leverage).unwrap(), Decimal::from_str(&total_notional_balance).unwrap());
//     println!("{}", value);
//     println!("{}", notional_value);

//     // No position
//     let quantity = Decimal::from_str("3").unwrap();
//     let avg_price = Decimal::from_str("3999000000").unwrap();
//     let margin = quantity * avg_price / Decimal::from_str(&leverage).unwrap();

//     let msg = QueryMsg::GetAction {
//         position: Position {
//             is_long: true,
//             quantity: quantity.to_string(),
//             avg_price: avg_price.to_string(),
//             margin: margin.to_string(),
//             cum_funding_entry: String::from("0"),
//         },
//         total_notional_balance: total_notional_balance.clone(),
//         standard_deviation: standard_deviation.clone(),
//         mid_price: mid_price.clone(),
//     };

//     let res = query(deps.as_ref(), mock_env(), msg).unwrap();
//     let value: WrappedGetActionResponse = from_binary(&res).unwrap();
//     let notional_value: Vec<Decimal> = value
//         .orders_to_open
//         .iter()
//         .map(|order| {
//             Decimal::from_str(&order.price).unwrap()
//                 * Decimal::from_str(&order.quantity).unwrap()
//         })
//         .collect();
//     let notional_value = notional_value
//         .into_iter()
//         .fold(Decimal::from_i32(0).unwrap(), |acc, x| acc + x);
//     // assert_eq!(notional_value  / Decimal::from_str(&leverage).unwrap(), Decimal::from_str(&total_notional_balance).unwrap());
//     println!("{}", value);
//     println!("{}", notional_value);
// }
// }
