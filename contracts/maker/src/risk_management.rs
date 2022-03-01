use crate::{
    exchange::{WrappedPosition, DerivativeOrder,  WrappedDerivativeMarket},
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
    // assert!(inv_base_bal == Decimal::zero());
    // assert!(position.is_none());
    //TODO: come back to this one
}

/// Determines the total margin that we are allowed to allocate to the new orders. It is influenced by the capital utilization ratio, the
/// current margined balance of orders we decided to keep on the book, and same sided position margin.
/// that already has a position open.
/// # Arguments
/// * `total_deposit_balance` - The total quote balance LPed
/// * `position_is_same_side` - True if the side is the same as the position
/// * `position_margin` - The margin value of a position taken. Is zero if no position is taken
/// * `max_active_capital_utilization_ratio` - A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book
/// * `agg_margin_of_orders_kept` - The total aggregated margined value of the orders we would like to keep
/// # Returns
/// * `total_margin_balance_for_new_orders` - The total margin that we are allowed to allocate to the new orders
pub fn total_margin_balance_for_new_orders(
    total_deposit_balance: Decimal,
    position_is_same_side: bool,
    position_margin: Decimal,
    max_active_capital_utilization_ratio: Decimal,
    agg_margin_of_orders_kept: Decimal,
) -> Decimal {
    let total_margin_balance_for_both_sides = total_deposit_balance * max_active_capital_utilization_ratio;
    let total_margin_balance_for_one_side = div_dec(total_margin_balance_for_both_sides, Decimal::from_str("2").unwrap());
    let total_margin_balance_for_new_orders = sub_no_overflow(total_margin_balance_for_one_side, agg_margin_of_orders_kept);
    if position_is_same_side {
        sub_no_overflow(total_margin_balance_for_new_orders, position_margin)
    } else {
        total_margin_balance_for_new_orders
    }
}

/// Ensures that the current tails have enough distance between them. We don't want our order spread to be too dense.
/// If they fall below the minimum distance, we update the tail to something more suitable.
/// # Arguments
/// * `new_buy_head` - The buy head that we are going to use
/// * `new_sell_head` - The the sell head that we are going to use
/// * `proposed_buy_tail` - The buyside tail obtained from the spread around the mid price
/// * `proposed_sell_tail` - The sellside tail obtained from the spread around the mid price
/// * `min_head_to_tail_deviation_ratio` - A constant between 0..1 that ensures our tail is at least some distance from the head
/// # Returns
/// * `new_buy_tail` - The new buyside tail post risk management
/// * `new_sell_tail` - The new sellside tail post risk management
pub fn check_tail_dist(
    new_buy_head: Decimal,
    new_sell_head: Decimal,
    proposed_buy_tail: Decimal,
    proposed_sell_tail: Decimal,
    min_head_to_tail_deviation_ratio: Decimal,
) -> (Decimal, Decimal) {
    let new_buy_tail =  {
        let proposed_buy_tail_dist = div_dec(sub_abs(new_buy_head, proposed_buy_tail), new_buy_head);
        if proposed_buy_tail_dist < min_head_to_tail_deviation_ratio || proposed_buy_tail >= new_buy_head{
            new_buy_head * sub_abs(Decimal::one(), min_head_to_tail_deviation_ratio)
        } else {
            proposed_buy_tail
        }
    };

    let new_sell_tail = {
        let proposed_sell_tail_dist = div_dec(sub_abs(new_sell_head, proposed_sell_tail), new_sell_head);
        if proposed_sell_tail_dist < min_head_to_tail_deviation_ratio || proposed_sell_tail <= new_sell_head {
            new_sell_head * (Decimal::one() + min_head_to_tail_deviation_ratio)
        } else {
            proposed_sell_tail
        }
    };

    (new_buy_tail, new_sell_tail)
}

/// Filters out any orders that dont comply with the exchange standards.
/// # Arguments
/// * `orders_to_place` - All the orders that we are trying to create
/// * `market` - Derivative market information
/// # Returns
/// * `filtered_orders_to_place` - The filtered orders
pub fn final_check(orders_to_place: Vec<DerivativeOrder>, market: &WrappedDerivativeMarket) -> Vec<DerivativeOrder> {
    orders_to_place.into_iter()
    .filter(|order| {
        Decimal::from_str(&order.order_info.quantity).unwrap().gt(&market.min_quantity_tick_size)
            && Decimal::from_str(&order.order_info.price).unwrap().gt(&market.min_price_tick_size)
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{check_tail_dist};
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;

    // #[test]
    // fn get_alloc_bal_new_orders_test() {
        // let inv_val = Decimal::from_str("1000000000").unwrap();
        // let max_active_capital_utilization_ratio = Decimal::from_str("1").unwrap();
        // let margin = Decimal::zero();

        // let alloc_bal_a = get_alloc_bal_new_orders(inv_val, true, margin, max_active_capital_utilization_ratio, Decimal::zero(), Decimal::zero());
        // let alloc_bal_b = get_alloc_bal_new_orders(inv_val, false, margin, max_active_capital_utilization_ratio, Decimal::zero(), Decimal::zero());
        // assert_eq!(alloc_bal_a, alloc_bal_b);
        // assert_eq!(alloc_bal_a, Decimal::from_str("0.5").unwrap() * inv_val);
        // println!("{} {}", alloc_bal_a, alloc_bal_b);

        // let margin = Decimal::from_str("1000000000").unwrap();

        // let alloc_bal_a = get_alloc_bal_new_orders(inv_val, true, margin, active_capital);
        // let alloc_bal_b = get_alloc_bal_new_orders(inv_val, false, margin, active_capital);
        // println!("{} {}", alloc_bal_a, alloc_bal_b);
        // // assert_eq!(alloc_bal_a + margin, alloc_bal_b);

        // let inv_val = Decimal::from_str("0").unwrap();
        // let margin = Decimal::from_str("10000").unwrap();

        // let alloc_bal_a = get_alloc_bal_new_orders(inv_val, true, margin, active_capital);
        // let alloc_bal_b = get_alloc_bal_new_orders(inv_val, false, margin, active_capital);
        // println!("{} {}", alloc_bal_a, alloc_bal_b);
        // assert_eq!(Decimal::zero(), alloc_bal_a);
        // assert_eq!(Decimal::zero(), alloc_bal_b);
    // }

    #[test]
    fn check_tail_dist_test() {
        let buy_head = Decimal::from_str("3999").unwrap();
        let sell_head = Decimal::from_str("4001").unwrap();
        let proposed_buy_tail = Decimal::from_str("2000").unwrap();
        let proposed_sell_tail = Decimal::from_str("7000").unwrap();
        let min_head_to_tail_deviation_ratio = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = check_tail_dist(buy_head, sell_head, proposed_buy_tail, proposed_sell_tail, min_head_to_tail_deviation_ratio);
        assert_eq!(buy_tail, proposed_buy_tail);
        assert_eq!(sell_tail, proposed_sell_tail);

        let proposed_buy_tail = Decimal::from_str("3998").unwrap();
        let proposed_sell_tail = Decimal::from_str("4002").unwrap();
        let (buy_tail, sell_tail) = check_tail_dist(buy_head, sell_head, proposed_buy_tail, proposed_sell_tail, min_head_to_tail_deviation_ratio);
        assert_eq!(buy_tail, buy_head * (Decimal::one() - min_head_to_tail_deviation_ratio));
        assert_eq!(sell_tail, sell_head * (Decimal::one() + min_head_to_tail_deviation_ratio));
    }
}
