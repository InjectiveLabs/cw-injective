use std::str::FromStr;

use crate::{
    exchange::{DerivativeOrder, OrderData, WrappedDerivativeLimitOrder, WrappedDerivativeMarket, WrappedPosition},
    state::State,
    utils::{div_dec, div_int, sub_abs, sub_no_overflow},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};

/// Calculates the inventory imbalance from the margined value of an open position
/// # Arguments
/// * `inv_base_val` - The notional value of all base assets
/// * `inv_val` - The total notional value of all assets
/// # Returns
/// * `inv_imbalance` - The inventory imbalance parameter
/// * `imbal_is_long` - True if the imbalance is skewed in favor of the base asset
pub fn inv_imbalance_deriv(position: &Option<WrappedPosition>, inv_val: Decimal) -> (Decimal, bool) {
    match position {
        None => (Decimal::zero(), true),
        Some(position) => {
            let position_value = position.margin;
            let inv_imbalance = div_dec(position_value, inv_val);
            (inv_imbalance, position.is_long)
        }
    }
}

/// Determines the new orders that should be placed between the new head/tail. Ensures
/// that the notional value of all open orders will be equal to the allocated value
/// passed in as a parameter. The value of each order will be constant (close to constant)
/// across each price step. If there is a position open on the opposite side, it will place
/// reduce only orders starting from the head to try to reduce the position.
/// # Arguments
/// * `new_head` - The new head (closest to the reservation price)
/// * `new_tail` - The new tail (farthest from the reservation price)
/// * `alloc_val_for_new_orders` - The value that all the new orders should sum to
/// * `position_qty` - A qty of position that we want to reduce
/// * `is_buy` - True if all open_orders are buy. False if they are all sell
/// * `state` - Contract state
/// # Returns
/// * `orders_to_open` - A list of all the new orders that we would like to place
pub fn base_deriv(
    new_head: Decimal,
    new_tail: Decimal,
    mut alloc_val_for_new_orders: Decimal,
    num_orders_to_keep: usize,
    mut position_qty: Decimal,
    touch_head: bool,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<DerivativeOrder>, Decimal, Decimal) {
    println!("bal2 {}", alloc_val_for_new_orders);
    let mut orders_to_open: Vec<DerivativeOrder> = Vec::new();
    let num_open_orders = Uint256::from_str(&num_orders_to_keep.to_string()).unwrap();
    let num_orders_to_open = state.order_density - num_open_orders;
    let val_per_order = alloc_val_for_new_orders / num_orders_to_open;
    let val_per_order = val_per_order * state.leverage;
    let price_dist = sub_abs(new_head, new_tail);
    let price_step = div_int(price_dist, num_orders_to_open);
    let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
    let mut current_price = if touch_head {
        new_head
    } else {
        if is_buy {
            new_head - price_step
        } else {
            new_head + price_step
        }
    };
    for _ in 0..num_orders_to_open {
        let qty = div_dec(val_per_order, current_price);
        if position_qty == Decimal::zero() {
            // If there is no position qty, no need to make reduce only orders
            let new_order = DerivativeOrder::new(state, current_price, qty, is_buy, false, market);
            alloc_val_for_new_orders = sub_no_overflow(alloc_val_for_new_orders, new_order.get_margin());
            println!("{:?}", new_order);
            orders_to_open.push(new_order);
        } else {
            // We need to manage reduce only orders here
            if qty > position_qty {
                let new_order_reduce = DerivativeOrder::new(state, current_price, position_qty, is_buy, true, market);
                orders_to_open.push(new_order_reduce);
                position_qty = Decimal::zero();
            } else {
                // This whole order should be reduce only
                let new_order_reduce = DerivativeOrder::new(state, current_price, qty, is_buy, true, market);
                position_qty = sub_no_overflow(position_qty, qty);
                orders_to_open.push(new_order_reduce);
            }
        }
        current_price = if is_buy {
            current_price - price_step
        } else {
            current_price + price_step
        };
    }
    (orders_to_open, position_qty, alloc_val_for_new_orders)
}

