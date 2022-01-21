use crate::{
    msg::{WrappedOpenOrder, WrappedOrderResponse, WrappedPosition},
    state::State,
    utils::{div_dec, div_int, sub_abs},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};
use std::str::FromStr;

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

pub fn base_deriv(
    new_head: Decimal,
    new_tail: Decimal,
    inv_val: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal,
    mut position_qty: Decimal,
    touch_head: bool,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Decimal) {
    let num_open_orders = Uint256::from_str(&orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let num_orders_to_open = state.order_density - num_open_orders;
    let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - orders_remaining_val;
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
                let new_order_reduce = WrappedOrderResponse::new(state, current_price, position_qty, is_buy, true);
                let new_order = WrappedOrderResponse::new(state, current_price, qty - position_qty, is_buy, false);
                orders_to_open.push(new_order_reduce);
                orders_to_open.push(new_order);
                position_qty = Decimal::zero();
            } else {
                // This whole order should be reduce only
                let new_order_reduce = WrappedOrderResponse::new(state, current_price, qty, is_buy, true);
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

pub fn tail_to_head_deriv(
    new_head: Decimal,
    inv_val: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal,
    position_qty: Decimal,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Vec<String>) {
    let (mut orders_to_open, position_qty) = base_deriv(
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
    let additional_hashes_to_cancel = handle_reduce_ony(orders_to_keep.clone(), position_qty, &mut orders_to_open, is_buy, state);
    (orders_to_open, additional_hashes_to_cancel)
}

pub fn head_to_tail_deriv(
    new_tail: Decimal,
    inv_val: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal,
    position_qty: Decimal,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Vec<String>) {
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let additional_hashes_to_cancel = handle_reduce_ony(orders_to_keep.clone(), position_qty, &mut orders_to_open, is_buy, state);
    let (mut orders_to_open_b, _) = base_deriv(
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

fn handle_reduce_ony(
    orders_to_keep: Vec<WrappedOpenOrder>,
    mut position_qty: Decimal,
    orders_to_open: &mut Vec<WrappedOrderResponse>,
    is_buy: bool,
    state: &State,
) -> Vec<String> {
    let mut additional_hashes_to_cancel: Vec<String> = Vec::new();
    orders_to_keep.iter().for_each(|o| {
        if position_qty > Decimal::zero() {
            if o.qty > position_qty {
                additional_hashes_to_cancel.push(o.order_hash.clone());
                let new_order_reduce = WrappedOrderResponse::new(state, o.price, position_qty, is_buy, true);
                let new_order = WrappedOrderResponse::new(state, o.price, o.qty - position_qty, is_buy, false);
                orders_to_open.push(new_order_reduce);
                orders_to_open.push(new_order);
                position_qty = Decimal::zero();
            } else {
                if o.is_reduce_only {
                    position_qty = position_qty - o.qty;
                } else {
                    // This whole order should be reduce only
                    additional_hashes_to_cancel.push(o.order_hash.clone());
                    let new_order_reduce = WrappedOrderResponse::new(state, o.price, o.qty, is_buy, true);
                    orders_to_open.push(new_order_reduce);
                    position_qty = position_qty - o.qty;
                }
            }
        } else {
            if o.is_reduce_only {
                additional_hashes_to_cancel.push(o.order_hash.clone());
                let new_order = WrappedOrderResponse::new(state, o.price, o.qty - position_qty, is_buy, false);
                orders_to_open.push(new_order);
            }
        }
    });
    additional_hashes_to_cancel
}
