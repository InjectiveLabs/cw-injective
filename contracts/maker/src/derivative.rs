use crate::{
    msg::{WrappedOpenOrder, WrappedOrderResponse, WrappedPosition},
    state::State,
    utils::{div_dec, div_int, sub_abs},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};
use std::str::FromStr;

pub fn inv_imbalance_deriv(
    position: &Option<WrappedPosition>, inv_val: Decimal,
) -> (Decimal, bool) {
    match position {
        None => (Decimal::zero(), true),
        Some(position) => {
            let position_value = position.margin;
            let inv_imbalance = div_dec(position_value, inv_val);
            (inv_imbalance, position.is_long)
        }
    }
}

/// Uses the new head/tail to determine if there are any orders that need to be cancelled.
/// # Arguments
/// * `open_orders` - A list of open orders from the last block
/// * `new_head` - The new head (closest to the reservation price)
/// * `new_tail` - The new tail (farthest from the reservation price)
/// * `is_buy` - True if all open_orders are buy. False if they are all sell
/// # Returns
/// * `hashes_to_cancel` - A list of order hashes that we would like to cancel
/// * `orders_to_keep` - A list of open orders that we are going to keep on the book
/// * `orders_remaining_val` - An aggregation of the total notional value of orders_to_keep
/// * `append_to_new_head` - An indication of whether we should append new orders to the new head
/// or to the back of the orders_to_keep block
pub fn orders_to_cancel_deriv(
    open_orders: Vec<WrappedOpenOrder>, new_head: Decimal, new_tail: Decimal, is_buy: bool,
) -> (Vec<String>, Vec<WrappedOpenOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut hashes_to_cancel: Vec<String> = Vec::new();
    // If there are any open orders, we need to check them to see if we should cancel
    if open_orders.len() > 0 {
        // Use the new tail/head to filter out the orders to cancel
        let orders_to_keep: Vec<WrappedOpenOrder> = open_orders
            .into_iter()
            .filter(|o| {
                let keep_if_buy = o.price <= new_head && o.price >= new_tail;
                let keep_if_sell = o.price >= new_head && o.price <= new_tail;
                let keep = (keep_if_buy && is_buy) || (keep_if_sell && !is_buy);
                if keep {
                    orders_remaining_val = orders_remaining_val + (o.price * o.qty);
                } else {
                    hashes_to_cancel.push(o.order_hash.clone());
                }
                keep
            })
            .collect();
        // Determine if we need to append to new orders to the new head or if we need to
        // append to the end of the block of orders we will be keeping
        let append_to_new_head = sub_abs(new_head, orders_to_keep.first().unwrap().price)
            > sub_abs(orders_to_keep.last().unwrap().price, new_tail);
        (hashes_to_cancel, orders_to_keep, orders_remaining_val, append_to_new_head)
    } else {
        (hashes_to_cancel, Vec::new(), orders_remaining_val, true)
    }
}