pub fn tail_to_head_deriv(
    new_head: Decimal,
    alloc_val_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    position_qty: Decimal,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<DerivativeOrder>, Vec<OrderData>) {
    let (orders_to_open_a, position_qty, alloc_val_for_new_orders) = base_deriv(
        new_head,
        orders_to_keep.first().unwrap().order_info.price,
        alloc_val_for_new_orders,
        orders_to_keep.len(),
        position_qty,
        true,
        is_buy,
        state,
        market,
    );
    let (additional_orders_to_cancel, orders_to_open_b, _, _) = handle_reduce_only(orders_to_keep.clone(), alloc_val_for_new_orders, position_qty, is_buy, state, market);
    (vec![orders_to_open_a, orders_to_open_b].concat(), additional_orders_to_cancel)
}

pub fn head_to_tail_deriv(
    new_tail: Decimal,
    alloc_val_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    position_qty: Decimal,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<DerivativeOrder>, Vec<OrderData>) {
    println!("bal: {}", alloc_val_for_new_orders);
    let (additional_orders_to_cancel, orders_to_open_a, alloc_val_for_new_orders, position_qty) = handle_reduce_only(orders_to_keep.clone(), alloc_val_for_new_orders, position_qty, is_buy, state, market);
    let (orders_to_open_b, _, _) = base_deriv(
        orders_to_keep.last().unwrap().order_info.price,
        new_tail,
        alloc_val_for_new_orders,
        orders_to_keep.len(),
        position_qty,
        false,
        is_buy,
        state,
        market,
    );
    (vec![orders_to_open_a, orders_to_open_b].concat(), additional_orders_to_cancel)
}

fn handle_reduce_only(
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    mut alloc_val_for_new_orders: Decimal,
    mut position_qty: Decimal,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<OrderData>, Vec<DerivativeOrder>, Decimal, Decimal) {
    let mut additional_orders_to_open: Vec<DerivativeOrder> = Vec::new();
    let mut additional_orders_to_cancel: Vec<OrderData> = Vec::new();
    orders_to_keep.iter().for_each(|o| {
        if position_qty > Decimal::zero() {
            if o.order_info.quantity > position_qty {
                additional_orders_to_cancel.push(OrderData::new(o, state));
                let new_order_reduce = DerivativeOrder::new(state, o.order_info.price, position_qty, is_buy, true, market);
                additional_orders_to_open.push(new_order_reduce);
                position_qty = Decimal::zero();
                alloc_val_for_new_orders = alloc_val_for_new_orders + o.margin;
            } else {
                if o.is_reduce_only() {
                    position_qty = position_qty - o.order_info.quantity;
                } else {
                    // This whole order should be reduce only
                    additional_orders_to_cancel.push(OrderData::new(o, state));
                    let new_order_reduce = DerivativeOrder::new(state, o.order_info.price, o.order_info.quantity, is_buy, true, market);
                    additional_orders_to_open.push(new_order_reduce);
                    position_qty = position_qty - o.order_info.quantity;
                    alloc_val_for_new_orders = alloc_val_for_new_orders + o.margin;
                }
            }
        } else {
            if o.is_reduce_only() {
                additional_orders_to_cancel.push(OrderData::new(o, state));
                let new_order = DerivativeOrder::new(
                    state,
                    o.order_info.price,
                    sub_no_overflow(o.order_info.quantity, position_qty),
                    is_buy,
                    false,
                    market,
                );
                if new_order.get_margin() < alloc_val_for_new_orders {
                    additional_orders_to_open.push(new_order);
                    alloc_val_for_new_orders = sub_no_overflow(alloc_val_for_new_orders, o.margin);
                }
            }
        }
    });
    (additional_orders_to_cancel, additional_orders_to_open, alloc_val_for_new_orders, position_qty)
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         derivative::base_deriv,
//         exchange::{DerivativeMarket, DerivativeOrder, WrappedDerivativeLimitOrder, WrappedDerivativeMarket, WrappedOrderInfo},
//         state::State,
//         utils::div_dec,
//     };
//     use cosmwasm_std::{Decimal256 as Decimal, Uint256};
//     use std::str::FromStr;

//     use super::handle_reduce_only;

//     #[test]
//     fn base_buy_deriv_test() {
//         let leverage = Decimal::from_str("2.5").unwrap();
//         let state = mock_state(leverage.to_string(), String::from("10"));
//         let market = mock_market();
//         base_deriv_test(
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             0,
//             Decimal::zero(),
//             true,
//             true,
//             &state,
//             &market,
//         );
//         base_deriv_test(
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             3,
//             Decimal::zero(),
//             true,
//             true,
//             &state,
//             &market,
//         );
//         base_deriv_test(
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             0,
//             div_dec(
//                 Decimal::from_str("999900000000000000").unwrap(),
//                 Decimal::from_str("100000000000000").unwrap(),
//             ),
//             true,
//             true,
//             &state,
//             &market,
//         );
//         base_deriv_test(
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             3,
//             div_dec(
//                 Decimal::from_str("999900000000000000").unwrap(),
//                 Decimal::from_str("100000000000000").unwrap(),
//             ),
//             true,
//             true,
//             &state,
//             &market,
//         );
//     }

//     #[test]
//     fn base_sell_deriv_test() {
//         let leverage = Decimal::from_str("2.5").unwrap();
//         let state = mock_state(leverage.to_string(), String::from("10"));
//         let market = mock_market();
//         base_deriv_test(
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             0,
//             Decimal::zero(),
//             true,
//             false,
//             &state,
//             &market,
//         );
//         base_deriv_test(
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             3,
//             Decimal::zero(),
//             true,
//             false,
//             &state,
//             &market,
//         );
//         base_deriv_test(
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             0,
//             div_dec(
//                 Decimal::from_str("999900000000000000").unwrap(),
//                 Decimal::from_str("100000000000000").unwrap(),
//             ),
//             true,
//             false,
//             &state,
//             &market,
//         );
//         base_deriv_test(
//             Decimal::from_str("99990000000000").unwrap(),
//             Decimal::from_str("100000000000000").unwrap(),
//             Decimal::from_str("9999000000000000000").unwrap(),
//             3,
//             div_dec(
//                 Decimal::from_str("999900000000000000").unwrap(),
//                 Decimal::from_str("100000000000000").unwrap(),
//             ),
//             true,
//             false,
//             &state,
//             &market,
//         );
//     }

//     #[test]
//     fn reduce_buy_test() {
//         let leverage = Decimal::from_str("1").unwrap();
//         let state = mock_state(leverage.to_string(), String::from("10"));
//         let market = mock_market();
//         let orders_to_keep = mock_wrapped_deriv_limit(
//             Decimal::from_str("10000000000000000").unwrap(),
//             Decimal::from_str("100000000").unwrap(),
//             Decimal::from_str("100").unwrap(),
//             0,
//             true,
//             leverage,
//         );
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::zero(), true, &state, &market);
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::from_str("2").unwrap(), true, &state, &market);
//         handle_reduce_only_test(orders_to_keep, Decimal::from_str("200000000").unwrap(), true, &state, &market);

