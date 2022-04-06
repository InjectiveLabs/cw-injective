use crate::{
    exchange::{DerivativeMarket, DerivativeOrder, EffectivePosition},
    state::State,
    utils::{div_dec, max, min, sub_abs, sub_no_overflow},
};
use cosmwasm_std::{Addr, CosmosMsg, Decimal256 as Decimal};
use injective_bindings::{create_derivative_market_order_msg, InjectiveMsgWrapper};
use std::str::FromStr;

pub fn only_owner(sender: &Addr, owner: &Addr) {
    assert_eq!(sender, owner);
}

// TODO: add more
pub fn sanity_check(_position: &Option<EffectivePosition>, _inv_base_ball: Decimal, _state: &State) {
    // assert!(inv_base_bal == Decimal::zero());
    // assert!(position.is_none());
    //TODO: come back to this one
}

/// # Description
/// Determines the total margin that we are allowed to allocate to the new orders. It is influenced by the capital utilization ratio, the
/// current margined balance of orders we decided to keep on the book, and same sided position margin.
/// # Arguments
/// * `total_deposit_balance` - The total quote balance LPed
/// * `position_is_same_side` - True if the side is the same as the position
/// * `position_margin` - The margin value of a position taken. Is zero if no position is taken
/// * `max_active_capital_utilization_ratio` - A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book
/// * `agg_margin_of_orders_kept` - The total aggregated margined value of the orders we would like to keep
/// # Returns
/// * `total_marginable_balance_for_new_orders` - The total margin that we are allowed to allocate to the new orders
pub fn total_marginable_balance_for_new_orders(
    total_deposit_balance: Decimal,
    position_is_same_side: bool,
    position_margin: Decimal,
    max_active_capital_utilization_ratio: Decimal,
    agg_margin_of_orders_kept: Decimal,
) -> Decimal {
    let total_margin_balance_for_both_sides = total_deposit_balance * max_active_capital_utilization_ratio;
    let total_margin_balance_for_one_side = div_dec(total_margin_balance_for_both_sides, Decimal::from_str("2").unwrap());
    let total_marginable_balance_for_new_orders = sub_no_overflow(total_margin_balance_for_one_side, agg_margin_of_orders_kept);
    if position_is_same_side {
        sub_no_overflow(total_marginable_balance_for_new_orders, position_margin)
    } else {
        total_marginable_balance_for_new_orders
    }
}

/// # Description
/// Ensures that the current tails have enough distance between them. We don't want our order spread to be too dense.
/// If they fall below the minimum distance, we update the tail to something more suitable.
/// # Formulas
/// * `max buy tail` = buy head * (1 - min head to tail deviation ratio)
/// * `min sell tail` = sell head * (1 + min head to tail deviation ratio)
/// * `new buy tail` = min(max buy tail, proposed buy tail)
/// * `new sell tail` = max(min sell tail, proposed sell tail)
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
    let max_buy_tail = new_buy_head * sub_abs(Decimal::one(), min_head_to_tail_deviation_ratio);
    let new_buy_tail = min(max_buy_tail, proposed_buy_tail);

    let min_sell_tail = new_sell_head * (Decimal::one() + min_head_to_tail_deviation_ratio);
    let new_sell_tail = max(min_sell_tail, proposed_sell_tail);

    (new_buy_tail, new_sell_tail)
}

/// # Description
/// Filters out any orders that don't comply with the exchange standards.
/// # Arguments
/// * `orders_to_place` - All the orders that we are trying to create
/// * `market` - Derivative market information
/// # Returns
/// * `filtered_orders_to_place` - The filtered orders
pub fn final_check(orders_to_place: Vec<DerivativeOrder>, market: &DerivativeMarket) -> Vec<DerivativeOrder> {
    orders_to_place
        .into_iter()
        .filter(|order| order.order_info.quantity.gt(&market.min_quantity_tick_size) && order.order_info.price.gt(&market.min_price_tick_size))
        .collect()
}

/// # Description
/// Returns true if the position is too close to liquidation
/// # Arguments
/// * `state` - All the orders that we are trying to create
/// * `position` - The position we have taken, if any
/// * `oracle_price` - On chain oracle price
/// * `market` - Derivative market information
/// # Returns
/// * `is_close_to_liquidation` - The position is about to be liquidated
pub fn position_close_to_liquidation(state: &State, position: &Option<EffectivePosition>, oracle_price: Decimal, market: &DerivativeMarket) -> bool {
    match position {
        None => false,
        Some(p) => {
            let position_margin_ratio = div_dec(p.effective_margin, oracle_price * p.quantity);
            let proximity_to_liquidation = div_dec(position_margin_ratio, market.maintenance_margin_ratio);
            proximity_to_liquidation <= state.min_proximity_to_liquidation
        }
    }
}

