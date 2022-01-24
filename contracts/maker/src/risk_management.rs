use std::str::FromStr;

use cosmwasm_std::Decimal256 as Decimal;

use crate::utils::{div_dec, sub_no_overflow};

/// Determines the notional balance that we are willing to assign to either the buy/sell side.
/// Takes into consideration the current margin to limit the new open orders on the side
/// that already has a positon open. If the current margin is greater than a certain percentage
/// of inventory value, that side will stop placing orders.
/// # Arguments
/// * `inv_val` - The total notional value of the inventory
/// * `margin` - The margin value of an open position (is zero if the position is on the opposite side or if there isn't one)
/// * `active_capital_perct` - The factor by which we multiply the inventory val to get total capital that should be on the book
/// * `max_notional_position_perct` - The threshold after which we will stop placing orders on the same side as an open position
/// # Returns
/// * `alloc_bal` - The notional balance we are willing to allocate to one side
pub fn get_alloc_bal_new_orders(inv_val: Decimal, margin: Decimal, active_capital_perct: Decimal, max_notional_position_perct: Decimal) -> Decimal {
    let alloc_for_both_sides = inv_val * active_capital_perct;
    let alloc_one_side = div_dec(alloc_for_both_sides, Decimal::from_str("2").unwrap());
    if div_dec(margin, inv_val) >= max_notional_position_perct {
        Decimal::zero()
    } else {
        if margin == Decimal::zero() {
            alloc_one_side
        } else {
            let inv_val = sub_no_overflow(inv_val, alloc_one_side);
            let inv_val = sub_no_overflow(inv_val, margin);
            let alloc_for_both_sides = inv_val * active_capital_perct;
            div_dec(alloc_for_both_sides, Decimal::from_str("2").unwrap())
        }
    }
}