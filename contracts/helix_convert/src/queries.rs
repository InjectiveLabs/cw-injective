use cosmwasm_std::{Coin, Deps, StdError, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_cosmwasm::{
    InjectiveQuerier, InjectiveQueryWrapper, MarketId, OrderSide, PriceLevel,
};
use injective_math::FPDecimal;
use injective_math::utils::round_to_min_tick;

use crate::helpers::counter_denom;
use crate::state::read_swap_route;

struct ExecutionPrice {
    worst_price: FPDecimal,
    average_price: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
struct FPCoin {
    amount: FPDecimal,
    denom: String,
}

fn estimate_swap_result(
    deps: Deps<InjectiveQueryWrapper>,
    from_denom: String,
    quantity: FPDecimal,
    to_denom: String,
) -> StdResult<FPDecimal> {
    let route = read_swap_route(&deps, &from_denom, &to_denom)?;
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

struct StepExecutionEstimate {
    worst_price: FPDecimal,
    result_denom: String,
    result_quantity: FPDecimal,
}

fn estimate_single_swap_execution(
    deps: &Deps<InjectiveQueryWrapper>,
    market_id: &MarketId,
    swap: FPCoin,
) -> StdResult<StepExecutionEstimate> {
    let querier = InjectiveQuerier::new(&deps.querier);

    let market = querier
        .query_spot_market(market_id)?
        .market
        .expect("market should be available");
    println!("Swapping {} {} on market: {}", swap.amount.clone(), swap.denom.clone(), market.ticker);

    let fee_multiplier = querier
        .query_market_atomic_execution_fee_multiplier(market_id)?
        .multiplier;
    let fee_percent = market.taker_fee_rate * fee_multiplier;

    let (expected_quantity, worst_price) = if &swap.denom == &market.quote_denom {
        estimate_execution_buy(&querier, market_id, swap.amount, fee_percent)?
    } else if &swap.denom == &market.base_denom {
        estimate_execution_sell(&querier, market_id, swap.amount, fee_percent)?
    } else {
        return Err(StdError::generic_err(
            "Invalid swap denom - neither base or quote",
        ));
    };
    let rounded = round_to_min_tick(expected_quantity, if &swap.denom == &market.quote_denom {market.min_quantity_tick_size} else {market.min_price_tick_size});

    let new_denom = counter_denom(&market, &swap.denom)?;
    Ok(StepExecutionEstimate {
        worst_price,
        result_denom: new_denom.to_string(),
        result_quantity: rounded,
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
        println!("Adding level {}x{} value: {value}, sum so far: {sum}", level.p.clone(), level.q.clone());
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
        println!("Added level {}x{}", order_to_add.p.clone(), order_to_add.q.clone());

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
    use std::collections::HashMap;
    use std::str::FromStr;

    use injective_cosmwasm::{
        create_mock_spot_market, create_orderbook_response_handler, create_spot_market_handler,
        create_spot_multi_market_handler, Hash, inj_mock_deps, OwnedDepsExt, SpotMarket,
        TEST_MARKET_ID_1, TEST_MARKET_ID_2,
    };

    use crate::state::{store_swap_route, SwapRoute};

    use super::*;

    // Helper function to create a PriceLevel
    fn create_price_level(p: u128, q: u128) -> PriceLevel {
        PriceLevel {
            p: FPDecimal::from(p),
            q: FPDecimal::from(q),
        }
    }

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

    /// In this test we swap 1000 INJ to ETH, we assume avg price of INJ at 8 usdt and avg price of eth 2000 usdt
    #[test]
    fn test_calculate_swap_price() {
        let mut deps_binding = inj_mock_deps(|querier| {
            let mut markets = HashMap::new();
            markets.insert(
                MarketId::new(TEST_MARKET_ID_1).unwrap(),
                create_mock_spot_market("eth", 0),
            );
            markets.insert(
                MarketId::new(TEST_MARKET_ID_2).unwrap(),
                create_mock_spot_market("inj", 1),
            );
            querier.spot_market_response_handler = create_spot_multi_market_handler(markets);

            let mut orderbooks = HashMap::new();
            let eth_buy_orderbook = vec![
                PriceLevel {
                    p: 201000u128.into(),
                    q: FPDecimal::from_str("0.5").unwrap(),
                },
                PriceLevel {
                    p: 195000u128.into(),
                    q: FPDecimal::from_str("0.4").unwrap(),
                },
                PriceLevel {
                    p: 192000u128.into(),
                    q: FPDecimal::from_str("0.3").unwrap(),
                },
            ];
            orderbooks.insert(MarketId::new(TEST_MARKET_ID_1).unwrap(), eth_buy_orderbook);

            let inj_sell_orderbook = vec![
                PriceLevel {
                    p: 800u128.into(),
                    q: 80u128.into(),
                },
                PriceLevel {
                    p: 810u128.into(),
                    q: 80u128.into(),
                },
                PriceLevel {
                    p: 820u128.into(),
                    q: 80u128.into(),
                },
                PriceLevel {
                    p: 830u128.into(),
                    q: 80u128.into(),
                },
            ];
            orderbooks.insert(MarketId::new(TEST_MARKET_ID_2).unwrap(), inj_sell_orderbook);

            querier.spot_market_orderbook_response_handler =
                create_orderbook_response_handler(orderbooks);
        });

        let mut deps = deps_binding.as_mut_deps();

        let route = SwapRoute {
            steps: vec![TEST_MARKET_ID_1.into(), TEST_MARKET_ID_2.into()],
            denom_1: "eth".to_string(),
            denom_2: "inj".to_string(),
        };

        store_swap_route(&mut deps, route).unwrap();

        let amount_inj = estimate_swap_result(
            deps.as_ref(),
            "eth".to_string(),
            FPDecimal::from_str("1.2").unwrap(),
            "inj".to_string(),
        )
        .unwrap();
        assert_eq!(amount_inj, FPDecimal::from_str("287.97").unwrap(), "Wrong amount of INJ received"); // value rounded to min tick
        println!("Got {amount_inj} inj");
    }
}
