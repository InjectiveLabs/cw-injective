use std::str::FromStr;

use crate::{
    exchange::{DerivativeOrder, OrderData, WrappedDerivativeLimitOrder, WrappedDerivativeMarket, WrappedPosition},
    state::State,
    utils::{div_dec, div_int, min, sub_abs, sub_no_overflow, sub_no_overflow_int},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};

/// Calculates the inventory imbalance parameter from the margin of an open position and the total deposited balance
/// # Arguments
/// * `position` - The position we have taken, if any
/// * `mid_price` - The true center between the best bid and ask
/// * `max_active_capital_utilization_ratio` - A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book
/// * `total_deposit_balance` - The total quote balance LPed
/// # Returns
/// * `inventory_imbalance` - A relationship between margined position and total deposit balance (margin/total_deposit_balance). Is
///    zero if there is no position open.
/// * `imbalance_is_long` - True if the imbalance is skewed towards being long
pub fn inventory_imbalance_deriv(position: &Option<WrappedPosition>, mid_price: Decimal, max_active_capital_utilization_ratio: Decimal, total_deposit_balance: Decimal) -> (Decimal, bool) {
    match position {
        None => (Decimal::zero(), true),
        Some(position) => {
            let unrealized_pnl_ratio = div_dec(mid_price - position.entry_price, position.entry_price);
            let unrealized_pnl_notionial = unrealized_pnl_ratio * position.margin;
            let position_value = position.margin + unrealized_pnl_notionial;
            let max_margin = max_active_capital_utilization_ratio * total_deposit_balance;
            let inventory_imbalance = div_dec(position_value, max_margin);
            (inventory_imbalance, position.is_long)
        }
    }
}

/// Determines the new orders that should be placed between the price bounds. The value of each
/// order will be constant (close to constant) across each price step. If there is a position
/// open on the opposite side, it will place reduce only orders from the start_price to try to reduce the
/// remaining quantity of the position.
/// # Arguments
/// * `start_price` - Could be the price of the new head or the last order to keep
/// * `end_price` - Could be the price of the new tail or the first order to keep
/// * `total_margin_balance_for_new_orders` - The total margin that we are allowed to allocate to the new orders
/// * `num_orders_to_keep` - The number of orders that we would like to keep resting on the book
/// * `position_qty_to_reduce` - The remaining quantity of the position that we need to reduce
/// * `is_buy` - If this block of orders will be on the buyside
/// * `state` - State that the contract was initialized with
/// * `market` - Derivative market information
/// # Returns
/// * `new_orders_to_open` - A list of all the new orders that we would like to place
/// * `position_qty_to_reduce` - The remaining position quantity that we need to reduce. Is zero if there is none left
/// * `total_margin_balance_for_new_orders` - The remaining total margin that we are allowed to allocate to any additional new orders
pub fn create_orders_between_bounds_deriv(
    start_price: Decimal,
    end_price: Decimal,
    mut total_margin_balance_for_new_orders: Decimal,
    num_orders_to_keep: usize,
    mut position_qty_to_reduce: Decimal,
    touch_head: bool,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<DerivativeOrder>, Decimal, Decimal) {
    let mut new_orders_to_open: Vec<DerivativeOrder> = Vec::new();
    let num_open_orders = Uint256::from_str(&num_orders_to_keep.to_string()).unwrap();
    let num_orders_to_open = sub_no_overflow_int(state.order_density, num_open_orders);
    if !num_orders_to_open.is_zero() {
        let value_per_order = div_int(total_margin_balance_for_new_orders, num_orders_to_open) * state.leverage;
        let price_range = sub_abs(start_price, end_price);
        let price_step = div_int(price_range, num_orders_to_open);
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        let mut current_price = if touch_head {
            start_price
        } else {
            if is_buy {
                sub_no_overflow(start_price, price_step)
            } else {
                start_price + price_step
            }
        };
        for _ in 0..num_orders_to_open {
            let new_order_quantity = div_dec(value_per_order, current_price);
            if position_qty_to_reduce == Decimal::zero() {
                // If there is no position qty, no need to make reduce only orders
                let new_order = DerivativeOrder::new(state, current_price, new_order_quantity, is_buy, false, market);
                total_margin_balance_for_new_orders = sub_no_overflow(total_margin_balance_for_new_orders, new_order.get_margin());
                new_orders_to_open.push(new_order);
            } else {
                // This whole order should be reduce only
                let new_reduce_only_order = DerivativeOrder::new(state, current_price, new_order_quantity, is_buy, true, market);
                position_qty_to_reduce = sub_no_overflow(position_qty_to_reduce, new_order_quantity);
                new_orders_to_open.push(new_reduce_only_order);
            }
            current_price = if is_buy {
                sub_no_overflow(current_price, price_step)
            } else {
                current_price + price_step
            };
        }
    }
    (new_orders_to_open, position_qty_to_reduce, total_margin_balance_for_new_orders)
}

