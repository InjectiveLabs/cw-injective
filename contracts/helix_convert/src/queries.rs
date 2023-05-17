use cosmwasm_std::{Deps, StdError, StdResult};



use injective_cosmwasm::{
    InjectiveQuerier, InjectiveQueryWrapper, MarketId, OrderSide, PriceLevel,
};
use injective_math::utils::round_to_min_tick;
use injective_math::FPDecimal;

use crate::helpers::counter_denom;
use crate::state::read_swap_route;
use crate::types::{FPCoin, StepExecutionEstimate};


pub fn estimate_swap_result(
    deps: Deps<InjectiveQueryWrapper>,
    from_denom: String,
    quantity: FPDecimal,
    to_denom: String,
) -> StdResult<FPDecimal> {
    let route = read_swap_route(deps.storage, &from_denom, &to_denom)?;
    let steps = route.steps_from(&from_denom);
    let mut current_swap = FPCoin {
        amount: quantity,
        denom: from_denom,
    };
    for step in steps {
        let cur_swap = current_swap.clone();
        let swap_estimate = estimate_single_swap_execution(&deps, &step, current_swap)?;
        let new_amount = swap_estimate.result_quantity;
        println!(
            "Exchanged {}{} into {}{}",
            &cur_swap.amount,
            &cur_swap.denom,
            &swap_estimate.result_quantity,
            &swap_estimate.result_denom
        );
        current_swap = FPCoin {
            amount: new_amount,
            denom: swap_estimate.result_denom,
        }
    }
    Ok(current_swap.amount)
}

pub fn estimate_single_swap_execution(
    deps: &Deps<InjectiveQueryWrapper>,
    market_id: &MarketId,
    balance_in: FPCoin,
) -> StdResult<StepExecutionEstimate> {
    let querier = InjectiveQuerier::new(&deps.querier);

    let market = querier
        .query_spot_market(market_id)?
        .market
        .expect("market should be available");
    println!(
        "Swapping {} {} on market: {}",
        balance_in.amount.clone(),
        balance_in.denom,
        market.ticker
    );

    let fee_multiplier = querier
        .query_market_atomic_execution_fee_multiplier(market_id)?
        .multiplier;
    let fee_percent = market.taker_fee_rate * fee_multiplier;
    let is_buy = if &balance_in.denom == &market.quote_denom {
        true
    } else if &balance_in.denom == &market.base_denom {
        false
    } else {
        return Err(StdError::generic_err(
            "Invalid swap denom - neither base or quote",
        ));
    };

    let (expected_quantity, worst_price) = if is_buy {
        estimate_execution_buy(&querier, market_id, balance_in.amount, fee_percent)?
    } else {
        estimate_execution_sell(&querier, market_id, balance_in.amount, fee_percent)?
    };
    let rounded = round_to_min_tick(
        expected_quantity,
        if is_buy {
            market.min_quantity_tick_size
        } else {
            market.min_price_tick_size
        },
    );

    let new_denom = counter_denom(&market, &balance_in.denom)?;
    Ok(StepExecutionEstimate {
        worst_price,
        result_denom: new_denom.to_string(),
        result_quantity: rounded,
        is_buy_order: is_buy,
    })
}

fn estimate_execution_buy(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    amount: FPDecimal,
    fee: FPDecimal,
) -> StdResult<(FPDecimal, FPDecimal)> {
    let available_funds = amount / (FPDecimal::one() + fee); // keep reserve for fee
    println!("estimate_execution_buy: Fee: {fee}, To change: {amount}, available (after fee): {available_funds}");
    let top_orders = find_minimum_orders(
        &querier
            .query_spot_market_orderbook(market_id, OrderSide::Sell, Some(available_funds), None)?
            .sells_price_level,
        available_funds,
        |l| l.q * l.p,
    )?;
    let avg_price = avg_price(&top_orders);
    let expected_quantity = available_funds / avg_price; // TODO check rounding

    let worst_price = worst_price(&top_orders);

    Ok((expected_quantity, worst_price))
}

fn estimate_execution_sell(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    amount: FPDecimal,
    fee: FPDecimal,
) -> StdResult<(FPDecimal, FPDecimal)> {
    println!("estimate_execution_sell: total: {amount}");
    let top_orders = find_minimum_orders(
        &querier
            .query_spot_market_orderbook(market_id, OrderSide::Buy, None, Some(amount))?
            .buys_price_level,
        amount,
        |l| l.q,
    )?;
    let avg_price = avg_price(&top_orders);
    let expected_exchange_quantity = amount * avg_price;
    let expected_fee = expected_exchange_quantity * fee;
    let expected_quantity = expected_exchange_quantity - expected_fee;
    println!("Sell exchange: {expected_exchange_quantity}, fee: {expected_fee}, total: {expected_quantity}");
    let worst_price = worst_price(&top_orders);
    Ok((expected_quantity, worst_price))
}

