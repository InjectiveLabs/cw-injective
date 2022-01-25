use crate::utils::{div_dec, sub_abs, sub_no_overflow};
use cosmwasm_std::Decimal256 as Decimal;
use std::str::FromStr;

/// Determines the notional balance that we are willing to assign to either the buy/sell side.
/// Takes into consideration the current margin to limit the new open orders on the side
/// that already has a positon open.
/// # Arguments
/// * `inv_val` - The total notional value of the inventory
/// * `margin` - The margin value of an open position (is zero if the position is on the opposite side or if there isn't one)
/// * `active_capital_perct` - The factor by which we multiply the inventory val to get total capital that should be on the book
/// # Returns
/// * `alloc_bal` - The notional balance we are willing to allocate to one side
pub fn get_alloc_bal_new_orders(inv_val: Decimal, margin: Decimal, active_capital_perct: Decimal) -> Decimal {
    let alloc_for_both_sides = inv_val * active_capital_perct;
    let alloc_one_side = div_dec(alloc_for_both_sides, Decimal::from_str("2").unwrap());

    if margin == Decimal::zero() {
        alloc_one_side
    } else {
        let inv_val = sub_no_overflow(inv_val, alloc_one_side);
        let inv_val = sub_no_overflow(inv_val, margin);
        let alloc_for_both_sides = inv_val * active_capital_perct;
        div_dec(alloc_for_both_sides, Decimal::from_str("2").unwrap())
    }
}

/// Ensures that the current tails have enough distance between them. We don't want our order spread to be too dense.
/// If they fall below the minimum distance, we update the tail to something more suitable.
/// # Arguments
/// * `buy_head` - The buy head that we are going to use
/// * `sell_head` - The the sell head that we are going to use
/// * `proposed_buy_tail` - The buyside tail obtained from the mid price
/// * `proposed_sell_tail` - The sellside tail obtained from the mid price
/// * `min_tail_dist_bp` - The minimum distance in BP from the head that we are willing to tolerate
/// # Returns
/// * `buy_tail` - The new buyside tail post risk management
/// * `sell_tail` - The new sellside tail post risk management
pub fn check_tail_dist(
    buy_head: Decimal,
    sell_head: Decimal,
    proposed_buy_tail: Decimal,
    proposed_sell_tail: Decimal,
    min_tail_dist_bp: Decimal,
) -> (Decimal, Decimal) {
    let buy_tail = if buy_head > proposed_buy_tail {
        let min_buy_tail_dist = div_dec(sub_abs(buy_head, proposed_buy_tail), buy_head);
        if min_buy_tail_dist * Decimal::from_str("10000").unwrap() < min_tail_dist_bp {
            min_tail_dist_bp * sub_abs(Decimal::one(), min_buy_tail_dist)
        } else {
            proposed_buy_tail
        }
    } else {
        proposed_buy_tail
    };

    let sell_tail = if sell_head < proposed_buy_tail {
        let min_sell_tail_dist = div_dec(sub_abs(sell_head, proposed_sell_tail), sell_head);
        if min_sell_tail_dist * Decimal::from_str("10000").unwrap() < min_tail_dist_bp {
            min_tail_dist_bp * (Decimal::one() + min_sell_tail_dist)
        } else {
            proposed_sell_tail
        }
    } else {
        proposed_sell_tail
    };

    (buy_tail, sell_tail)
}
