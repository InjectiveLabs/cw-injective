use crate::{
    msg::{WrappedOrderResponse, WrappedPosition},
    state::State,
    utils::{div_dec, div_int, sub_abs},
};
use cosmwasm_std::Decimal256 as Decimal;

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
/// accross each price step. If there is a position open on the opposite side, it will place
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
pub fn create_new_orders_deriv(
    new_head: Decimal,
    new_tail: Decimal,
    alloc_val_for_new_orders: Decimal,
    mut position_qty: Decimal,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Decimal) {
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let val_per_order = alloc_val_for_new_orders / state.order_density;
    let val_per_order = val_per_order * state.leverage;
    let price_dist = sub_abs(new_head, new_tail);
    let price_step = div_int(price_dist, state.order_density);
    let num_orders_to_open = state.order_density.to_string().parse::<i32>().unwrap();
    let mut current_price = new_head;
    for _ in 0..num_orders_to_open {
        let qty = div_dec(val_per_order, current_price);
        if position_qty == Decimal::zero() {
            // If there is no position qty, no need to make reduce only orders
            let new_order = WrappedOrderResponse::new(state, current_price, qty, is_buy, false);
            orders_to_open.push(new_order);
        } else {
            // We need to manage reduce only orders here
            if qty > position_qty {
                // We need to make two orders here, one reduce only and one for the remainder
                let new_order_reduce = WrappedOrderResponse::new(state, current_price, position_qty, is_buy, true);
                let new_order = WrappedOrderResponse::new(state, current_price, qty - position_qty, is_buy, false);
                orders_to_open.push(new_order_reduce);
                orders_to_open.push(new_order);
                position_qty = Decimal::zero();
            } else {
                // This whole order should be reduce only
                let new_order_reduce = WrappedOrderResponse::new(state, current_price, qty, is_buy, true);
                position_qty = position_qty - new_order_reduce.get_qty();
                orders_to_open.push(new_order_reduce);
            }
        }
        current_price = if is_buy {
            current_price - price_step
        } else {
            current_price + price_step
        }
    }
    (orders_to_open, position_qty)
}

#[cfg(test)]
mod tests {
    use crate::{derivative::create_new_orders_deriv, state::State, utils::div_dec};
    use cosmwasm_std::{Addr, Decimal256 as Decimal, Uint256};
    use std::str::FromStr;