/// Creates orders in the situation where there is more room between the newly proposed head and the first order to keep
/// than there is between the end of the last order to keep and newly proposed tail. We need to create new orders between it and the tail before we
/// manage the reduce only orders at the start of the orders to keep block.
/// # Arguments
/// * `new_head` - The start price bound for the base case of order creation
/// * `total_margin_balance_for_new_orders` - The total margin that we are allowed to allocate to the new orders
/// * `orders_to_keep` - The number of orders that we would like to keep resting on the book
/// * `position_qty_to_reduce` - The remaining quantity of the position that we need to reduce
/// * `is_buy` - If this block of orders will be on the buyside
/// * `state` - State that the contract was initialized with
/// * `market` - Derivative market information
/// # Returns
/// * `additional_orders_to_cancel` - A list of all the additional orders that we need to cancel
/// * `additional_orders_to_open` - A list of new orders that we need to open
pub fn create_orders_tail_to_head_deriv(
    new_head: Decimal,
    total_margin_balance_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    position_qty_to_reduce: Decimal,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<DerivativeOrder>, Vec<OrderData>) {
    let (orders_to_open_from_base_case, position_qty_to_reduce, total_margin_balance_for_new_orders) = create_orders_between_bounds_deriv(
        new_head,
        orders_to_keep.first().unwrap().order_info.price,
        total_margin_balance_for_new_orders,
        orders_to_keep.len(),
        position_qty_to_reduce,
        true,
        is_buy,
        state,
        market,
    );
    let (additional_orders_to_cancel, orders_to_open_from_reduce_only_management, _, _) = manage_reduce_only_deriv(
        orders_to_keep,
        total_margin_balance_for_new_orders,
        position_qty_to_reduce,
        false,
        is_buy,
        state,
        market,
    );
    (
        vec![orders_to_open_from_base_case, orders_to_open_from_reduce_only_management].concat(),
        additional_orders_to_cancel,
    )
}

/// Creates orders in the situation where there is more room between the end of the last order to keep and newly proposed tail
/// than there is between the newly proposed head and the first order to keep. We need to manage the reduce only orders at the
/// start of the orders to keep block before we create new orders between it and the tail.
/// # Arguments
/// * `new_tail` - The end price bound for the base case of order creation
/// * `total_margin_balance_for_new_orders` - The total margin that we are allowed to allocate to the new orders
/// * `orders_to_keep` - The number of orders that we would like to keep resting on the book
/// * `position_qty_to_reduce` - The remaining quantity of the position that we need to reduce
/// * `is_buy` - If this block of orders will be on the buyside
/// * `state` - State that the contract was initialized with
/// * `market` - Derivative market information
/// # Returns
/// * `additional_orders_to_cancel` - A list of all the additional orders that we need to cancel
/// * `additional_orders_to_open` - A list of new orders that we need to open
pub fn create_orders_head_to_tail_deriv(
    new_tail: Decimal,
    total_margin_balance_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    position_qty_to_reduce: Decimal,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<DerivativeOrder>, Vec<OrderData>) {
    let (additional_orders_to_cancel, orders_to_open_from_reduce_only_management, total_margin_balance_for_new_orders, position_qty_to_reduce) =
        manage_reduce_only_deriv(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            position_qty_to_reduce,
            true,
            is_buy,
            state,
            market,
        );
    let (orders_to_open_from_base_case, _, _) = create_orders_between_bounds_deriv(
        orders_to_keep.last().unwrap().order_info.price,
        new_tail,
        total_margin_balance_for_new_orders,
        orders_to_keep.len(),
        position_qty_to_reduce,
        false,
        is_buy,
        state,
        market,
    );
    (
        vec![orders_to_open_from_reduce_only_management, orders_to_open_from_base_case].concat(),
        additional_orders_to_cancel,
    )
}

