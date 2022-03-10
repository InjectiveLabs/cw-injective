use std::str::FromStr;

use crate::{
    exchange::{DerivativeOrder, OrderData, WrappedDerivativeLimitOrder, WrappedDerivativeMarket, WrappedPosition},
    state::State,
    utils::{div_dec, div_int, min, sub_abs, sub_no_overflow, sub_no_overflow_int},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};

/// # Description
/// Calculates the inventory imbalance parameter from the margin of an open position and the total deposited balance
/// # Formulas
/// * `inventory imbalance (case: no position)` = 0
/// * `inventory imbalance (case: open position)` = (margin + (((mid price - position entry price) / position entry price) * margin)) / (active capital utilization ratio * total deposit balance)
/// # Arguments
/// * `position` - The position we have taken, if any
/// * `mid_price` - The true center between the best bid and ask
/// * `max_active_capital_utilization_ratio` - A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book
/// * `total_deposit_balance` - The total quote balance LPed
/// # Returns
/// * `inventory_imbalance` - A relationship between margined position and total deposit balance (margin/total_deposit_balance). Is
///    zero if there is no position open.
/// * `imbalance_is_long` - True if the imbalance is skewed towards being long
pub fn inventory_imbalance_deriv(
    position: &Option<WrappedPosition>,
    mid_price: Decimal,
    max_active_capital_utilization_ratio: Decimal,
    total_deposit_balance: Decimal,
) -> (Decimal, bool) {
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

/// # Description
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
            let new_order_qty = div_dec(value_per_order, current_price);
            if position_qty_to_reduce == Decimal::zero() {
                // If there is no position qty, no need to make reduce only orders
                let new_order = DerivativeOrder::new(state, current_price, new_order_qty, is_buy, false, market);
                if !new_order.non_reduce_only_is_invalid() && !total_margin_balance_for_new_orders.is_zero() {
                    total_margin_balance_for_new_orders = sub_no_overflow(total_margin_balance_for_new_orders, new_order.get_margin());
                    new_orders_to_open.push(new_order);
                }
            } else {
                // This whole order should be reduce only
                let new_qty = min(position_qty_to_reduce, new_order_qty);
                let new_reduce_only_order = DerivativeOrder::new(state, current_price, new_qty, is_buy, true, market);
                position_qty_to_reduce = update_reduce_only(new_reduce_only_order, position_qty_to_reduce, &mut new_orders_to_open);
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

/// # Description
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
        is_buy,
        state,
        market,
    );
    (
        vec![orders_to_open_from_base_case, orders_to_open_from_reduce_only_management].concat(),
        additional_orders_to_cancel,
    )
}