pub fn create_new_orders_base_deriv(
    new_head: Decimal, new_tail: Decimal, inv_val: Decimal, orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal, mut position_qty: Decimal, touch_head: bool, is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Decimal) {
    let num_open_orders = Uint256::from_str(&orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let num_orders_to_open = state.order_density - num_open_orders;
    let alloc_val_for_new_orders =
        div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap())
            - orders_remaining_val;
    let val_per_order = alloc_val_for_new_orders / num_orders_to_open;
    let val_per_order = val_per_order * state.leverage;

    // Since we have no orders remaining after cancellation, all we need to do is create orders
    // between the new head and tail
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
            // If position is the same side as these orders no need to create any reduce only
            let new_order = WrappedOrderResponse::new(state, current_price, qty, is_buy, false);
            orders_to_open.push(new_order);
        } else {
            // We need to manage reduce only orders here
            if qty > position_qty {
                // We need to make two orders here, one reduce only and one for the remainder
                let new_order_reduce =
                    WrappedOrderResponse::new(state, current_price, position_qty, is_buy, true);
                let new_order = WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty - position_qty,
                    is_buy,
                    false,
                );
                orders_to_open.push(new_order_reduce);
                orders_to_open.push(new_order);
                position_qty = Decimal::zero();
            } else {
                // This whole order should be reduce only
                let new_order_reduce =
                    WrappedOrderResponse::new(state, current_price, qty, is_buy, true);
                orders_to_open.push(new_order_reduce);
                position_qty = position_qty - qty;
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

pub fn create_new_orders_tail_to_head_deriv(
    new_head: Decimal, inv_val: Decimal, orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal, position_qty: Decimal, is_buy: bool, state: &State,
) -> (Vec<WrappedOrderResponse>, Vec<String>) {
    let (mut orders_to_open, mut position_qty) = create_new_orders_base_deriv(
        new_head,
        orders_to_keep.first().unwrap().price,
        inv_val,
        orders_to_keep.clone(),
        orders_remaining_val,
        position_qty,
        true,
        is_buy,
        state,
    );
    let mut additional_hashes_to_cancel: Vec<String> = Vec::new();
    orders_to_keep.iter().for_each(|o| {
        if position_qty > Decimal::zero() {
            if o.qty > position_qty {
                additional_hashes_to_cancel.push(o.order_hash.clone());
                let new_order_reduce =
                    WrappedOrderResponse::new(state, o.price, position_qty, is_buy, true);
                let new_order =
                    WrappedOrderResponse::new(state, o.price, o.qty - position_qty, is_buy, false);
                orders_to_open.push(new_order_reduce);
                orders_to_open.push(new_order);
                position_qty = Decimal::zero();
            } else {
                if o.is_reduce_only {
                    position_qty = position_qty - o.qty;
                } else {
                    // This whole order should be reduce only
                    additional_hashes_to_cancel.push(o.order_hash.clone());
                    let new_order_reduce =
                        WrappedOrderResponse::new(state, o.price, o.qty, is_buy, true);
                    orders_to_open.push(new_order_reduce);
                    position_qty = position_qty - o.qty;
                }
            }
        } else {
            if o.is_reduce_only {
                additional_hashes_to_cancel.push(o.order_hash.clone());
                let new_order =
                    WrappedOrderResponse::new(state, o.price, o.qty - position_qty, is_buy, false);
                orders_to_open.push(new_order);
            }
        }
    });

    (orders_to_open, additional_hashes_to_cancel)
}

pub fn create_new_orders_head_to_tail_deriv(
    new_tail: Decimal, inv_val: Decimal, orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal, mut position_qty: Decimal, is_buy: bool, state: &State,
) -> (Vec<WrappedOrderResponse>, Vec<String>) {
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let mut additional_hashes_to_cancel: Vec<String> = Vec::new();
    orders_to_keep.iter().for_each(|o| {
        if position_qty > Decimal::zero() {
            if o.qty > position_qty {
                additional_hashes_to_cancel.push(o.order_hash.clone());
                let new_order_reduce =
                    WrappedOrderResponse::new(state, o.price, position_qty, is_buy, true);
                let new_order =
                    WrappedOrderResponse::new(state, o.price, o.qty - position_qty, is_buy, false);
                orders_to_open.push(new_order_reduce);
                orders_to_open.push(new_order);
                position_qty = Decimal::zero();
            } else {
                if o.is_reduce_only {
                    position_qty = position_qty - o.qty;
                } else {
                    // This whole order should be reduce only
                    additional_hashes_to_cancel.push(o.order_hash.clone());
                    let new_order_reduce =
                        WrappedOrderResponse::new(state, o.price, o.qty, is_buy, true);
                    orders_to_open.push(new_order_reduce);
                    position_qty = position_qty - o.qty;
                }
            }
        } else {
            if o.is_reduce_only {
                additional_hashes_to_cancel.push(o.order_hash.clone());
                let new_order =
                    WrappedOrderResponse::new(state, o.price, o.qty - position_qty, is_buy, false);
                orders_to_open.push(new_order);
            }
        }
    });
    let (mut orders_to_open_b, _) = create_new_orders_base_deriv(
        orders_to_keep.last().unwrap().price,
        new_tail,
        inv_val,
        orders_to_keep,
        orders_remaining_val,
        position_qty,
        false,
        is_buy,
        state,
    );
    orders_to_open.append(&mut orders_to_open_b);
    (orders_to_open, additional_hashes_to_cancel)
}

#[test]
fn orders_to_cancel_for_buy_test() {
    let mut open_buy_orders: Vec<WrappedOpenOrder> = Vec::new();
    let order = WrappedOpenOrder {
        order_hash: String::from(""),
        is_buy: true,
        qty: Decimal::one(),
        price: Decimal::zero(),
        is_reduce_only: false,
    };

    let old_tail_price = 10;
    let order_density = 10;

    let new_buy_head_a =
        Decimal::from_str(&(old_tail_price + order_density + 1).to_string()).unwrap();
    let new_buy_tail_a = Decimal::from_str(&(old_tail_price + 1).to_string()).unwrap();
    let mut buy_orders_remaining_val_a = Decimal::zero();

    let new_buy_head_b =
        Decimal::from_str(&(old_tail_price + order_density - 1).to_string()).unwrap();
    let new_buy_tail_b = Decimal::from_str(&(old_tail_price + 1).to_string()).unwrap();
    let mut buy_orders_remaining_val_b = Decimal::zero();

    let mut buy = order.clone();
    for i in (old_tail_price..(old_tail_price + order_density + 1))
        .into_iter()
        .rev()
    {
        let price = Decimal::from_str(&i.to_string()).unwrap();
        buy.price = price;
        buy.is_reduce_only = i % 2 == 0;
        if price <= new_buy_head_a && price >= new_buy_tail_a {
            buy_orders_remaining_val_a = buy_orders_remaining_val_a + price;
        }
        if price <= new_buy_head_b && price >= new_buy_tail_b {
            buy_orders_remaining_val_b = buy_orders_remaining_val_b + price;
        }
        open_buy_orders.push(buy.clone());
    }

    // Check case where we need to cancel orders by the tail because the new head > old head
    let (
        buy_hashes_to_cancel,
        buy_orders_to_keep,
        buy_orders_remaining_val,
        buy_append_new_to_head,
    ) = orders_to_cancel_deriv(open_buy_orders.clone(), new_buy_head_a, new_buy_tail_a, true);
    assert!(buy_append_new_to_head);
    assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_a);
    assert_eq!(open_buy_orders.len() - buy_orders_to_keep.len(), buy_hashes_to_cancel.len());

    // Check case where we need to cancel orders by the head because the new head < old head
    let (
        buy_hashes_to_cancel,
        buy_orders_to_keep,
        buy_orders_remaining_val,
        buy_append_new_to_head,
    ) = orders_to_cancel_deriv(open_buy_orders.clone(), new_buy_head_b, new_buy_tail_b, true);
    assert!(!buy_append_new_to_head);
    assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_b);
    assert_eq!(open_buy_orders.len() - buy_orders_to_keep.len(), buy_hashes_to_cancel.len());

    // Check case where there were no open orders at all
    let (buy_hashes_to_cancel, _, _, buy_append_new_to_head) =
        orders_to_cancel_deriv(Vec::new(), new_buy_head_a, new_buy_tail_a, true);
    assert!(buy_append_new_to_head);
    assert_eq!(0, buy_hashes_to_cancel.len());
}

