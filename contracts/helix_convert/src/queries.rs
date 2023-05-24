use std::str::FromStr;
use cosmwasm_std::{Addr, Deps, Env, StdError, StdResult};

use injective_cosmwasm::{InjectiveQuerier, InjectiveQueryWrapper, MarketId, OrderSide, PriceLevel, SpotMarket};
use injective_math::utils::round_to_min_tick;
use injective_math::FPDecimal;
use crate::ContractError;
use crate::ContractError::CustomError;

use crate::helpers::counter_denom;
use crate::state::{CONFIG, read_swap_route};
use crate::types::{Config, FPCoin, StepExecutionEstimate};

pub fn estimate_swap_result(
    deps: Deps<InjectiveQueryWrapper>,
    env: Env,
    from_denom: String,
    quantity: FPDecimal,
    to_denom: String,
) -> StdResult<FPDecimal> {
    if quantity.is_zero() || quantity.is_negative() {
        return Err(StdError::generic_err("from_quantity must be positive"));
    }
    let route = read_swap_route(deps.storage, &from_denom, &to_denom)?;
    let steps = route.steps_from(&from_denom);
    let mut current_swap = FPCoin {
        amount: quantity,
        denom: from_denom,
    };
    for step in steps {
        let cur_swap = current_swap.clone();
        let swap_estimate = estimate_single_swap_execution(&deps, &env, &step,current_swap)?;
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
    env: &Env,
    market_id: &MarketId,
    balance_in: FPCoin,
) -> StdResult<StepExecutionEstimate> {
    let querier = InjectiveQuerier::new(&deps.querier);

    let market = querier
        .query_spot_market(market_id)?
        .market
        .expect("market should be available");
    deps.api.debug(&format!(
        "Estimating swap step for {} {} on market: {}, base: {}, quote: {}",
        balance_in.amount.clone(),
        balance_in.denom,
        market.ticker,
        market.base_denom,
        market.quote_denom,
    ));
    let config = CONFIG.load(deps.storage)?;
    let is_self_relayer = config.fee_recipient == env.contract.address;
    let fee_multiplier = querier
        .query_market_atomic_execution_fee_multiplier(market_id)?
        .multiplier;
    let fee_percent = market.taker_fee_rate * fee_multiplier * (FPDecimal::one() - effective_fee_discount_rate(&market, is_self_relayer));
    deps.api.debug(&format!(
        "market.taker_fee_rate: {}, multiplier: {}, final Fee percent: {}",
        market.taker_fee_rate, fee_multiplier, fee_percent,
    ));
    let is_buy = if &balance_in.denom == &market.quote_denom {
        true
    } else if &balance_in.denom == &market.base_denom {
        false
    } else {
        return Err(StdError::generic_err(
            "Invalid swap denom - neither base or quote",
        ));
    };
    deps.api.debug(&format!("Is buy: {}", is_buy));

    let (expected_quantity, worst_price) = if is_buy {
        estimate_execution_buy(deps, &env.contract.address, &market, balance_in.amount,fee_percent)?
    } else {
        estimate_execution_sell(deps, &querier, market_id, balance_in.amount, fee_percent)?
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
    deps: &Deps<InjectiveQueryWrapper>,
    contract_address: &Addr,
    market: &SpotMarket,
    amount: FPDecimal,
    fee: FPDecimal,
) -> StdResult<(FPDecimal, FPDecimal)> {
    let inj_querier =  InjectiveQuerier::new(&deps.querier);
    let available_funds = amount / (FPDecimal::one() + fee); // keep reserve for fee
    deps.api.debug(&format!("estimate_execution_buy: Fee: {fee}, To change: {amount}, available (after fee): {available_funds}"));
    let top_orders = find_minimum_orders(
        deps,
        &inj_querier
            .query_spot_market_orderbook(&market.market_id, OrderSide::Sell, None, Some(available_funds))?
            .sells_price_level,
        available_funds,
        |l| l.q * l.p,
    )?;
    let avg_price = avg_price(&top_orders);
    let expected_quantity = available_funds / avg_price;
    let worst_price = worst_price(&top_orders);

    // check if user funds + contract funds are enough to create order

    let required_funds = worst_price * expected_quantity;
    let funds_in_contract = deps.querier
        .query_balance(contract_address, &market.quote_denom)
        .expect("query own balance should not fail")
        .amount.into();
    if required_funds > funds_in_contract {
        Err(StdError::generic_err("Swap amount too high"))
    } else {
        Ok((expected_quantity, worst_price))
    }
}

fn estimate_execution_sell(
    deps: &Deps<InjectiveQueryWrapper>,
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    amount: FPDecimal,
    fee: FPDecimal,
) -> StdResult<(FPDecimal, FPDecimal)> {
    deps.api
        .debug(&format!("estimate_execution_sell: total: {amount}, will call query now"));
    let orders = &querier
        .query_spot_market_orderbook(market_id, OrderSide::Buy, Some(amount), None)?;
    deps.api.debug(&format!("estimate_execution_sell: orders sells: {}, buys: {}", orders.sells_price_level.len(), orders.buys_price_level.len()));
    let top_orders = find_minimum_orders(
        deps,
        &querier
            .query_spot_market_orderbook(market_id, OrderSide::Buy, Some(amount), None)?
            .buys_price_level,
        amount,
        |l| l.q,
    )?;
    let avg_price = avg_price(&top_orders);
    let expected_exchange_quantity = amount * avg_price;
    let expected_fee = expected_exchange_quantity * fee;
    let expected_quantity = expected_exchange_quantity - expected_fee;
    deps.api.debug(&format!("Sell exchange: {expected_exchange_quantity}, fee: {expected_fee}, total: {expected_quantity}"));
    let worst_price = worst_price(&top_orders);
    Ok((expected_quantity, worst_price))
}

pub fn find_minimum_orders(
    deps: &Deps<InjectiveQueryWrapper>,
    levels: &Vec<PriceLevel>,
    total: FPDecimal,
    calc: fn(&PriceLevel) -> FPDecimal,
) -> StdResult<Vec<PriceLevel>> {
    deps.api
        .debug(&format!("find_minimum_orders, total: {total}"));
    deps.api
        .debug(&format!("levels: {:?}", levels));
    let mut sum = FPDecimal::zero();
    let mut orders: Vec<PriceLevel> = Vec::new();
    for level in levels {
        let value = calc(level);
        deps.api
            .debug(&format!(
            "Adding level {}x{} value: {value}, sum so far: {sum}",
            level.p.clone(),
            level.q.clone()
        ));
        let order_to_add = if sum + value > total {
            let excess = value + sum - total;
            deps.api
                .debug(&format!("Value: {value}, excess value: {excess}, sum so far: {sum}"));
            PriceLevel {
                p: level.p,
                q: ((value - excess) / value) * level.q, // we only take a part of this price level
            }
        } else {
            level.clone() // take fully
        };
        deps.api
            .debug(&format!(
            "Added level {}x{}",
            order_to_add.p.clone(),
            order_to_add.q.clone()
        ));

        sum += value;
        orders.push(order_to_add);
        if sum >= total {
            break;
        }
    }
    if sum < total {
        deps.api
            .debug(&format!("Wanted: {total}, got: {sum}"));
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

fn effective_fee_discount_rate(market: &SpotMarket, is_self_relayer: bool) -> FPDecimal {
    if !is_self_relayer {
        FPDecimal::zero()
    } else {
        market.relayer_fee_share_rate
    }
}

#[cfg(test)]
mod tests {
    use injective_cosmwasm::inj_mock_deps;
    use crate::testing::test_utils::create_price_level;

    use super::*;

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

        let result = find_minimum_orders(&inj_mock_deps(|_|{}).as_ref(), &levels, FPDecimal::from(1000u128), |l| l.q);
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

        let result = find_minimum_orders(&inj_mock_deps(|_|{}).as_ref(), &levels, FPDecimal::from(800u128), |l| l.q);
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

        let result = find_minimum_orders(&inj_mock_deps(|_|{}).as_ref(), &levels, FPDecimal::from(450u128), |l| l.q);
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

        let result = find_minimum_orders(&inj_mock_deps(|_|{}).as_ref(), &buy_levels, FPDecimal::from(3450u128), |l| l.q * l.p);
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