//         let orders_to_keep = mock_wrapped_deriv_limit(
//             Decimal::from_str("10000000000000000").unwrap(),
//             Decimal::from_str("100000000").unwrap(),
//             Decimal::from_str("100").unwrap(),
//             2,
//             true,
//             leverage,
//         );
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::zero(), true, &state, &market);
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::from_str("2").unwrap(), true, &state, &market);
//         handle_reduce_only_test(orders_to_keep, Decimal::from_str("200000000").unwrap(), true, &state, &market);
//     }

//     #[test]
//     fn reduce_sell_test() {
//         let leverage = Decimal::from_str("1").unwrap();
//         let state = mock_state(leverage.to_string(), String::from("10"));
//         let market = mock_market();
//         let orders_to_keep = mock_wrapped_deriv_limit(
//             Decimal::from_str("10000000000000000").unwrap(),
//             Decimal::from_str("100000000").unwrap(),
//             Decimal::from_str("100").unwrap(),
//             0,
//             false,
//             leverage,
//         );
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::zero(), false, &state, &market);
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::from_str("2").unwrap(), false, &state, &market);
//         handle_reduce_only_test(orders_to_keep, Decimal::from_str("200000000").unwrap(), false, &state, &market);

//         let orders_to_keep = mock_wrapped_deriv_limit(
//             Decimal::from_str("10000000000000000").unwrap(),
//             Decimal::from_str("100000000").unwrap(),
//             Decimal::from_str("100").unwrap(),
//             2,
//             false,
//             leverage,
//         );
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::zero(), false, &state, &market);
//         handle_reduce_only_test(orders_to_keep.clone(), Decimal::from_str("2").unwrap(), false, &state, &market);
//         handle_reduce_only_test(orders_to_keep, Decimal::from_str("200000000").unwrap(), false, &state, &market);
//     }

