use crate::{
    msg::{div_dec, div_int, WrappedOpenOrder, WrappedOrderResponse},
    state::State,
};
use core::num;
use cosmwasm_std::{Decimal256 as Decimal, Uint256};
use std::str::FromStr;

pub fn inv_imbalance_spot(inv_base_val: Decimal, inv_val: Decimal) -> (Decimal, bool) {
    let half_inv_val = div_int(inv_val, Uint256::from_str("2").unwrap());
    let inv_imbalance = if inv_base_val > half_inv_val {
        div_dec(inv_base_val - half_inv_val, inv_val)
    } else {
        div_dec(half_inv_val - inv_base_val, inv_val)
    };
    (inv_imbalance, inv_base_val > half_inv_val)
}

pub fn orders_to_cancel_for_buy(
    open_buy_orders: Vec<WrappedOpenOrder>,
    new_buy_head: Decimal,
    new_buy_tail: Decimal,
    inv_val: Decimal,
) -> (Vec<String>, Vec<WrappedOpenOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut hashes_to_cancel: Vec<String> = Vec::new();
    let buy_orders_to_keep: Vec<WrappedOpenOrder> = open_buy_orders
        .into_iter()
        .filter(|o| {
            let keep = new_buy_head > o.price && o.price > new_buy_tail;
            if keep {
                orders_remaining_val = orders_remaining_val + (o.price * o.qty);
            } else {
                hashes_to_cancel.push(o.order_hash.clone());
            }
            keep
        })
        .collect();
    let append_new_to_head = new_buy_head - buy_orders_to_keep.first().unwrap().price
        < buy_orders_to_keep.last().unwrap().price - new_buy_tail;
    (
        hashes_to_cancel,
        buy_orders_to_keep,
        orders_remaining_val,
        append_new_to_head,
    )
}

pub fn create_new_buy_orders(
    new_buy_head: Decimal,
    new_buy_tail: Decimal,
    inv_val: Decimal,
    buy_orders_to_keep: Vec<WrappedOpenOrder>,
    buy_orders_remaining_val: Decimal,
    buy_append_new_to_head: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    let num_open_orders = Uint256::from_str(&buy_orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    if num_open_orders < state.order_density {
        let num_orders_to_open = state.order_density - num_open_orders;
        let price_step = if buy_append_new_to_head {
            div_int(
                new_buy_head - buy_orders_to_keep.first().unwrap().price,
                num_orders_to_open,
            )
        } else {
            div_int(
                buy_orders_to_keep.last().unwrap().price - new_buy_tail,
                num_orders_to_open,
            )
        };
        let alloc_val_for_new_orders = div_dec(
            inv_val * state.active_capital_perct,
            Decimal::from_str("2").unwrap(),
        ) - buy_orders_remaining_val;
        let val_per_order = alloc_val_for_new_orders / num_orders_to_open;
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        if buy_append_new_to_head {
            let mut current_price = new_buy_head;
            for i in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                orders_to_open.push(WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty,
                    true,
                    false,
                ));
                current_price = current_price - price_step;
            }
        } else {
            let mut current_price = new_buy_tail - price_step;
            for i in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                orders_to_open.push(WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty,
                    true,
                    false,
                ));
                current_price = current_price - price_step;
            }
        }
    }
    orders_to_open
}

pub fn orders_to_cancel_for_sell(
    open_sell_orders: Vec<WrappedOpenOrder>,
    new_sell_head: Decimal,
    new_sell_tail: Decimal,
    inv_val: Decimal,
) -> (Vec<String>, Vec<WrappedOpenOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut hashes_to_cancel: Vec<String> = Vec::new();
    let sell_orders_to_keep: Vec<WrappedOpenOrder> = open_sell_orders
        .into_iter()
        .filter(|o| {
            let keep = o.price > new_sell_head && new_sell_tail > o.price;
            if keep {
                orders_remaining_val = orders_remaining_val + (o.price * o.qty);
            } else {
                hashes_to_cancel.push(o.order_hash.clone());
            }
            keep
        })
        .collect();
    let append_new_to_head = sell_orders_to_keep.first().unwrap().price - new_sell_head
        < new_sell_tail - sell_orders_to_keep.last().unwrap().price;
    (
        hashes_to_cancel,
        sell_orders_to_keep,
        orders_remaining_val,
        append_new_to_head,
    )
}

pub fn create_new_sell_orders(
    new_sell_head: Decimal,
    new_sell_tail: Decimal,
    inv_val: Decimal,
    sell_orders_to_keep: Vec<WrappedOpenOrder>,
    sell_orders_remaining_val: Decimal,
    sell_append_new_to_head: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    let num_open_orders = Uint256::from_str(&sell_orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    if num_open_orders < state.order_density {
        let num_orders_to_open = state.order_density - num_open_orders;
        let price_step = if sell_append_new_to_head {
            div_int(
                sell_orders_to_keep.first().unwrap().price - new_sell_head,
                num_orders_to_open,
            )
        } else {
            div_int(
                new_sell_tail - sell_orders_to_keep.last().unwrap().price,
                num_orders_to_open,
            )
        };
        let alloc_val_for_new_orders = div_dec(
            inv_val * state.active_capital_perct,
            Decimal::from_str("2").unwrap(),
        ) - sell_orders_remaining_val;
        let val_per_order = alloc_val_for_new_orders / num_orders_to_open;
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        if sell_append_new_to_head {
            let mut current_price = new_sell_head;
            for i in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                orders_to_open.push(WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty,
                    true,
                    false,
                ));
                current_price = current_price + price_step;
            }
        } else {
            let mut current_price = new_sell_tail + price_step;
            for i in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                orders_to_open.push(WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty,
                    true,
                    false,
                ));
                current_price = current_price + price_step;
            }
        }
    }
    orders_to_open
}
