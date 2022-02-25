use crate::{
    exchange::WrappedPosition,
    state::State,
    utils::{div_dec, sub_abs, sub_no_overflow},
};
use cosmwasm_std::{Addr, Decimal256 as Decimal};
use std::str::FromStr;

pub fn only_owner(sender: &Addr, owner: &Addr) {
    assert_eq!(sender, owner);
}

// TODO: add more
pub fn sanity_check(_position: &Option<WrappedPosition>, _inv_base_ball: Decimal, _state: &State) {
    // assert!(state.is_deriv && inv_base_bal == Decimal::zero());
    // assert!(!state.is_deriv || position.is_none());
    //TODO: come back to this one
}

/// Determines the notional balance that we are willing to assign to either the buy/sell side.
/// Takes into consideration the current margin to limit the new open orders on the side
/// that already has a position open.
/// # Arguments
/// * `inv_val` - The total notional value of the inventory
/// * `margin` - The margin value of an open position
/// * `is_same_side` - True if the side is the same as the position
/// * `active_capital` - The factor by which we multiply the inventory val to get total capital that should be on the book (between 0..1)
/// # Returns
/// * `alloc_bal` - The notional balance we are willing to allocate to one side
pub fn get_alloc_bal_new_orders(inv_val: Decimal, is_same_side: bool, margin: Decimal, active_capital: Decimal) -> Decimal {
    let alloc_for_both_sides = inv_val * active_capital;
    let alloc_one_side = div_dec(alloc_for_both_sides, Decimal::from_str("2").unwrap());
    if is_same_side {
        sub_no_overflow(alloc_one_side, margin)
    } else {
        alloc_one_side + margin
    }
}

/// Ensures that the current tails have enough distance between them. We don't want our order spread to be too dense.
/// If they fall below the minimum distance, we update the tail to something more suitable.
/// # Arguments
/// * `buy_head` - The buy head that we are going to use
/// * `sell_head` - The the sell head that we are going to use
/// * `proposed_buy_tail` - The buyside tail obtained from the mid price
/// * `proposed_sell_tail` - The sellside tail obtained from the mid price
/// * `min_tail_dist` - The minimum distance in from the head that we are willing to tolerate (between 0..1)
/// # Returns
/// * `buy_tail` - The new buyside tail post risk management
/// * `sell_tail` - The new sellside tail post risk management
pub fn check_tail_dist(
    buy_head: Decimal,
    sell_head: Decimal,
    proposed_buy_tail: Decimal,
    proposed_sell_tail: Decimal,
    min_tail_dist: Decimal,
) -> (Decimal, Decimal) {
    let buy_tail = if buy_head > proposed_buy_tail {
        let proposed_buy_tail_dist = div_dec(sub_abs(buy_head, proposed_buy_tail), buy_head);
        if proposed_buy_tail_dist < min_tail_dist {
            buy_head * sub_abs(Decimal::one(), min_tail_dist)
        } else {
            proposed_buy_tail
        }
    } else {
        proposed_buy_tail
    };

    let sell_tail = if sell_head < proposed_sell_tail {
        let proposed_sell_tail_dist = div_dec(sub_abs(sell_head, proposed_sell_tail), sell_head);
        if proposed_sell_tail_dist < min_tail_dist {
            sell_head * (Decimal::one() + min_tail_dist)
        } else {
            proposed_sell_tail
        }
    } else {
        proposed_sell_tail
    };

    (buy_tail, sell_tail)
}

#[cfg(test)]
mod tests {
    use super::{check_tail_dist, get_alloc_bal_new_orders};
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;

    #[test]
    fn get_alloc_bal_new_orders_test() {
        let inv_val = Decimal::from_str("1000000000").unwrap();
        let active_capital = Decimal::from_str("0.2").unwrap();
        let margin = Decimal::zero();

        let alloc_bal_a = get_alloc_bal_new_orders(inv_val, true, margin, active_capital);
        let alloc_bal_b = get_alloc_bal_new_orders(inv_val, false, margin, active_capital);
        assert_eq!(alloc_bal_a, alloc_bal_b);
        assert_eq!(alloc_bal_a, Decimal::from_str("0.1").unwrap() * inv_val);
        println!("{} {}", alloc_bal_a, alloc_bal_b);

        let margin = Decimal::from_str("10000").unwrap();

        let alloc_bal_a = get_alloc_bal_new_orders(inv_val, true, margin, active_capital);
        let alloc_bal_b = get_alloc_bal_new_orders(inv_val, false, margin, active_capital);
        println!("{} {}", alloc_bal_a, alloc_bal_b);
        assert_eq!(alloc_bal_a + margin, alloc_bal_b - margin);

        let inv_val = Decimal::from_str("0").unwrap();
        let margin = Decimal::from_str("10000").unwrap();

        let alloc_bal_a = get_alloc_bal_new_orders(inv_val, true, margin, active_capital);
        let alloc_bal_b = get_alloc_bal_new_orders(inv_val, false, margin, active_capital);
        println!("{} {}", alloc_bal_a, alloc_bal_b);
        assert_eq!(Decimal::zero(), alloc_bal_a);
        assert_eq!(margin, alloc_bal_b);
    }

    #[test]
    fn check_tail_dist_test() {
        let buy_head = Decimal::from_str("3999").unwrap();
        let sell_head = Decimal::from_str("4001").unwrap();
        let proposed_buy_tail = Decimal::from_str("2000").unwrap();
        let proposed_sell_tail = Decimal::from_str("7000").unwrap();
        let min_tail_dist = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = check_tail_dist(buy_head, sell_head, proposed_buy_tail, proposed_sell_tail, min_tail_dist);
        assert_eq!(buy_tail, proposed_buy_tail);
        assert_eq!(sell_tail, proposed_sell_tail);

        let proposed_buy_tail = Decimal::from_str("3998").unwrap();
        let proposed_sell_tail = Decimal::from_str("4002").unwrap();
        let (buy_tail, sell_tail) = check_tail_dist(buy_head, sell_head, proposed_buy_tail, proposed_sell_tail, min_tail_dist);
        assert_eq!(buy_tail, buy_head * (Decimal::one() - min_tail_dist));
        assert_eq!(sell_tail, sell_head * (Decimal::one() + min_tail_dist));
    }
}