pub fn find_minimum_orders(
    levels: &Vec<PriceLevel>,
    total: FPDecimal,
    calc: fn(&PriceLevel) -> FPDecimal,
) -> StdResult<Vec<PriceLevel>> {
    println!("find_minimum_orders, total: {}", total.clone());
    let mut sum = FPDecimal::zero();
    let mut orders: Vec<PriceLevel> = Vec::new();
    for level in levels {
        let value = calc(level);
        println!(
            "Adding level {}x{} value: {value}, sum so far: {sum}",
            level.p.clone(),
            level.q.clone()
        );
        let order_to_add = if sum + value > total {
            let excess = value + sum - total;
            println!("Value: {value}, excess value: {excess}, sum so far: {sum}");
            PriceLevel {
                p: level.p,
                q: ((value - excess) / value) * level.q, // we only take a part of this price level
            }
        } else {
            level.clone() // take fully
        };
        println!(
            "Added level {}x{}",
            order_to_add.p.clone(),
            order_to_add.q.clone()
        );

        sum += value;
        orders.push(order_to_add);
        if sum >= total {
            break;
        }
    }
    if sum < total {
        Err(StdError::generic_err(
            "Not enough liquidity to fulfill order",
        ))
    } else {
        Ok(orders)
    }
}

fn avg_price(levels: &Vec<PriceLevel>) -> FPDecimal {
    let (total_quantity, total_notional) = levels
        .iter()
        .fold((FPDecimal::zero(), FPDecimal::zero()), |acc, pl| {
            (acc.0 + pl.q, acc.1 + pl.p * pl.q)
        });
    total_notional / total_quantity
}

fn worst_price(levels: &Vec<PriceLevel>) -> FPDecimal {
    levels.last().unwrap().p // assume there's at least one element
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::test_utils::create_price_level;

    #[test]
    fn test_avg_price_simple() {
        let levels = vec![
            create_price_level(1, 200),
            create_price_level(2, 200),
            create_price_level(3, 200),
        ];

        let avg = avg_price(&levels);
        assert_eq!(avg, FPDecimal::from(2u128));
    }

    #[test]
    fn test_avg_price_simple_2() {
        let levels = vec![
            create_price_level(1, 300),
            create_price_level(2, 200),
            create_price_level(3, 100),
        ];

        let avg = avg_price(&levels);
        assert_eq!(avg, FPDecimal::from(1000u128) / FPDecimal::from(600u128));
    }

    #[test]
    fn test_worst_price() {
        let levels = vec![
            create_price_level(1, 100),
            create_price_level(2, 200),
            create_price_level(3, 300),
        ];

        let worst = worst_price(&levels);
        assert_eq!(worst, FPDecimal::from(3u128));
    }

    #[test]
    fn test_find_minimum_orders_not_enough_liquidity() {
        let levels = vec![create_price_level(1, 100), create_price_level(2, 200)];

        let result = find_minimum_orders(&levels, FPDecimal::from(1000u128), |l| l.q);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            StdError::generic_err("Not enough liquidity to fulfill order")
        );
    }

    #[test]
    fn test_find_minimum_orders_with_gaps() {
        let levels = vec![
            create_price_level(1, 100),
            create_price_level(3, 300),
            create_price_level(5, 500),
        ];

        let result = find_minimum_orders(&levels, FPDecimal::from(800u128), |l| l.q);
        assert!(result.is_ok());
        let min_orders = result.unwrap();
        assert_eq!(min_orders.len(), 3);
        assert_eq!(min_orders[0].p, FPDecimal::from(1u128));
        assert_eq!(min_orders[1].p, FPDecimal::from(3u128));
        assert_eq!(min_orders[2].p, FPDecimal::from(5u128));
    }

    #[test]
    fn test_find_minimum_buy_orders_not_consuming_fully() {
        let levels = vec![
            create_price_level(1, 100),
            create_price_level(3, 300),
            create_price_level(5, 500),
        ];

        let result = find_minimum_orders(&levels, FPDecimal::from(450u128), |l| l.q);
        assert!(result.is_ok());
        let min_orders = result.unwrap();
        assert_eq!(min_orders.len(), 3);
        assert_eq!(min_orders[0].p, FPDecimal::from(1u128));
        assert_eq!(min_orders[0].q, FPDecimal::from(100u128));
        assert_eq!(min_orders[1].p, FPDecimal::from(3u128));
        assert_eq!(min_orders[1].q, FPDecimal::from(300u128));
        assert_eq!(min_orders[2].p, FPDecimal::from(5u128));
        assert_eq!(min_orders[2].q, FPDecimal::from(50u128));
    }

    #[test]
    fn test_find_minimum_sell_orders_not_consuming_fully() {
        let buy_levels = vec![
            create_price_level(5, 500),
            create_price_level(3, 300),
            create_price_level(1, 100),
        ];

        let result = find_minimum_orders(&buy_levels, FPDecimal::from(3450u128), |l| l.q * l.p);
        assert!(result.is_ok());
        let min_orders = result.unwrap();
        assert_eq!(min_orders.len(), 3);
        assert_eq!(min_orders[0].p, FPDecimal::from(5u128));
        assert_eq!(min_orders[0].q, FPDecimal::from(500u128));
        assert_eq!(min_orders[1].p, FPDecimal::from(3u128));
        assert_eq!(min_orders[1].q, FPDecimal::from(300u128));
        assert_eq!(min_orders[2].p, FPDecimal::from(1u128));
        assert_eq!(min_orders[2].q, FPDecimal::from(50u128));
    }
}
