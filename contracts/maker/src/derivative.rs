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
    alloc_val_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    mut position_qty: Decimal,
    touch_head: bool,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Decimal) {
    let num_open_orders = Uint256::from_str(&orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let num_orders_to_open = state.order_density - num_open_orders;
    let val_per_order = alloc_val_for_new_orders / num_orders_to_open;
    let val_per_order = val_per_order * state.leverage;

    // All we need to do is create orders between the new head and tail
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
    alloc_val_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    position_qty: Decimal,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Vec<String>) {
    let (mut orders_to_open, position_qty) = base_deriv(
        new_head,
        orders_to_keep.first().unwrap().price,
        alloc_val_for_new_orders,
        orders_to_keep.clone(),
        position_qty,
        true,
        is_buy,
        state,
    );
    let additional_hashes_to_cancel = handle_reduce_only(orders_to_keep.clone(), position_qty, &mut orders_to_open, is_buy, state);
    (orders_to_open, additional_hashes_to_cancel)
}

pub fn head_to_tail_deriv(
    new_tail: Decimal,
    alloc_val_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    position_qty: Decimal,
    is_buy: bool,
    state: &State,
) -> (Vec<WrappedOrderResponse>, Vec<String>) {
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let additional_hashes_to_cancel = handle_reduce_only(orders_to_keep.clone(), position_qty, &mut orders_to_open, is_buy, state);
    let (mut orders_to_open_b, _) = base_deriv(
        orders_to_keep.last().unwrap().price,
        new_tail,
        alloc_val_for_new_orders,
        orders_to_keep,
        position_qty,
        false,
        is_buy,
        state,
    );
    orders_to_open.append(&mut orders_to_open_b);
    (orders_to_open, additional_hashes_to_cancel)
}

fn handle_reduce_only(
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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Decimal256 as Decimal, Uint256};
    use std::str::FromStr;
    use crate::{derivative::base_deriv, msg::WrappedOpenOrder, state::State, utils::div_dec};

    #[test]
    fn base_buy_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        let state = mock_state(leverage.to_string(), String::from("20"));
        for i in 0..100 {
            let qty = i * 100;
            base_test(
                Decimal::from_str("20").unwrap(),
                Decimal::from_str("9").unwrap(),
                Decimal::from_str("10000").unwrap(),
                Vec::new(),
                Decimal::from_str(&qty.to_string()).unwrap(),
                true,
                true,
                &state,
            );
            let orders_to_keep = mock_orders_to_keep(20, 15, true, leverage, 10000);
            base_test(
                Decimal::from_str("15").unwrap(),
                Decimal::from_str("10").unwrap(),
                Decimal::from_str("10000").unwrap(),
                orders_to_keep,
                Decimal::from_str(&qty.to_string()).unwrap(),
                true,
                true,
                &state,
            );
        }
    }
    #[test]
    fn base_sell_test() {
        let leverage = Decimal::from_str("2.5").unwrap();
        let state = mock_state(leverage.to_string(), String::from("20"));
        for i in 0..100 {
            let qty = i * 100;
            base_test(
                Decimal::from_str("9").unwrap(),
                Decimal::from_str("20").unwrap(),
                Decimal::from_str("10000").unwrap(),
                Vec::new(),
                Decimal::from_str(&qty.to_string()).unwrap(),
                true,
                false,
                &state,
            );
            let orders_to_keep = mock_orders_to_keep(10, 15, false, leverage, 10000);
            base_test(
                Decimal::from_str("10").unwrap(),
                Decimal::from_str("15").unwrap(),
                Decimal::from_str("10000").unwrap(),
                orders_to_keep,
                Decimal::from_str(&qty.to_string()).unwrap(),
                true,
                false,
                &state,
            );
        }
    }

    // Test Helpers
    fn mock_state(leverage: String, order_density: String) -> State {
        State {
            market_id: String::from(""),
            manager: String::from(""),
            fee_recipient: String::from(""),
            sub_account: String::from(""),
            risk_aversion: Decimal::from_str("0.2").unwrap(),
            order_density: Uint256::from_str(&order_density).unwrap(),
            active_capital_perct: Decimal::from_str("0.2").unwrap(),
            max_notional_position: Decimal::zero(),
            min_pnl: Decimal::zero(),
            manual_offset_perct: Decimal::zero(),
            tail_dist_to_head_bp: Decimal::from_str("300").unwrap(),
            head_chg_tol_bp: Decimal::zero(),
            max_dd: Decimal::one(),
            leverage: Decimal::from_str(&leverage).unwrap(),
            decimal_shift: Uint256::from_str("1000000").unwrap(),
            base_precision_shift: Uint256::from_str("1000").unwrap(),
        }
    }

    fn mock_orders_to_keep(head: i32, tail: i32, is_buy: bool, leverage: Decimal, total_val: i32) -> Vec<WrappedOpenOrder> {
        let order = WrappedOpenOrder {
            order_hash: String::from(""),
            is_buy,
            qty: Decimal::one(),
            price: Decimal::zero(),
            is_reduce_only: false,
        };
        let mut orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let val_per_order = Decimal::from_str(&(total_val / (tail - head).abs()).to_string()).unwrap();
        if is_buy {
            assert!(head > tail);
            for i in ((head)..(tail)).rev() {
                let mut o = order.clone();
                o.price = Decimal::from_str(&i.to_string()).unwrap();
                o.qty = div_dec(val_per_order, o.price) * leverage;
                orders_to_keep.push(o.clone());
            }
        } else {
            assert!(head < tail);
            for i in head..tail {
                let mut o = order.clone();
                o.price = Decimal::from_str(&i.to_string()).unwrap();
                o.qty = div_dec(val_per_order, o.price) * leverage;
                orders_to_keep.push(o.clone());
            }
        }
        orders_to_keep
    }

    fn base_test(
        new_head: Decimal,
        new_tail: Decimal,
        alloc_val_for_new_orders: Decimal,
        orders_to_keep: Vec<WrappedOpenOrder>,
        position_qty_og: Decimal,
        touch_head: bool,
        is_buy: bool,
        state: &State,
    ) {
        let max_tolerance = Decimal::from_str("0.0001").unwrap();
        let (new_orders, position_qty) = base_deriv(
            new_head,
            new_tail,
            alloc_val_for_new_orders,
            orders_to_keep.clone(),
            position_qty_og,
            touch_head,
            is_buy,
            state,
        );

        let num_open_orders = Uint256::from_str(&orders_to_keep.len().to_string()).unwrap();
        let num_orders_to_open = state.order_density - num_open_orders;
        let val_per_order = alloc_val_for_new_orders / num_orders_to_open;
        let val_per_order = val_per_order * state.leverage;
        let mut total_reduce_only_qty = Decimal::zero();
        let mut total_value = Decimal::zero();
        let mut num_diff_priced_orders = 0;
        for i in 0..new_orders.len() {
            if new_orders[i].is_reduce_only {
                total_reduce_only_qty = total_reduce_only_qty + new_orders[i].get_qty();
            }
            total_value = total_value + new_orders[i].get_val();
            if i > 0 {
                // Ensure that price is changing in the right direction
                if is_buy {
                    let lhs = new_orders[i - 1].get_price() > new_orders[i].get_price();
                    let rhs = new_orders[i - 1].get_val() + new_orders[i].get_val() + (max_tolerance * val_per_order) >= val_per_order;
                    assert!(lhs || rhs);
                } else {
                    let lhs = new_orders[i - 1].get_price() < new_orders[i].get_price();
                    let rhs = new_orders[i - 1].get_val() + new_orders[i].get_val() + (max_tolerance * val_per_order) >= val_per_order;
                    assert!(lhs || rhs);
                }
                if new_orders[i - 1].price != new_orders[i].price {
                    num_diff_priced_orders += 1;
                }
                // Ensure that the notional val of orders is consistent
                let lhs = new_orders[i].get_val() + (max_tolerance * val_per_order) >= val_per_order;
                let mhs = new_orders[i - 1].get_val() + new_orders[i].get_val() + (max_tolerance * val_per_order) >= val_per_order;
                let rhs = new_orders[i - 1].get_val() + new_orders[i].get_val() <= val_per_order + (max_tolerance * val_per_order);
                assert!(lhs || mhs || rhs);
            } else {
                num_diff_priced_orders += 1;
            }
        }

        // Ensure we never have too many orders
        assert_eq!(
            num_diff_priced_orders + orders_to_keep.len(),
            state.order_density.to_string().parse::<usize>().unwrap()
        );

        // Esure that we tried the best we could to reduce the position
        assert!(position_qty_og >= total_reduce_only_qty);
        if position_qty == Decimal::zero() {
            assert!(position_qty + (position_qty_og * max_tolerance) >= position_qty_og - total_reduce_only_qty);
        } else {
            for i in 0..new_orders.len() {
                assert!(new_orders[i].is_reduce_only);
            }
        }

        // Ensure that we used all possible inventory within a tolerance
        assert!(div_dec(total_value, state.leverage) + (alloc_val_for_new_orders * max_tolerance) >= alloc_val_for_new_orders);
    }
}