#[cfg(test)]
mod tests {
    use crate::{msg::WrappedOpenOrder, spot::orders_to_cancel_spot};
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;

    #[test]
    fn orders_to_cancel_for_sell_test() {
        let mut open_sell_orders: Vec<WrappedOpenOrder> = Vec::new();
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: false,
            qty: Decimal::one(),
            price: Decimal::zero(),
            is_reduce_only: false,
        };

        let old_head_price = 10;
        let order_density = 10;

        let new_sell_head_a = Decimal::from_str(&(old_head_price + 1).to_string()).unwrap();
        let new_sell_tail_a =
            Decimal::from_str(&(old_head_price + order_density + 1).to_string()).unwrap();
        let mut sell_orders_remaining_val_a = Decimal::zero();

        let new_sell_head_b = Decimal::from_str(&(old_head_price - 1).to_string()).unwrap();
        let new_sell_tail_b =
            Decimal::from_str(&(old_head_price + order_density - 1).to_string()).unwrap();
        let mut sell_orders_remaining_val_b = Decimal::zero();

        let mut sell = order.clone();
        for i in old_head_price..(old_head_price + order_density + 1) {
            let price = Decimal::from_str(&i.to_string()).unwrap();
            sell.price = price;
            sell.is_reduce_only = i % 2 == 0;
            if price >= new_sell_head_a && price <= new_sell_tail_a {
                sell_orders_remaining_val_a = sell_orders_remaining_val_a + price;
            }
            if price >= new_sell_head_b && price <= new_sell_tail_b {
                sell_orders_remaining_val_b = sell_orders_remaining_val_b + price;
            }
            open_sell_orders.push(sell.clone());
        }

        // Check case where we need to cancel orders by the tail because the new head > old head
        let (
            sell_hashes_to_cancel,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            sell_append_new_to_head,
        ) = orders_to_cancel_spot(
            open_sell_orders.clone(),
            new_sell_head_a,
            new_sell_tail_a,
            false,
        );
        assert!(!sell_append_new_to_head);
        assert_eq!(sell_orders_remaining_val, sell_orders_remaining_val_a);
        assert_eq!(open_sell_orders.len() - sell_orders_to_keep.len(), sell_hashes_to_cancel.len());

        // Check case where we need to cancel orders by the tail because the new head > old head
        let (
            sell_hashes_to_cancel,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            sell_append_new_to_head,
        ) = orders_to_cancel_spot(
            open_sell_orders.clone(),
            new_sell_head_b,
            new_sell_tail_b,
            false,
        );
        assert!(sell_append_new_to_head);
        assert_eq!(sell_orders_remaining_val, sell_orders_remaining_val_b);
        assert_eq!(open_sell_orders.len() - sell_orders_to_keep.len(), sell_hashes_to_cancel.len());

        // Check case where there were no open orders at all
        let (sell_hashes_to_cancel, _, _, sell_append_new_to_head) =
            orders_to_cancel_spot(Vec::new(), new_sell_head_a, new_sell_tail_a, false);
        assert!(sell_append_new_to_head);
        assert_eq!(0, sell_hashes_to_cancel.len());
    }
}