//     // Test Helpers
//     fn base_deriv_test(
//         new_head: Decimal,
//         new_tail: Decimal,
//         alloc_val_for_new_orders: Decimal,
//         num_orders_to_keep: usize,
//         position_qty: Decimal,
//         touch_head: bool,
//         is_buy: bool,
//         state: &State,
//         market: &WrappedDerivativeMarket,
//     ) {
//         let max_tolerance = Decimal::from_str("0.01").unwrap();
//         let (new_orders, rem_position_qty) = base_deriv(
//             new_head,
//             new_tail,
//             alloc_val_for_new_orders,
//             num_orders_to_keep,
//             position_qty,
//             touch_head,
//             is_buy,
//             state,
//             market,
//         );
//         let val_per_order = alloc_val_for_new_orders / state.order_density;
//         let val_per_order = val_per_order * state.leverage;
//         let mut total_reduce_only_qty = Decimal::zero();
//         let mut total_value = Decimal::zero();
//         let mut num_same_priced_orders = 0;
//         for i in 0..new_orders.len() {
//             println!("{} {} {}", new_orders[i].get_price(), new_orders[i].get_qty(), new_orders[i].get_val())
//         }

//         for i in 0..new_orders.len() {
//             if new_orders[i].is_reduce_only() {
//                 total_reduce_only_qty = total_reduce_only_qty + new_orders[i].get_qty();
//             }
//             total_value = total_value + new_orders[i].get_val();
//             if i > 0 {
//                 // Ensure that price is changing in the right direction
//                 if !(new_orders[i - 1].is_reduce_only() && !new_orders[i].is_reduce_only()) {
//                     if is_buy {
//                         assert!(new_orders[i - 1].get_price() > new_orders[i].get_price());
//                     } else {
//                         assert!(new_orders[i - 1].get_price() < new_orders[i].get_price());
//                     }
//                 }
//                 // Ensure that the notional val of orders is consistent
//                 let value = if new_orders[i - 1].is_reduce_only() && !new_orders[i].is_reduce_only() {
//                     new_orders[i - 1].get_val() + new_orders[i].get_val()
//                 } else if new_orders[i - 1].is_reduce_only() {
//                     new_orders[i - 1].get_val()
//                 } else {
//                     new_orders[i].get_val()
//                 };
//                 if new_orders[i - 1].get_price() == new_orders[i].get_price() {
//                     num_same_priced_orders += 1;
//                 }
//                 assert!(value * (max_tolerance * val_per_order) >= val_per_order);
//             }
//         }

//         // Ensure we never have too many orders
//         assert_eq!(
//             new_orders.len() - num_same_priced_orders,
//             state.order_density.to_string().parse::<usize>().unwrap() - num_orders_to_keep
//         );

//         // Esure that we tried the best we could to reduce the position
//         assert!(position_qty >= total_reduce_only_qty);
//         if rem_position_qty == Decimal::zero() {
//             assert!((position_qty * max_tolerance) >= position_qty - total_reduce_only_qty);
//         } else {
//             for i in 0..new_orders.len() {
//                 assert!(new_orders[i].is_reduce_only());
//             }
//         }

//         // Ensure that we used all possible inventory within a tolerance
//         if position_qty.is_zero() {
//             assert!(div_dec(total_value, state.leverage) + (alloc_val_for_new_orders * max_tolerance) >= alloc_val_for_new_orders);
//             assert!(div_dec(total_value, state.leverage) - (alloc_val_for_new_orders * max_tolerance) <= alloc_val_for_new_orders);
//         } else {
//             assert!(div_dec(total_value, state.leverage) >= alloc_val_for_new_orders - val_per_order);
//         }
//     }

