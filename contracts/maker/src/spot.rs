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

pub fn orders_to_cancel_spot(
    open_orders: Vec<WrappedOpenOrder>,
    new_head: Decimal,
    new_tail: Decimal,
    is_buy: bool,
) -> (Vec<String>, Vec<WrappedOpenOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut hashes_to_cancel: Vec<String> = Vec::new();
    if open_orders.len() > 0 {
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
    } else {
        (hashes_to_cancel, Vec::new(), orders_remaining_val, true)
    }
}

pub fn create_new_orders_spot(
    new_head: Decimal,
    new_tail: Decimal,
    inv_val: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    orders_remaining_val: Decimal,
    append_new_to_head: bool,
    is_buy: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    let num_open_orders = Uint256::from_str(&orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let num_orders_to_open = state.order_density - num_open_orders;
    let alloc_val_for_new_orders = div_dec(
        inv_val * state.active_capital_perct,
        Decimal::from_str("2").unwrap(),
    ) - orders_remaining_val;
    let val_per_order = alloc_val_for_new_orders / num_orders_to_open;

    if orders_to_keep.len() == 0 {
        let price_dist = sub_abs(new_head, new_tail);
        let price_step = div_int(price_dist, num_orders_to_open);
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        let mut current_price = new_head;
        for _ in 0..num_orders_to_open {
            let qty = div_dec(val_per_order, current_price);
            orders_to_open.push(WrappedOrderResponse::new(
                state,
                current_price,
                qty,
                true,
                false,
            ));
            current_price = if is_buy {
                current_price - price_step
            } else {
                current_price + price_step
            }
        }
    } else if num_open_orders < state.order_density {
        let price_dist = if append_new_to_head {
            sub_abs(new_head, orders_to_keep.first().unwrap().price)
        } else {
            sub_abs(orders_to_keep.last().unwrap().price, new_tail)
        };
        let price_step = div_int(price_dist, num_orders_to_open);
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        if append_new_to_head {
            let mut current_price = new_head;
            for _ in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                orders_to_open.push(WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty,
                    true,
                    false,
                ));
                current_price = if is_buy {
                    current_price - price_step
                } else {
                    current_price + price_step
                }
            }
        } else {
            let mut current_price = if is_buy {
                orders_to_keep.last().unwrap().price - price_step
            } else {
                orders_to_keep.last().unwrap().price + price_step
            };
            for _ in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                orders_to_open.push(WrappedOrderResponse::new(
                    state,
                    current_price,
                    qty,
                    true,
                    false,
                ));
                current_price = if is_buy {
                    current_price - price_step
                } else {
                    current_price + price_step
                }
            }
        }
    }
    orders_to_open
}

#[cfg(test)]
mod tests {
    use crate::{
        msg::WrappedOpenOrder,
        spot::{create_new_orders_spot, orders_to_cancel_spot},
        state::State,
        utils::{div_dec, sub_abs},
    };
    use cosmwasm_std::{Decimal256 as Decimal, Uint256};
    use std::str::FromStr;

    #[test]
    fn create_orders_for_buy_test() {
        let state = State {
            market_id: String::from(""),
            manager: String::from(""),
            fee_recipient: String::from(""),
            sub_account: String::from(""),
            risk_aversion: Decimal::from_str("0.2").unwrap(),
            order_density: Uint256::from_str("33").unwrap(),
            active_capital_perct: Decimal::from_str("0.2").unwrap(),
            max_notional_position: Decimal::zero(),
            min_pnl: Decimal::zero(),
            manual_offset_perct: Decimal::zero(),
            tail_dist_to_head_bp: Decimal::from_str("300").unwrap(),
            head_chg_tol_bp: Decimal::zero(),
            max_dd: Decimal::one(),
            leverage: Decimal::one(),
            decimal_shift: Uint256::from_str("1000000").unwrap(),
            base_precision_shift: Uint256::from_str("1000").unwrap(),
        };
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: true,
            qty: Decimal::one(),
            price: Decimal::zero(),
        };
        let inv_val = Decimal::from_str("1000000").unwrap();