/// Traverses through a list of existing orders, either flipping reduce only orders to regular or vice versa depending on the kind of order
/// and position quantity remaing for reduction.
/// # Arguments
/// * `orders_to_keep` - The number of orders that we would like to keep resting on the book
/// * `total_margin_balance_for_new_orders` - The total margin that we are allowed to allocate to the new orders
/// * `position_qty_to_reduce` - The remaining quantity of the position that we need to reduce
/// * `is_buy` - If this block of orders will be on the buyside
/// * `state` - State that the contract was initialized with
/// * `market` - Derivative market information
/// # Returns
/// * `additional_orders_to_cancel` - A list of all the additional orders that we need to cancel in order to replace
/// * `additional_orders_to_open` - A list of alll the additional orders that we need to open to replace the ones we cancelled
/// * `total_margin_balance_for_new_orders` - The remaining total margin that we are allowed to allocate to any additional new orders
/// * `position_qty_to_reduce` - The remaining position quantity that we need to reduce. Is zero if there is none left
fn manage_reduce_only_deriv(
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    mut total_margin_balance_for_new_orders: Decimal,
    mut position_qty_to_reduce: Decimal,
    is_before_base_case: bool,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> (Vec<OrderData>, Vec<DerivativeOrder>, Decimal, Decimal) {
    let mut additional_orders_to_open: Vec<DerivativeOrder> = Vec::new();
    let mut additional_orders_to_cancel: Vec<OrderData> = Vec::new();
    orders_to_keep.into_iter().for_each(|o| {
        if position_qty_to_reduce > Decimal::zero() {
            // There is a position to reduce
            if o.is_reduce_only() {
                position_qty_to_reduce = sub_no_overflow(position_qty_to_reduce, o.order_info.quantity);
            } else {
                // We need to cancel the order and create an order with the min of remaining position qty to reduce and the cancelled order's qty
                additional_orders_to_cancel.push(OrderData::new(o.order_hash, state, market));
                let new_quantity = min(position_qty_to_reduce, o.order_info.quantity);
                let new_reduce_only_order = DerivativeOrder::new(state, o.order_info.price, new_quantity, is_buy, true, market);
                additional_orders_to_open.push(new_reduce_only_order);
                position_qty_to_reduce = sub_no_overflow(position_qty_to_reduce, new_quantity);
                if !is_before_base_case {
                    total_margin_balance_for_new_orders = total_margin_balance_for_new_orders + o.margin;
                }
            }
        } else {
            // No position to reduce
            if o.is_reduce_only() {
                // If we encounter a reduce only order we need to cancel it and replace it if we have sufficient allocated margin balance
                additional_orders_to_cancel.push(OrderData::new(o.order_hash, state, market));
                let new_quantity = min(div_dec(total_margin_balance_for_new_orders, o.order_info.price), o.order_info.quantity);
                let new_order = DerivativeOrder::new(state, o.order_info.price, new_quantity, is_buy, false, market);
                if new_order.get_margin() <= total_margin_balance_for_new_orders {
                    additional_orders_to_open.push(new_order);
                    total_margin_balance_for_new_orders = sub_no_overflow(total_margin_balance_for_new_orders, o.margin);
                }
            }
        }
    });
    (
        additional_orders_to_cancel,
        additional_orders_to_open,
        total_margin_balance_for_new_orders,
        position_qty_to_reduce,
    )
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
//             subaccount_id: String::from(""),
//             order_density: Uint256::from_str(&order_density).unwrap(),
//             active_capital: Decimal::from_str("0.2").unwrap(),
//             min_head_to_tail_deviation_ratio: Decimal::from_str("0.03").unwrap(),
//             max_mid_price_tail_deviation_ratio: Decimal::from_str("0.06").unwrap(),
//             head_change_tolerance_ratio: Decimal::zero(),
//             leverage: Decimal::from_str(&leverage).unwrap(),
//             reservation_price_sensitivity_ratio: Decimal::zero(),
//             spread_param: Decimal::zero(),
//             fee_recipient: String::from(""),
//             lp_token_address: None,
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