    #[test]
    fn create_new_buy_orders_deriv_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        for i in 2..10 {
            let decimal_base_shift = 10_i128.pow(i);
            let state = mock_state(leverage.to_string(), String::from("10"), decimal_base_shift.to_string());
            let head_price = Decimal::from_str(&10_i32.pow(i).to_string()).unwrap();
            let tail_price = head_price * Decimal::one() - state.min_tail_dist;
            for j in 3..10 {
                let alloc_value = Decimal::from_str(&10_i32.pow(j).to_string()).unwrap();
                create_new_orders_deriv_test(head_price, tail_price, alloc_value, Decimal::zero(), true, &state);
                for p in 1..20 {
                    create_new_orders_deriv_test(
                        head_price,
                        tail_price,
                        alloc_value,
                        div_dec(
                            div_dec(alloc_value * state.leverage, head_price),
                            Decimal::from_str(&p.to_string()).unwrap(),
                        ),
                        true,
                        &state,
                    );
                }
            }
        }
    }

    #[test]
    fn create_new_sell_orders_deriv_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        for i in 2..10 {
            let decimal_base_shift = 10_i128.pow(i);
            let state = mock_state(leverage.to_string(), String::from("10"), decimal_base_shift.to_string());
            let head_price = Decimal::from_str(&10_i32.pow(i).to_string()).unwrap();
            let tail_price = head_price * (Decimal::one() + state.min_tail_dist);
            for j in 3..10 {
                let alloc_value = Decimal::from_str(&10_i32.pow(j).to_string()).unwrap();
                create_new_orders_deriv_test(
                    head_price,
                    Decimal::from_str(&tail_price.to_string()).unwrap(),
                    alloc_value,
                    Decimal::zero(),
                    false,
                    &state,
                );
                for p in 1..20 {
                    create_new_orders_deriv_test(
                        head_price,
                        Decimal::from_str(&tail_price.to_string()).unwrap(),
                        alloc_value,
                        div_dec(
                            div_dec(alloc_value * state.leverage, head_price),
                            Decimal::from_str(&p.to_string()).unwrap(),
                        ),
                        false,
                        &state,
                    );
                }
            }
        }
    }

    // Test Helpers
    fn create_new_orders_deriv_test(
        new_head: Decimal,
        new_tail: Decimal,
        alloc_val_for_new_orders: Decimal,
        position_qty: Decimal,
        is_buy: bool,
        state: &State,
    ) {
        let max_tolerance = Decimal::from_str("0.01").unwrap();
        let (new_orders, rem_position_qty) = create_new_orders_deriv(new_head, new_tail, alloc_val_for_new_orders, position_qty, is_buy, state);
        let val_per_order = alloc_val_for_new_orders / state.order_density;
        let val_per_order = val_per_order * state.leverage;
        let mut total_reduce_only_qty = Decimal::zero();
        let mut total_value = Decimal::zero();
        let mut num_same_priced_orders = 0;
        for i in 0..new_orders.len() {
            if new_orders[i].is_reduce_only {
                total_reduce_only_qty = total_reduce_only_qty + new_orders[i].get_qty();
            }
            total_value = total_value + new_orders[i].get_val();
            if i > 0 {
                // Ensure that price is changing in the right direction
                if !(new_orders[i - 1].is_reduce_only && !new_orders[i].is_reduce_only) {
                    if is_buy {
                        assert!(new_orders[i - 1].get_price() > new_orders[i].get_price());
                    } else {
                        assert!(new_orders[i - 1].get_price() < new_orders[i].get_price());
                    }
                }
                // Ensure that the notional val of orders is consistent
                let value = if new_orders[i - 1].is_reduce_only && !new_orders[i].is_reduce_only {
                    new_orders[i - 1].get_val() + new_orders[i].get_val()
                } else if new_orders[i - 1].is_reduce_only {
                    new_orders[i - 1].get_val()
                } else {
                    new_orders[i].get_val()
                };
                if new_orders[i - 1].get_price() == new_orders[i].get_price() {
                    num_same_priced_orders += 1;
                }
                assert!(value * (max_tolerance * val_per_order) >= val_per_order);
            }
        }

        // Ensure we never have too many orders
        assert_eq!(
            new_orders.len() - num_same_priced_orders,
            state.order_density.to_string().parse::<usize>().unwrap()
        );

        // Esure that we tried the best we could to reduce the position
        assert!(position_qty >= total_reduce_only_qty);
        if rem_position_qty == Decimal::zero() {
            assert!((position_qty * max_tolerance) >= position_qty - total_reduce_only_qty);
        } else {
            for i in 0..new_orders.len() {
                assert!(new_orders[i].is_reduce_only);
            }
        }

        // Ensure that we used all possible inventory within a tolerance
        assert!(div_dec(total_value, state.leverage) + (alloc_val_for_new_orders * max_tolerance) >= alloc_val_for_new_orders);
        assert!(div_dec(total_value, state.leverage) - (alloc_val_for_new_orders * max_tolerance) <= alloc_val_for_new_orders);
    }

    fn mock_state(leverage: String, order_density: String, base_precision_shift: String) -> State {
        State {
            market_id: String::from(""),
            is_deriv: true,
            sub_account: String::from(""),
            order_density: Uint256::from_str(&order_density).unwrap(),
            active_capital: Decimal::from_str("0.2").unwrap(),
            min_tail_dist: Decimal::from_str("0.03").unwrap(),
            tail_dist_from_mid: Decimal::from_str("0.06").unwrap(),
            head_chg_tol: Decimal::zero(),
            leverage: Decimal::from_str(&leverage).unwrap(),
            decimal_shift: Uint256::from_str("1000000").unwrap(),
            base_precision_shift: Uint256::from_str(&base_precision_shift.to_string()).unwrap(),
            mid_price: Decimal::zero(),
            volitility: Decimal::zero(),
            reservation_param: Decimal::zero(),
            spread_param: Decimal::zero(),
            manager: Addr::unchecked(""),
            last_update_utc: 0,
            max_market_data_delay: 0,
        }
    }
}
