use crate::{
    msg::{WrappedOpenOrder, WrappedOrderResponse},
    state::State,
    utils::{div_dec, div_int, sub_abs},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};
use std::str::FromStr;

pub fn inv_imbalance_spot(inv_base_val: Decimal, inv_val: Decimal) -> (Decimal, bool) {
    let half_inv_val = div_int(inv_val, Uint256::from_str("2").unwrap());
    let inv_imbalance = div_dec(sub_abs(inv_base_val, half_inv_val), inv_val);
    (inv_imbalance, inv_base_val > half_inv_val)
}

pub fn orders_to_cancel(
    open_orders: Vec<WrappedOpenOrder>,
    new_head: Decimal,
    new_tail: Decimal,
    is_buy: bool,
) -> (Vec<String>, Vec<WrappedOpenOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut hashes_to_cancel: Vec<String> = Vec::new();
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
    let append_new_to_head = if is_buy {
        sub_abs(new_head, orders_to_keep.first().unwrap().price)
            > sub_abs(orders_to_keep.last().unwrap().price, new_tail)
    } else {
        sub_abs(orders_to_keep.first().unwrap().price, new_head)
            > sub_abs(new_tail, orders_to_keep.last().unwrap().price)
    };
    (
        hashes_to_cancel,
        orders_to_keep,
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
            for _ in 0..num_orders_to_open {
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
            for _ in 0..num_orders_to_open {
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
            for _ in 0..num_orders_to_open {
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
            for _ in 0..num_orders_to_open {
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

#[cfg(test)]
mod tests {
    use crate::{msg::WrappedOpenOrder, spot::orders_to_cancel};
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;

    #[test]
    fn orders_to_cancel_for_buy_test() {
        let mut open_buy_orders: Vec<WrappedOpenOrder> = Vec::new();
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: true,
            qty: Decimal::one(),
            price: Decimal::zero(),
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
            if price <= new_buy_head_a && price >= new_buy_tail_a {
                buy_orders_remaining_val_a = buy_orders_remaining_val_a + price;
            }
            if price <= new_buy_head_b && price >= new_buy_tail_b {
                buy_orders_remaining_val_b = buy_orders_remaining_val_b + price;
            }
            open_buy_orders.push(buy.clone());
        }

        let (
            buy_hashes_to_cancel,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            buy_append_new_to_head,
        ) = orders_to_cancel(open_buy_orders.clone(), new_buy_head_a, new_buy_tail_a, true);
        assert!(buy_append_new_to_head);
        assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_a);
        assert_eq!(
            open_buy_orders.len() - buy_orders_to_keep.len(),
            buy_hashes_to_cancel.len()
        );

        let (
            buy_hashes_to_cancel,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            buy_append_new_to_head,
        ) = orders_to_cancel(open_buy_orders.clone(), new_buy_head_b, new_buy_tail_b, true);
        assert!(!buy_append_new_to_head);
        assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_b);
        assert_eq!(
            open_buy_orders.len() - buy_orders_to_keep.len(),
            buy_hashes_to_cancel.len()
        );
    }

    #[test]
    fn orders_to_cancel_for_sell_test() {
        let mut open_sell_orders: Vec<WrappedOpenOrder> = Vec::new();
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: false,
            qty: Decimal::one(),
            price: Decimal::zero(),
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
            if price >= new_sell_head_a && price <= new_sell_tail_a {
                sell_orders_remaining_val_a = sell_orders_remaining_val_a + price;
            }
            if price >= new_sell_head_b && price <= new_sell_tail_b {
                sell_orders_remaining_val_b = sell_orders_remaining_val_b + price;
            }
            open_sell_orders.push(sell.clone());
        }

        let (
            sell_hashes_to_cancel,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            sell_append_new_to_head,
        ) = orders_to_cancel(open_sell_orders.clone(), new_sell_head_a, new_sell_tail_a, false);
        assert!(!sell_append_new_to_head);
        assert_eq!(sell_orders_remaining_val, sell_orders_remaining_val_a);
        assert_eq!(
            open_sell_orders.len() - sell_orders_to_keep.len(),
            sell_hashes_to_cancel.len()
        );

        let (
            sell_hashes_to_cancel,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            sell_append_new_to_head,
        ) = orders_to_cancel(open_sell_orders.clone(), new_sell_head_b, new_sell_tail_b, false);
        assert!(sell_append_new_to_head);
        assert_eq!(sell_orders_remaining_val, sell_orders_remaining_val_b);
        assert_eq!(
            open_sell_orders.len() - sell_orders_to_keep.len(),
            sell_hashes_to_cancel.len()
        );
    }
}