/// # Description
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
            is_buy,
            state,
            market,
        );
    let (orders_to_open_from_base_case, _, _) = create_orders_between_bounds_deriv(
        orders_to_keep.last().unwrap().order_info.price,
        new_tail,
        total_margin_balance_for_new_orders,
        orders_to_keep.len() + orders_to_open_from_reduce_only_management.len() - additional_orders_to_cancel.len(),
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

/// # Description
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
                if position_qty_to_reduce > o.order_info.quantity {
                    // Update the position quantity requiring reduction to consider the RO order that we just encountered
                    position_qty_to_reduce = sub_no_overflow(position_qty_to_reduce, o.order_info.quantity);
                } else {
                    // We need to cancel the old RO and adjust its quantity to be specific to that which needs to be reduced
                    additional_orders_to_cancel.push(OrderData::new(o.order_hash, state, market));
                    let new_reduce_only_order = DerivativeOrder::new(state, o.order_info.price, position_qty_to_reduce, is_buy, true, market);
                    position_qty_to_reduce = update_reduce_only(new_reduce_only_order, position_qty_to_reduce, &mut additional_orders_to_open);
                }
            } else {
                // We need to cancel the order and create a new RO order to try to reduce the remaining position quantity
                additional_orders_to_cancel.push(OrderData::new(o.order_hash, state, market));
                total_margin_balance_for_new_orders = total_margin_balance_for_new_orders + o.margin;
                let new_qty = min(position_qty_to_reduce, o.order_info.quantity);
                let new_reduce_only_order = DerivativeOrder::new(state, o.order_info.price, new_qty, is_buy, true, market);
                position_qty_to_reduce = update_reduce_only(new_reduce_only_order, position_qty_to_reduce, &mut additional_orders_to_open);
            }
        } else {
            // No position to reduce
            if o.is_reduce_only() {
                // If we encounter a reduce only order we need to cancel it and replace it if we have sufficient allocated margin balance
                additional_orders_to_cancel.push(OrderData::new(o.order_hash, state, market));
                if !total_margin_balance_for_new_orders.is_zero() {
                    let new_qty = min(div_dec(total_margin_balance_for_new_orders, o.order_info.price), o.order_info.quantity);
                    let new_order = DerivativeOrder::new(state, o.order_info.price, new_qty, is_buy, false, market);
                    if !new_order.non_reduce_only_is_invalid() {
                        total_margin_balance_for_new_orders = sub_no_overflow(total_margin_balance_for_new_orders, new_order.get_margin());
                        additional_orders_to_open.push(new_order);
                    }
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

fn update_reduce_only(new_reduce_only_order: DerivativeOrder, position_qty_to_reduce: Decimal, orders_to_open: &mut Vec<DerivativeOrder>) -> Decimal {
    if !new_reduce_only_order.reduce_only_is_invalid() {
        // RO order is valid after rounding
        // Update the remaining position quantity to reduce
        let new_position_qty_to_reduce = sub_no_overflow(position_qty_to_reduce, new_reduce_only_order.get_qty());
        // Push new order to open
        orders_to_open.push(new_reduce_only_order);
        new_position_qty_to_reduce
    } else {
        // Position qty to reduce is likely very small here but we don't want to continue cancelling all subsequent orders so we set it to zero
        Decimal::zero()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        derivative::{create_orders_between_bounds_deriv, manage_reduce_only_deriv},
        exchange::{DerivativeMarket, DerivativeOrder, WrappedDerivativeLimitOrder, WrappedDerivativeMarket, WrappedOrderInfo},
        state::State,
        utils::div_dec,
    };
    use cosmwasm_std::{Decimal256 as Decimal, Uint256};
    use std::str::FromStr;

    #[test]
    fn base_buy_deriv_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        let start_price = Decimal::from_str("39300700000.000000000000000000").unwrap();
        let end_price = Decimal::from_str("39250700000.000000000000000000").unwrap();
        let total_margin_balance_for_new_orders = Decimal::from_str("40250700000").unwrap();
        let state = mock_state(leverage.to_string(), String::from("10"));
        let market = mock_market();
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::zero(),
            true,
            true,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            3,
            Decimal::zero(),
            true,
            true,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::from_str("0.15").unwrap(),
            true,
            true,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::from_str("0.55").unwrap(),
            true,
            true,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            3,
            Decimal::from_str("0.55").unwrap(),
            true,
            true,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            10,
            Decimal::from_str("0.55").unwrap(),
            true,
            true,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            3,
            Decimal::from_str("3").unwrap(),
            true,
            true,
            &state,
            &market,
        );
    }

    #[test]
    fn base_sell_deriv_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        let start_price = Decimal::from_str("39250700000.000000000000000000").unwrap();
        let end_price = Decimal::from_str("39300700000.000000000000000000").unwrap();
        let total_margin_balance_for_new_orders = Decimal::from_str("40250700000").unwrap();
        let state = mock_state(leverage.to_string(), String::from("10"));
        let market = mock_market();
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::zero(),
            true,
            false,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            3,
            Decimal::zero(),
            true,
            false,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::from_str("0.15").unwrap(),
            true,
            false,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::from_str("0.55").unwrap(),
            true,
            false,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            3,
            Decimal::from_str("0.55").unwrap(),
            true,
            false,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            10,
            Decimal::from_str("0.55").unwrap(),
            true,
            false,
            &state,
            &market,
        );
        create_orders_between_bounds_deriv_test(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            3,
            Decimal::from_str("3").unwrap(),
            true,
            false,
            &state,
            &market,
        );
    }

    // Test Helpers
    fn create_orders_between_bounds_deriv_test(
        start_price: Decimal,
        end_price: Decimal,
        total_margin_balance_for_new_orders: Decimal,
        num_orders_to_keep: usize,
        position_qty_to_reduce: Decimal,
        touch_head: bool,
        is_buy: bool,
        state: &State,
        market: &WrappedDerivativeMarket,
    ) {
        let (orders_to_open_from_base_case, new_position_qty_to_reduce, new_total_margin_balance_for_new_orders) = create_orders_between_bounds_deriv(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            num_orders_to_keep,
            position_qty_to_reduce,
            touch_head,
            is_buy,
            state,
            market,
        );

        for o in orders_to_open_from_base_case.iter() {
            println!("p: {} | q: {} | val: {} | m: {}", o.get_price(), o.get_qty(), o.get_val(), o.get_margin())
        }

        let mut total_val = Decimal::zero();
        let mut total_margin = Decimal::zero();
        let mut reduced_qty = Decimal::zero();
        for i in 0..orders_to_open_from_base_case.len() {
            if i > 0 {
                if is_buy {
                    assert!(orders_to_open_from_base_case[i].get_price() < orders_to_open_from_base_case[i - 1].get_price());
                } else {
                    assert!(orders_to_open_from_base_case[i].get_price() > orders_to_open_from_base_case[i - 1].get_price());
                }
            }
            if orders_to_open_from_base_case[i].is_reduce_only() {
                reduced_qty = reduced_qty + orders_to_open_from_base_case[i].get_qty();
            }
            total_val = total_val + orders_to_open_from_base_case[i].get_val();
            total_margin = total_margin + orders_to_open_from_base_case[i].get_margin();
        }

        println!(
            "val / lev: {} | total marg bal: {}",
            div_dec(total_val, state.leverage),
            total_margin_balance_for_new_orders
        );
        println!(
            "total marg: {} | total marg bal: {}",
            div_dec(total_val, state.leverage),
            total_margin_balance_for_new_orders
        );
        println!("actual ro qty: {} | qty we need to reduce: {}", reduced_qty, position_qty_to_reduce);

        assert!(div_dec(total_val, state.leverage) <= total_margin_balance_for_new_orders);
        assert!(total_margin <= total_margin_balance_for_new_orders);
        assert!(orders_to_open_from_base_case.len() + num_orders_to_keep <= state.order_density.to_string().parse::<usize>().unwrap());
        assert!(reduced_qty <= position_qty_to_reduce);
        assert!(position_qty_to_reduce - reduced_qty == new_position_qty_to_reduce);
        assert!(total_margin_balance_for_new_orders - total_margin == new_total_margin_balance_for_new_orders);
        println!("===========================================================")
    }

    #[test]
    fn reduce_buy_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        let start_price = Decimal::from_str("39300700000.000000000000000000").unwrap();
        let end_price = Decimal::from_str("39250700000.000000000000000000").unwrap();
        let state = mock_state(leverage.to_string(), String::from("10"));
        let market = mock_market();
        let total_margin_balance_for_new_orders = Decimal::from_str("40250700000").unwrap();

        let (orders_to_keep, _, _) = create_orders_between_bounds_deriv(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::zero(),
            true,
            true,
            &state,
            &market,
        );
        let orders_to_keep = wrap_orders(orders_to_keep);
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("0").unwrap(),
            true,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            market.min_quantity_tick_size,
            true,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("2.6").unwrap(),
            true,
            &state,
            &market,
        );

        let (orders_to_keep, _, _) = create_orders_between_bounds_deriv(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::from_str("1").unwrap(),
            true,
            true,
            &state,
            &market,
        );
        let orders_to_keep = wrap_orders(orders_to_keep);
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("0").unwrap(),
            true,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            market.min_quantity_tick_size,
            true,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("2.6").unwrap(),
            true,
            &state,
            &market,
        );
    }

    #[test]
    fn reduce_sell_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        let start_price = Decimal::from_str("39250700000.000000000000000000").unwrap();
        let end_price = Decimal::from_str("39300700000.000000000000000000").unwrap();
        let state = mock_state(leverage.to_string(), String::from("10"));
        let market = mock_market();
        let total_margin_balance_for_new_orders = Decimal::from_str("40250700000").unwrap();

        let (orders_to_keep, _, _) = create_orders_between_bounds_deriv(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::zero(),
            true,
            false,
            &state,
            &market,
        );
        let orders_to_keep = wrap_orders(orders_to_keep);
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("0").unwrap(),
            false,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            market.min_quantity_tick_size,
            false,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("2.6").unwrap(),
            false,
            &state,
            &market,
        );

        let (orders_to_keep, _, _) = create_orders_between_bounds_deriv(
            start_price,
            end_price,
            total_margin_balance_for_new_orders,
            0,
            Decimal::from_str("1").unwrap(),
            true,
            false,
            &state,
            &market,
        );
        let orders_to_keep = wrap_orders(orders_to_keep);
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("0").unwrap(),
            false,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            market.min_quantity_tick_size,
            false,
            &state,
            &market,
        );
        manage_reduce_only_deriv_test(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            Decimal::from_str("2.6").unwrap(),
            false,
            &state,
            &market,
        );
    }

    fn manage_reduce_only_deriv_test(
        orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
        total_margin_balance_for_new_orders: Decimal,
        position_qty_to_reduce: Decimal,
        is_buy: bool,
        state: &State,
        market: &WrappedDerivativeMarket,
    ) {
        println!("BEFORE");
        for o in orders_to_keep.iter() {
            println!(
                "hash: {} | p: {} | q: {} | m: {}",
                o.order_hash, o.order_info.price, o.order_info.quantity, o.margin
            )
        }
        let (
            additional_orders_to_cancel,
            orders_to_open_from_reduce_only_management,
            new_total_margin_balance_for_new_orders,
            new_position_qty_to_reduce,
        ) = manage_reduce_only_deriv(
            orders_to_keep.clone(),
            total_margin_balance_for_new_orders,
            position_qty_to_reduce,
            is_buy,
            state,
            market,
        );

        let mut gained_margin_from_cancels = Decimal::zero();
        for c in additional_orders_to_cancel.iter() {
            for k in orders_to_keep.iter() {
                if k.order_hash == c.order_hash {
                    gained_margin_from_cancels = gained_margin_from_cancels + k.margin;
                }
            }
        }
        let mut lost_margin_from_new = Decimal::zero();
        let mut total_val = Decimal::zero();
        let mut total_margin = Decimal::zero();
        let mut reduced_qty_after_this = Decimal::zero();
        let mut num_orders_after_this = 0;
        for o in orders_to_open_from_reduce_only_management.iter() {
            if o.is_reduce_only() {
                reduced_qty_after_this = reduced_qty_after_this + o.get_qty();
            } else {
                total_val = total_val + o.get_val();
            }
            total_margin = total_margin + o.get_margin();
            num_orders_after_this += 1;
            lost_margin_from_new = lost_margin_from_new + o.get_margin();
        }
        for o in orders_to_keep.iter() {
            let mut log = true;
            for c in additional_orders_to_cancel.iter() {
                if c.order_hash == o.order_hash {
                    log = false;
                }
            }
            if log {
                if o.is_reduce_only() {
                    reduced_qty_after_this = reduced_qty_after_this + o.order_info.quantity;
                } else {
                    total_val = total_val + (o.order_info.price * o.order_info.quantity);
                }
                total_margin = total_margin + o.margin;
                num_orders_after_this += 1;
            }
        }
        println!(
            "val / lev: {} | total marg bal: {}",
            div_dec(total_val, state.leverage),
            total_margin_balance_for_new_orders
        );
        println!(
            "total marg: {} | total marg bal: {}",
            div_dec(total_val, state.leverage),
            total_margin_balance_for_new_orders
        );
        println!(
            "actual ro qty: {} | qty we need to reduce: {}",
            reduced_qty_after_this, position_qty_to_reduce
        );
        println!(
            "expected margin rem: {} | actual margin rem: {}",
            new_total_margin_balance_for_new_orders,
            total_margin_balance_for_new_orders + gained_margin_from_cancels - lost_margin_from_new
        );

        assert!(div_dec(total_val, state.leverage) <= total_margin_balance_for_new_orders);
        assert!(total_margin <= total_margin_balance_for_new_orders);
        assert_eq!(num_orders_after_this, orders_to_keep.len());
        assert!(reduced_qty_after_this <= position_qty_to_reduce);
        assert_eq!(position_qty_to_reduce - reduced_qty_after_this, new_position_qty_to_reduce);
        assert_eq!(
            total_margin_balance_for_new_orders + gained_margin_from_cancels - lost_margin_from_new,
            new_total_margin_balance_for_new_orders
        );
        assert_eq!(additional_orders_to_cancel.len(), orders_to_open_from_reduce_only_management.len());

        // ================================================ Debug Logs ====================================== //
        println!("NEW");
        for o in orders_to_open_from_reduce_only_management.iter() {
            println!("p: {} | q: {} | m: {}", o.get_price(), o.get_qty(), o.get_margin())
        }
        println!("CANCELLED");
        for o in additional_orders_to_cancel.iter() {
            println!("hash: {}", o.order_hash);
        }
        println!("NEXT BLOCK");
        let mut hash = if additional_orders_to_cancel.len() > 0 {
            additional_orders_to_cancel.first().unwrap().order_hash.parse::<usize>().unwrap()
        } else {
            0
        };
        for o in orders_to_open_from_reduce_only_management.iter() {
            println!("newh: {} | p: {} | q: {} | m: {}", hash, o.get_price(), o.get_qty(), o.get_margin());
            hash += 1;
        }
        for o in orders_to_keep.iter() {
            let mut log = true;
            for c in additional_orders_to_cancel.iter() {
                if c.order_hash == o.order_hash {
                    log = false;
                }
            }
            if log {
                println!(
                    "hash: {} | p: {} | q: {} | m: {}",
                    o.order_hash, o.order_info.price, o.order_info.quantity, o.margin
                );
            }
        }
        println!("===========================================================")
    }

    fn mock_state(leverage: String, order_density: String) -> State {
        State {
            market_id: String::from(""),
            subaccount_id: String::from(""),
            order_density: Uint256::from_str(&order_density).unwrap(),
            max_active_capital_utilization_ratio: Decimal::from_str("0.2").unwrap(),
            min_head_to_tail_deviation_ratio: Decimal::from_str("0.03").unwrap(),
            mid_price_tail_deviation_ratio: Decimal::from_str("0.06").unwrap(),
            head_change_tolerance_ratio: Decimal::zero(),
            leverage: Decimal::from_str(&leverage).unwrap(),
            reservation_price_sensitivity_ratio: Decimal::zero(),
            reservation_spread_sensitivity_ratio: Decimal::zero(),
            fee_recipient: String::from(""),
            lp_token_address: None,
        }
    }

    fn mock_market() -> WrappedDerivativeMarket {
        DerivativeMarket {
            ticker: String::from(""),
            oracle_base: String::from(""),
            oracle_quote: String::from(""),
            oracle_type: 0,
            oracle_scale_factor: 0,
            quote_denom: String::from(""),
            market_id: String::from(""),
            initial_margin_ratio: String::from("0"),
            maintenance_margin_ratio: String::from("0"),
            maker_fee_rate: String::from("0"),
            taker_fee_rate: String::from("0"),
            isPerpetual: true,
            status: 0,
            min_price_tick_size: String::from("100000.000000000000000000"),
            min_quantity_tick_size: String::from("0.000100000000000000"),
        }
        .wrap()
        .unwrap()
    }

    fn wrap_orders(mocked_orders: Vec<DerivativeOrder>) -> Vec<WrappedDerivativeLimitOrder> {
        let mut orders: Vec<WrappedDerivativeLimitOrder> = Vec::new();
        let mut hash = 0;
        for o in mocked_orders.into_iter() {
            orders.push(WrappedDerivativeLimitOrder {
                trigger_price: None,
                order_info: WrappedOrderInfo {
                    subaccount_id: "".to_string(),
                    fee_recipient: "".to_string(),
                    price: o.get_price(),
                    quantity: o.get_qty(),
                },
                order_type: 0,
                margin: o.get_margin(),
                fillable: Decimal::zero(),
                order_hash: hash.to_string(),
            });
            hash += 1;
        }
        orders
    }
}