        // Check when there are no open orders
        let new_buy_head = Decimal::from_str("10").unwrap();
        let new_buy_tail = Decimal::from_str("1").unwrap();
        let buy_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let buy_orders_remaining_val = Decimal::zero();
        let buy_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_buy_head,
            new_buy_tail,
            inv_val,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            buy_append_new_to_head,
            true,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() > orders[i].get_price())
            }
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        assert_eq!(new_buy_head, orders.first().unwrap().get_price());
        assert!(new_buy_tail <= orders.last().unwrap().get_price());
        assert_eq!(
            Uint256::from_str(&orders.len().to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(
            state.active_capital_perct * inv_val,
            Decimal::from_str("2").unwrap(),
        );
        assert!(
            sub_abs(expected_notional_value, total_notional_val)
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );

        // Check when there are open orders and we want to add to the new head
        let new_buy_head_i = 20;
        let new_buy_tail_i = 10;
        let new_buy_head = Decimal::from_str(&new_buy_head_i.to_string()).unwrap();
        let new_buy_tail = Decimal::from_str(&new_buy_tail_i.to_string()).unwrap();

        let mut buy_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let mut buy_orders_remaining_val = Decimal::zero();
        for i in ((new_buy_tail_i)..(new_buy_head_i)).rev() {
            let mut o = order.clone();
            o.price = Decimal::from_str(&i.to_string()).unwrap();
            buy_orders_remaining_val = buy_orders_remaining_val + (o.price * o.qty);
            buy_orders_to_keep.push(o.clone());
        }
        let buy_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_buy_head,
            new_buy_tail,
            inv_val,
            buy_orders_to_keep.clone(),
            buy_orders_remaining_val,
            buy_append_new_to_head,
            true,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() > orders[i].get_price())
            }
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        buy_orders_to_keep.iter().for_each(|o| {
            total_notional_val = total_notional_val + o.price;
        });
        assert_eq!(new_buy_head, orders.first().unwrap().get_price());
        assert!(new_buy_tail <= orders.last().unwrap().get_price());
        assert_eq!(
            Uint256::from_str(&(orders.len() + buy_orders_to_keep.len()).to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(
            state.active_capital_perct * inv_val,
            Decimal::from_str("2").unwrap(),
        );
        assert!(
            sub_abs(expected_notional_value, total_notional_val)
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );

        // Check when there are open orders and we want to add to the last order we kept
        let mut buy_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let mut buy_orders_remaining_val = Decimal::zero();
        for i in ((new_buy_tail_i + 1)..(new_buy_head_i + 1)).rev() {
            let mut o = order.clone();
            o.price = Decimal::from_str(&i.to_string()).unwrap();
            buy_orders_remaining_val = buy_orders_remaining_val + (o.price * o.qty);
            buy_orders_to_keep.push(o.clone());
        }
        let buy_append_new_to_head = false;
        let orders = create_new_orders_spot(
            new_buy_head,
            new_buy_tail,
            inv_val,
            buy_orders_to_keep.clone(),
            buy_orders_remaining_val,
            buy_append_new_to_head,
            true,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() > orders[i].get_price())
            }
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        buy_orders_to_keep.iter().for_each(|o| {
            total_notional_val = total_notional_val + o.price;
        });
        assert!(new_buy_head >= orders.first().unwrap().get_price());
        assert_eq!(new_buy_tail, orders.last().unwrap().get_price());
        assert_eq!(
            Uint256::from_str(&(orders.len() + buy_orders_to_keep.len()).to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(
            state.active_capital_perct * inv_val,
            Decimal::from_str("2").unwrap(),
        );
        assert!(
            sub_abs(expected_notional_value, total_notional_val)
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );
    }

    #[test]
    fn create_orders_for_sell_test() {
        let state = State {
            market_id: String::from(""),
            manager: String::from(""),
            fee_recipient: String::from(""),
            sub_account: String::from(""),
            risk_aversion: Decimal::from_str("0.2").unwrap(),
            order_density: Uint256::from_str("33").unwrap(),
            active_capital_perct: Decimal::from_str("0.2").unwrap(),
            max_notional_position: Decimal::zero(),
            min_pnl: Decimal::zero(),
            manual_offset_perct: Decimal::zero(),
            tail_dist_to_head_bp: Decimal::from_str("300").unwrap(),
            head_chg_tol_bp: Decimal::zero(),
            max_dd: Decimal::one(),
            leverage: Decimal::one(),
            decimal_shift: Uint256::from_str("1000000").unwrap(),
            base_precision_shift: Uint256::from_str("1000").unwrap(),
        };
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy: false,
            qty: Decimal::one(),
            price: Decimal::zero(),
        };
        let inv_val = Decimal::from_str("1000000").unwrap();

        // Check when there are no open orders
        let new_sell_head = Decimal::from_str("1").unwrap();
        let new_sell_tail = Decimal::from_str("10").unwrap();
        let sell_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let sell_orders_remaining_val = Decimal::zero();
        let sell_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_sell_head,
            new_sell_tail,
            inv_val,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            sell_append_new_to_head,
            false,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() < orders[i].get_price())
            }
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        assert_eq!(new_sell_head, orders.first().unwrap().get_price());
        assert!(new_sell_tail >= orders.last().unwrap().get_price());
        assert_eq!(
            Uint256::from_str(&orders.len().to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(
            state.active_capital_perct * inv_val,
            Decimal::from_str("2").unwrap(),
        );
        assert!(
            sub_abs(expected_notional_value, total_notional_val)
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );

        // Check when there are open orders and we want to add to the new head
        let new_sell_head_i = 10;
        let new_sell_tail_i = 20;
        let new_sell_head = Decimal::from_str(&new_sell_head_i.to_string()).unwrap();
        let new_sell_tail = Decimal::from_str(&new_sell_tail_i.to_string()).unwrap();

        let mut sell_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let mut sell_orders_remaining_val = Decimal::zero();
        for i in (new_sell_head_i + 1)..(new_sell_tail_i + 1) {
            let mut o = order.clone();
            o.price = Decimal::from_str(&i.to_string()).unwrap();
            sell_orders_remaining_val = sell_orders_remaining_val + (o.price * o.qty);
            sell_orders_to_keep.push(o.clone());
        }
        let sell_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_sell_head,
            new_sell_tail,
            inv_val,
            sell_orders_to_keep.clone(),
            sell_orders_remaining_val,
            sell_append_new_to_head,
            false,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() < orders[i].get_price())
            }
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        sell_orders_to_keep.iter().for_each(|o| {
            total_notional_val = total_notional_val + o.price;
        });
        assert_eq!(new_sell_head, orders.first().unwrap().get_price());
        assert!(new_sell_tail >= orders.last().unwrap().get_price());
        assert_eq!(
            Uint256::from_str(&(orders.len() + sell_orders_to_keep.len()).to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(
            state.active_capital_perct * inv_val,
            Decimal::from_str("2").unwrap(),
        );
        assert!(
            sub_abs(expected_notional_value, total_notional_val)
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );

        // Check when there are open orders and we want to add to the last order we kept
        let mut sell_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let mut sell_orders_remaining_val = Decimal::zero();
        for i in (new_sell_head_i - 1)..(new_sell_tail_i - 1) {
            let mut o = order.clone();
            o.price = Decimal::from_str(&i.to_string()).unwrap();
            sell_orders_remaining_val = sell_orders_remaining_val + (o.price * o.qty);
            sell_orders_to_keep.push(o.clone());
        }
        let sell_append_new_to_head = false;
        let orders = create_new_orders_spot(
            new_sell_head,
            new_sell_tail,
            inv_val,
            sell_orders_to_keep.clone(),
            sell_orders_remaining_val,
            sell_append_new_to_head,
            false,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() < orders[i].get_price())
            }
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        sell_orders_to_keep.iter().for_each(|o| {
            total_notional_val = total_notional_val + o.price;
        });
        assert!(new_sell_head <= orders.first().unwrap().get_price());
        assert!(
            sub_abs(new_sell_tail, orders.last().unwrap().get_price())
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );
        assert_eq!(
            Uint256::from_str(&(orders.len() + sell_orders_to_keep.len()).to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(
            state.active_capital_perct * inv_val,
            Decimal::from_str("2").unwrap(),
        );
        assert!(
            sub_abs(expected_notional_value, total_notional_val)
                < Decimal::from_str("0.00001").unwrap() * expected_notional_value
        );
    }

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

        // Check case where we need to cancel orders by the tail because the new head > old head
        let (
            buy_hashes_to_cancel,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            buy_append_new_to_head,
        ) = orders_to_cancel_spot(
            open_buy_orders.clone(),
            new_buy_head_a,
            new_buy_tail_a,
            true,
        );
        assert!(buy_append_new_to_head);
        assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_a);
        assert_eq!(
            open_buy_orders.len() - buy_orders_to_keep.len(),
            buy_hashes_to_cancel.len()
        );

        // Check case where we need to cancel orders by the head because the new head < old head
        let (
            buy_hashes_to_cancel,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            buy_append_new_to_head,
        ) = orders_to_cancel_spot(
            open_buy_orders.clone(),
            new_buy_head_b,
            new_buy_tail_b,
            true,
        );
        assert!(!buy_append_new_to_head);
        assert_eq!(buy_orders_remaining_val, buy_orders_remaining_val_b);
        assert_eq!(
            open_buy_orders.len() - buy_orders_to_keep.len(),
            buy_hashes_to_cancel.len()
        );

        // Check case where there were no open orders at all
        let (buy_hashes_to_cancel, _, _, buy_append_new_to_head) =
            orders_to_cancel_spot(Vec::new(), new_buy_head_a, new_buy_tail_a, true);
        assert!(buy_append_new_to_head);
        assert_eq!(0, buy_hashes_to_cancel.len());
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
        assert_eq!(
            open_sell_orders.len() - sell_orders_to_keep.len(),
            sell_hashes_to_cancel.len()
        );

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
        assert_eq!(
            open_sell_orders.len() - sell_orders_to_keep.len(),
            sell_hashes_to_cancel.len()
        );

        // Check case where there were no open orders at all
        let (sell_hashes_to_cancel, _, _, sell_append_new_to_head) =
            orders_to_cancel_spot(Vec::new(), new_sell_head_a, new_sell_tail_a, false);
        assert!(sell_append_new_to_head);
        assert_eq!(0, sell_hashes_to_cancel.len());
    }
}
