use crate::{
    msg::{WrappedOpenOrder, WrappedOrderResponse},
    state::State,
    utils::{div_dec, div_int, sub_abs},
};
use cosmwasm_std::{Decimal256 as Decimal, Uint256};
use std::str::FromStr;

/// Calculates the inventory imbalance from 50/50 target balance.
/// # Arguments
/// * `inv_base_val` - The notional value of all base assets
/// * `inv_val` - The total notional value of all assets
/// # Returns
/// * `inv_imbalance` - The inventory imbalance parameter
/// * `imbal_is_long` - True if the imbalance is skewed in favor of the base asset
pub fn inv_imbalance_spot(inv_base_val: Decimal, inv_val: Decimal) -> (Decimal, bool) {
    let half_inv_val = div_int(inv_val, Uint256::from_str("2").unwrap());
    let inv_imbalance = div_dec(sub_abs(inv_base_val, half_inv_val), inv_val);
    (inv_imbalance, inv_base_val > half_inv_val)
}

/// Determines the new orders that should be placed between the new head/tail. Ensures
/// that the notional value of all open orders will be equal to the percent of active
/// capital defined upon instantiation.
/// # Arguments
/// * `new_head` - The new head (closest to the reservation price)
/// * `new_tail` - The new tail (farthest from the reservation price)
/// * `inv_val` - The total notional value of our inventory
/// * `orders_to_keep` - A list of open orders that we are going to keep on the book
/// * `orders_remaining_val` - An aggregation of the total notional value of orders_to_keep
/// * `is_buy` - True if all open_orders are buy. False if they are all sell
/// * `state` - Contract state
/// # Returns
/// * `orders_to_open` - A list of all the new orders that we would like to place
pub fn create_new_orders_spot(
    new_head: Decimal,
    new_tail: Decimal,
    alloc_val_for_new_orders: Decimal,
    orders_to_keep: Vec<WrappedOpenOrder>,
    append_to_new_head: bool,
    is_buy: bool,
    state: &State,
) -> Vec<WrappedOrderResponse> {
    let num_open_orders = Uint256::from_str(&orders_to_keep.len().to_string()).unwrap();
    let mut orders_to_open: Vec<WrappedOrderResponse> = Vec::new();
    let num_orders_to_open = state.order_density - num_open_orders;
    let val_per_order = alloc_val_for_new_orders / num_orders_to_open;

    if orders_to_keep.len() == 0 {
        // If we have no orders remaining after cancellation, all we need to do is create orders
        // between the new head and tail
        let price_dist = sub_abs(new_head, new_tail);
        let price_step = div_int(price_dist, num_orders_to_open);
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        let mut current_price = new_head;
        for _ in 0..num_orders_to_open {
            let qty = div_dec(val_per_order, current_price);
            let new_order = WrappedOrderResponse::new(state, current_price, qty, is_buy, false);
            orders_to_open.push(new_order);
            current_price = if is_buy {
                current_price - price_step
            } else {
                current_price + price_step
            }
        }
    } else if num_open_orders < state.order_density {
        // If we have some orders remaining but room for new ones, we need to create new orders in
        // the gap that will be created after we cancel
        let price_dist = if append_to_new_head {
            sub_abs(new_head, orders_to_keep.first().unwrap().price)
        } else {
            sub_abs(orders_to_keep.last().unwrap().price, new_tail)
        };
        let price_step = div_int(price_dist, num_orders_to_open);
        let num_orders_to_open = num_orders_to_open.to_string().parse::<i32>().unwrap();
        if append_to_new_head {
            // We need to create new orders in the price range between the new head and the start of
            // the orders_to_keep block
            let mut current_price = new_head;
            for _ in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                let new_order = WrappedOrderResponse::new(state, current_price, qty, is_buy, false);
                orders_to_open.push(new_order);
                current_price = if is_buy {
                    current_price - price_step
                } else {
                    current_price + price_step
                }
            }
        } else {
            // We need to create new orders in the price range between the end of
            // the orders_to_keep block and the new tail
            let mut current_price = if is_buy {
                orders_to_keep.last().unwrap().price - price_step
            } else {
                orders_to_keep.last().unwrap().price + price_step
            };
            for _ in 0..num_orders_to_open {
                let qty = div_dec(val_per_order, current_price);
                let new_order = WrappedOrderResponse::new(state, current_price, qty, is_buy, false);
                orders_to_open.push(new_order);
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
        spot::create_new_orders_spot,
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
            is_reduce_only: false,
        };
        let inv_val = Decimal::from_str("1000000").unwrap();

        // Check when there are no open orders
        let new_buy_head = Decimal::from_str("10").unwrap();
        let new_buy_tail = Decimal::from_str("1").unwrap();
        let buy_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let buy_orders_remaining_val = Decimal::zero();
        let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - buy_orders_remaining_val;
        let buy_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_buy_head,
            new_buy_tail,
            alloc_val_for_new_orders,
            buy_orders_to_keep,
            buy_append_new_to_head,
            true,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() > orders[i].get_price())
            }
            assert!(orders[i].is_buy);
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        assert_eq!(new_buy_head, orders.first().unwrap().get_price());
        assert!(new_buy_tail <= orders.last().unwrap().get_price());
        assert_eq!(Uint256::from_str(&orders.len().to_string()).unwrap(), state.order_density);
        let expected_notional_value = div_dec(state.active_capital_perct * inv_val, Decimal::from_str("2").unwrap());
        assert!(sub_abs(expected_notional_value, total_notional_val) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);

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
        let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - buy_orders_remaining_val;
        let buy_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_buy_head,
            new_buy_tail,
            alloc_val_for_new_orders,
            buy_orders_to_keep.clone(),
            buy_append_new_to_head,
            true,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() > orders[i].get_price())
            }
            assert!(orders[i].is_buy);
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
        let expected_notional_value = div_dec(state.active_capital_perct * inv_val, Decimal::from_str("2").unwrap());
        assert!(sub_abs(expected_notional_value, total_notional_val) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);

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
        let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - buy_orders_remaining_val;
        let orders = create_new_orders_spot(
            new_buy_head,
            new_buy_tail,
            alloc_val_for_new_orders,
            buy_orders_to_keep.clone(),
            buy_append_new_to_head,
            true,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() > orders[i].get_price())
            }
            assert!(orders[i].is_buy);
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
        let expected_notional_value = div_dec(state.active_capital_perct * inv_val, Decimal::from_str("2").unwrap());
        assert!(sub_abs(expected_notional_value, total_notional_val) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);
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
            is_reduce_only: false,
        };
        let inv_val = Decimal::from_str("1000000").unwrap();

        // Check when there are no open orders
        let new_sell_head = Decimal::from_str("1").unwrap();
        let new_sell_tail = Decimal::from_str("10").unwrap();
        let sell_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let sell_orders_remaining_val = Decimal::zero();
        let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - sell_orders_remaining_val;
        let sell_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_sell_head,
            new_sell_tail,
            alloc_val_for_new_orders,
            sell_orders_to_keep,
            sell_append_new_to_head,
            false,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() < orders[i].get_price())
            }
            assert!(!orders[i].is_buy);
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        assert_eq!(new_sell_head, orders.first().unwrap().get_price());
        assert!(new_sell_tail >= orders.last().unwrap().get_price());
        assert_eq!(Uint256::from_str(&orders.len().to_string()).unwrap(), state.order_density);
        let expected_notional_value = div_dec(state.active_capital_perct * inv_val, Decimal::from_str("2").unwrap());
        assert!(sub_abs(expected_notional_value, total_notional_val) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);

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
        let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - sell_orders_remaining_val;
        let sell_append_new_to_head = true;
        let orders = create_new_orders_spot(
            new_sell_head,
            new_sell_tail,
            alloc_val_for_new_orders,
            sell_orders_to_keep.clone(),
            sell_append_new_to_head,
            false,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() < orders[i].get_price())
            }
            assert!(!orders[i].is_buy);
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
        let expected_notional_value = div_dec(state.active_capital_perct * inv_val, Decimal::from_str("2").unwrap());
        assert!(sub_abs(expected_notional_value, total_notional_val) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);

        // Check when there are open orders and we want to add to the last order we kept
        let mut sell_orders_to_keep: Vec<WrappedOpenOrder> = Vec::new();
        let mut sell_orders_remaining_val = Decimal::zero();
        for i in (new_sell_head_i - 1)..(new_sell_tail_i - 1) {
            let mut o = order.clone();
            o.price = Decimal::from_str(&i.to_string()).unwrap();
            sell_orders_remaining_val = sell_orders_remaining_val + (o.price * o.qty);
            sell_orders_to_keep.push(o.clone());
        }
        let alloc_val_for_new_orders = div_dec(inv_val * state.active_capital_perct, Decimal::from_str("2").unwrap()) - sell_orders_remaining_val;
        let sell_append_new_to_head = false;
        let orders = create_new_orders_spot(
            new_sell_head,
            new_sell_tail,
            alloc_val_for_new_orders,
            sell_orders_to_keep.clone(),
            sell_append_new_to_head,
            false,
            &state,
        );

        let mut total_notional_val = Decimal::zero();
        for i in 0..orders.len() {
            if i > 0 {
                assert!(orders[i - 1].get_price() < orders[i].get_price())
            }
            assert!(!orders[i].is_buy);
            total_notional_val = total_notional_val + orders[i].get_val();
        }
        sell_orders_to_keep.iter().for_each(|o| {
            total_notional_val = total_notional_val + o.price;
        });
        assert!(new_sell_head <= orders.first().unwrap().get_price());
        assert!(sub_abs(new_sell_tail, orders.last().unwrap().get_price()) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);
        assert_eq!(
            Uint256::from_str(&(orders.len() + sell_orders_to_keep.len()).to_string()).unwrap(),
            state.order_density
        );
        let expected_notional_value = div_dec(state.active_capital_perct * inv_val, Decimal::from_str("2").unwrap());
        assert!(sub_abs(expected_notional_value, total_notional_val) < Decimal::from_str("0.00001").unwrap() * expected_notional_value);
    }
}