/// # Description
/// Returns true if the position is greater than the max position
/// # Arguments
/// * `position` - The position we have taken, if any
/// * `total_deposit_balance` - The total quote balance LPed
/// # Returns
/// * `position_is_too_large` - The position needs to be reduced below the max position
pub fn position_too_large(position: &Option<EffectivePosition>, total_deposit_balance: Decimal, market: &DerivativeMarket) -> bool {
    match position {
        None => false,
        Some(p) => {
            let max_position_value = div_dec(total_deposit_balance, Decimal::from_str("2").unwrap());
            p.effective_margin > max_position_value + market.min_price_tick_size
        }
    }
}

/// # Description
/// Creates an exchange message intended to close the entire position
/// # Arguments
/// * `contract_address` - The maker contract's address
/// * `market` - Derivative market information
/// * `position` - The position we have taken, if any
/// * `mid_price` - The current onchain mid price
/// * `state` - All the orders that we are trying to create
/// # Returns
/// * `position_close_order` - The exchange message that will close our position
pub fn close_position(
    contract_address: String,
    market: &DerivativeMarket,
    position: &EffectivePosition,
    mid_price: Decimal,
    state: &State,
) -> CosmosMsg<InjectiveMsgWrapper> {
    let worst_price = if position.is_long {
        mid_price * Decimal::from_str("4").unwrap()
    } else {
        Decimal::zero()
    };
    let position_close_order = DerivativeOrder::new(&state, worst_price, position.quantity, !position.is_long, true, market);
    create_derivative_market_order_msg(
        contract_address,
        serde_json_wasm::from_str(&serde_json_wasm::to_string(&position_close_order).unwrap()).unwrap(),
    )
}

/// # Description
/// Creates an exchange message intended to reduce the position below 95% of our max position value
/// # Arguments
/// * `contract_address` - The maker contract's address
/// * `market` - Derivative market information
/// * `position` - The position we have taken, if any
/// * `mid_price` - The current onchain mid price
/// * `total_deposit_balance` - The total quote balance LPed
/// * `state` - All the orders that we are trying to create
/// # Returns
/// * `position_reduce_order` - The exchange message that will reduce our position
pub fn reduce_below_max_position(
    contract_address: String,
    market: &DerivativeMarket,
    position: &EffectivePosition,
    mid_price: Decimal,
    total_deposit_balance: Decimal,
    state: &State,
) -> CosmosMsg<InjectiveMsgWrapper> {
    let worst_price = if position.is_long {
        mid_price * Decimal::from_str("4").unwrap()
    } else {
        Decimal::zero()
    };
    let max_position_value = div_dec(total_deposit_balance, Decimal::from_str("2").unwrap());
    let target_position_value = max_position_value * Decimal::from_str("0.95").unwrap();
    let excess_value = sub_no_overflow(position.effective_margin, target_position_value);
    let quantity_to_reduce = div_dec(excess_value, mid_price);
    let position_reduce_order = DerivativeOrder::new(&state, worst_price, quantity_to_reduce, !position.is_long, true, &market);
    create_derivative_market_order_msg(
        contract_address,
        serde_json_wasm::from_str(&serde_json_wasm::to_string(&position_reduce_order).unwrap()).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::check_tail_dist;
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
        let (buy_tail, sell_tail) = check_tail_dist(
            buy_head,
            sell_head,
            proposed_buy_tail,
            proposed_sell_tail,
            min_head_to_tail_deviation_ratio,
        );
        assert_eq!(buy_tail, proposed_buy_tail);
        assert_eq!(sell_tail, proposed_sell_tail);

        let proposed_buy_tail = Decimal::from_str("3998").unwrap();
        let proposed_sell_tail = Decimal::from_str("4002").unwrap();
        let (buy_tail, sell_tail) = check_tail_dist(
            buy_head,
            sell_head,
            proposed_buy_tail,
            proposed_sell_tail,
            min_head_to_tail_deviation_ratio,
        );
        assert_eq!(buy_tail, buy_head * (Decimal::one() - min_head_to_tail_deviation_ratio));
        assert_eq!(sell_tail, sell_head * (Decimal::one() + min_head_to_tail_deviation_ratio));
    }
}