//     fn handle_reduce_only_test(
//         orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
//         position_qty: Decimal,
//         is_buy: bool,
//         state: &State,
//         market: &WrappedDerivativeMarket,
//     ) {
//         let (additional_orders_to_cancel, orders_to_open) = handle_reduce_only(orders_to_keep.clone(), position_qty, is_buy, state, market);
//         let mut total_val_before: Decimal = Decimal::zero();
//         for order in orders_to_keep.iter() {
//             total_val_before = total_val_before + (order.order_info.price * order.order_info.quantity);
//         }
//         let mut total_val_after: Decimal = Decimal::zero();
//         let mut reduce_qty_after = Decimal::zero();
//         let mut num_orders_rem = 0;
//         for order in orders_to_keep.clone().iter() {
//             let mut found = false;
//             for cancel in &additional_orders_to_cancel {
//                 if cancel.order_hash == order.order_hash {
//                     found = true;
//                 }
//             }
//             if !found {
//                 total_val_after = total_val_after + (order.order_info.price * order.order_info.quantity);
//                 if order.is_reduce_only() {
//                     reduce_qty_after = reduce_qty_after + order.order_info.quantity;
//                 }
//                 num_orders_rem += 1;
//             }
//         }
//         for order in orders_to_open.iter() {
//             total_val_after =
//                 total_val_after + (Decimal::from_str(&order.order_info.price).unwrap() * Decimal::from_str(&order.order_info.quantity).unwrap());
//             if order.is_reduce_only() {
//                 reduce_qty_after = reduce_qty_after + Decimal::from_str(&order.order_info.quantity).unwrap();
//             }
//             num_orders_rem += 1;
//         }
//         if position_qty.is_zero() {
//             assert_eq!(total_val_before, total_val_after);
//         } else {
//             assert!(total_val_before >= total_val_after);
//         }
//         assert_eq!(position_qty, reduce_qty_after);
//         assert_eq!(
//             num_orders_rem,
//             orders_to_keep.len() - additional_orders_to_cancel.len() + orders_to_open.len()
//         );
//         assert!(Uint256::from_str(&num_orders_rem.to_string()).unwrap() == state.order_density);

//         println!("val before {} val after {}", total_val_before, total_val_after);
//         println!("cancelled {} orders", additional_orders_to_cancel.len());
//         println!("placed {} orders", orders_to_open.len());
//     }

//     fn mock_state(leverage: String, order_density: String) -> State {
//         State {
//             market_id: String::from(""),
//             is_deriv: true,
//             subaccount_id: String::from(""),
//             order_density: Uint256::from_str(&order_density).unwrap(),
//             active_capital: Decimal::from_str("0.2").unwrap(),
//             min_tail_dist: Decimal::from_str("0.03").unwrap(),
//             tail_dist_from_mid: Decimal::from_str("0.06").unwrap(),
//             head_chg_tol: Decimal::zero(),
//             leverage: Decimal::from_str(&leverage).unwrap(),
//             reservation_param: Decimal::zero(),
//             spread_param: Decimal::zero(),
//             max_market_data_delay: 0,
//             fee_recipient: String::from(""),
//             cw_20_contract: None,
//         }
//     }

//     fn mock_market() -> WrappedDerivativeMarket {
//         DerivativeMarket {
//             ticker: String::from(""),
//             oracle_base: String::from(""),
//             oracle_quote: String::from(""),
//             oracle_type: 0,
//             oracle_scale_factor: 0,
//             quote_denom: String::from(""),
//             market_id: String::from(""),
//             initial_margin_ratio: String::from("0"),
//             maintenance_margin_ratio: String::from("0"),
//             maker_fee_rate: String::from("0"),
//             taker_fee_rate: String::from("0"),
//             isPerpetual: true,
//             status: 0,
//             min_price_tick_size: String::from("1000000"),
//             min_quantity_tick_size: String::from("0.00001"),
//         }
//         .wrap()
//         .unwrap()
//     }

//     fn mock_wrapped_deriv_limit(
//         value: Decimal,
//         mp: Decimal,
//         price_step_mult: Decimal,
//         num_reduce_only: usize,
//         is_buy: bool,
//         leverage: Decimal,
//     ) -> Vec<WrappedDerivativeLimitOrder> {
//         let mut orders: Vec<WrappedDerivativeLimitOrder> = Vec::new();
//         for i in 0..10 {
//             let price = if is_buy {
//                 mp - (Decimal::from_str(&i.to_string()).unwrap() * price_step_mult)
//             } else {
//                 mp + (Decimal::from_str(&i.to_string()).unwrap() * price_step_mult)
//             };
//             let quantity = div_dec(value, price);
//             let margin = if i < num_reduce_only {
//                 Decimal::zero()
//             } else {
//                 div_dec(quantity * price, leverage)
//             };
//             orders.push(WrappedDerivativeLimitOrder {
//                 trigger_price: None,
//                 order_info: WrappedOrderInfo {
//                     subaccount_id: "".to_string(),
//                     fee_recipient: "".to_string(),
//                     price,
//                     quantity,
//                 },
//                 order_type: 0,
//                 margin,
//                 fillable: Decimal::zero(),
//                 order_hash: i.to_string(),
//             });
//         }
//         orders
//     }
// }
