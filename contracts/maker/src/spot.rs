use crate::{
    msg::WrappedOrderResponse,
    state::State,
    utils::{div_dec, div_int, sub_abs},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};
use std::str::FromStr;

/// Calculates the inventory imbalance from 50/50 target balance.
/// # Arguments
/// * `inv_base_val` - The notional value of all base assets
/// * `inv_val` - The total notional value of all assets
/// # Returns
/// * `inv_imbalance` - The inventory imbalance parameter
/// * `imbal_is_long` - True if the imbalance is skewed in favor of the base asset
pub fn inv_imbalance_spot(inv_base_val: Decimal, inv_val: Decimal) -> (Decimal, bool) {
    let half_inv_val = div_int(inv_val, Uint256::from_str("2").unwrap());
    let inv_imbalance = div_dec(sub_abs(inv_base_val, half_inv_val), inv_val);
    (inv_imbalance, inv_base_val > half_inv_val)
}

/// Determines the new orders that should be placed between the new head/tail. Ensures
/// that the notional value of all open orders will be equal to the allocated value
/// passed in as a parameter. The value of each order will be constant (close to constant)
/// accross each price step.
/// # Arguments
/// * `new_head` - The new head (closest to the reservation price)
/// * `new_tail` - The new tail (farthest from the reservation price)
/// * `alloc_val_for_new_orders` - The value that all the new orders should sum to
/// * `is_buy` - True if all open_orders are buy. False if they are all sell
/// * `state` - Contract state
/// # Returns
/// * `orders_to_open` - A list of all the new orders that we would like to place
pub fn create_new_orders_spot(
    new_head: Decimal,
    new_tail: Decimal,
    alloc_val_for_new_orders: Decimal,
    is_buy: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let val_per_order = alloc_val_for_new_orders / state.order_density;
    let price_dist = sub_abs(new_head, new_tail);
    let price_step = div_int(price_dist, state.order_density);
    let num_orders_to_open = state.order_density.to_string().parse::<i32>().unwrap();
    let mut current_price = new_head;
    for _ in 0..num_orders_to_open {
        let qty = div_dec(val_per_order, current_price);
        let new_order = WrappedOrderResponse::new(state, current_price, qty, is_buy, false);
        orders_to_open.push(new_order);
        current_price = if is_buy {
            current_price - price_step
        } else {
            current_price + price_step
        }
    }
    orders_to_open
}

#[cfg(test)]
mod tests {
    use crate::{spot::create_new_orders_spot, state::State, utils::div_dec};
    use cosmwasm_std::{Decimal256 as Decimal, Uint256};
    use std::str::FromStr;

    use super::inv_imbalance_spot;
    #[test]
    fn inv_imbalance_test() {
        let inv_bal = Decimal::from_str("100000").unwrap();
        let base_balanced_bal = Decimal::from_str("50000").unwrap();
        let (inv_imbal, _) = inv_imbalance_spot(base_balanced_bal, inv_bal );
        assert_eq!(inv_imbal, Decimal::zero());

        let base_imbal_long_bal = Decimal::from_str("50001").unwrap();
        let (inv_imbal, imbal_is_long) = inv_imbalance_spot(base_imbal_long_bal, inv_bal );
        assert!(inv_imbal > Decimal::zero());
        assert!(imbal_is_long);

        let base_imbal_short_bal = Decimal::from_str("49999").unwrap();
        let (inv_imbal, imbal_is_long) = inv_imbalance_spot(base_imbal_short_bal, inv_bal );
        assert!(inv_imbal > Decimal::zero());
        assert!(!imbal_is_long);
    }

    #[test]
    fn create_buy_orders_test() {
        for i in 2..10 {
            let decimal_base_shift = 10_i128.pow(i);
            let state = mock_state(String::from("1"), String::from("10"), decimal_base_shift.to_string());
            let head_price = Decimal::from_str(&10_i32.pow(i).to_string()).unwrap();
            let tail_price = head_price * (Decimal::one() - div_dec(state.min_tail_dist_bp, Decimal::from_str("10000").unwrap()));
            for j in 3..10 {
                let alloc_value = 10_i32.pow(j);
                create_new_orders_spot_test(head_price, tail_price, Decimal::from_str(&alloc_value.to_string()).unwrap(), true, &state);
            }
        }
    }

    #[test]
    fn create_sell_orders_test() {
        for i in 2..10 {
            let decimal_base_shift = 10_i128.pow(i);
            let state = mock_state(String::from("1"), String::from("10"), decimal_base_shift.to_string());
            let head_price = Decimal::from_str(&10_i32.pow(i).to_string()).unwrap();
            let tail_price = head_price * (Decimal::one() + div_dec(state.min_tail_dist_bp, Decimal::from_str("10000").unwrap()));
            for j in 3..10 {
                let alloc_value = 10_i32.pow(j);
                create_new_orders_spot_test(
                    head_price,
                    tail_price,
                    Decimal::from_str(&alloc_value.to_string()).unwrap(),
                    false,
                    &state,
                );
            }
        }
    }

    fn create_new_orders_spot_test(new_head: Decimal, new_tail: Decimal, alloc_val_for_new_orders: Decimal, is_buy: bool, state: &State) {
        let max_tolerance = Decimal::from_str("0.01").unwrap();
        let new_orders = create_new_orders_spot(new_head, new_tail, alloc_val_for_new_orders, is_buy, state);
        let val_per_order = alloc_val_for_new_orders / state.order_density;
        let mut total_value = Decimal::zero();
        for i in 0..new_orders.len() {
            total_value = total_value + new_orders[i].get_val();
            if i > 0 {
                // Ensure that price is changing in the right direction
                if is_buy {
                    assert!(new_orders[i - 1].get_price() > new_orders[i].get_price());
                } else {
                    assert!(new_orders[i - 1].get_price() < new_orders[i].get_price());
                }
            }
            // Ensure that the notional val of orders is consistent
            assert!(new_orders[i].get_val() + (max_tolerance * val_per_order) >= val_per_order);
            assert!(new_orders[i].get_val() - (max_tolerance * val_per_order) <= val_per_order);
        }

        // Ensure we never have too many orders
        assert_eq!(new_orders.len(), state.order_density.to_string().parse::<usize>().unwrap());

        // Ensure that we used all possible inventory within a tolerance
        assert!(total_value + (alloc_val_for_new_orders * max_tolerance) >= alloc_val_for_new_orders);
        assert!(total_value - (alloc_val_for_new_orders * max_tolerance) <= alloc_val_for_new_orders);

        // Ensure the first order has the head price
        assert_eq!(new_orders.first().unwrap().get_price(), new_head);
    }

    fn mock_state(leverage: String, order_density: String, base_precision_shift: String) -> State {
        State {
            market_id: String::from(""),
            manager: String::from(""),
            fee_recipient: String::from(""),
            sub_account: String::from(""),
            risk_aversion: Decimal::from_str("0.2").unwrap(),
            order_density: Uint256::from_str(&order_density).unwrap(),
            active_capital_perct: Decimal::from_str("0.2").unwrap(),
            manual_offset_perct: Decimal::zero(),
            min_tail_dist_bp: Decimal::from_str("300").unwrap(),
            tail_dist_from_mid_bp: Decimal::from_str("800").unwrap(),
            head_chg_tol_bp: Decimal::zero(),
            leverage: Decimal::from_str(&leverage).unwrap(),
            decimal_shift: Uint256::from_str("1000000").unwrap(),
            base_precision_shift: Uint256::from_str(&base_precision_shift.to_string()).unwrap(),
        }
    }
}
