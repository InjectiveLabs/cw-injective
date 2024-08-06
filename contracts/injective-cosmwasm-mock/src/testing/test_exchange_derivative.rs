use crate::{
    msg::{ExecuteMsg, QueryMsg},
    utils::{
        add_derivative_order_as, add_derivative_orders, add_perp_initial_liquidity, dec_to_proto, execute_all_authorizations,
        get_initial_perp_liquidity_orders_vector, get_perpetual_market_id, human_to_dec, scale_price_quantity_perp_market,
        scale_price_quantity_perp_market_dec, ExchangeType, HumanOrder, Setup, BASE_DENOM, QUOTE_DECIMALS, QUOTE_DENOM,
    },
};
use cosmwasm_std::Addr;
use injective_cosmwasm::{
    checked_address_to_subaccount_id, exchange::response::QueryOrderbookResponse, CancellationStrategy, DerivativeMarketResponse, MarketId,
    MarketMidPriceAndTOBResponse, PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, PriceLevel,
    SubaccountEffectivePositionInMarketResponse, SubaccountId, SubaccountPositionInMarketResponse, TraderDerivativeOrdersResponse,
    TrimmedDerivativeLimitOrder,
};
use injective_math::FPDecimal;
use injective_test_tube::{
    injective_cosmwasm::get_default_subaccount_id_for_checked_address,
    injective_std::types::injective::exchange::v1beta1::{MsgInstantPerpetualMarketLaunch, OrderType},
    Account, Exchange, Module, Wasm,
};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_perpetual_market_info() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);

    let ticker = "INJ/USDT".to_string();
    let derivative_market_id = get_perpetual_market_id(&exchange, ticker.to_owned());
    let market_id = MarketId::new(derivative_market_id.clone()).unwrap();
    let query_msg = QueryMsg::TestPerpetualMarketInfo {
        market_id: market_id.clone(),
    };
    let res: PerpetualMarketInfoResponse = wasm.query(&env.contract_address, &query_msg).unwrap();

    assert!(res.info.is_some());
    let market_info = res.info.clone().unwrap();
    assert_eq!(market_info.market_id, market_id);
    assert_eq!(market_info.funding_interval, 3600i64);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_derivative_market() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let ticker = "INJ/USDT".to_string();
    let initial_margin_ratio = FPDecimal::must_from_str("0.195");
    let maintenance_margin_ratio = FPDecimal::must_from_str("0.05");
    let min_price_tick_size = FPDecimal::must_from_str("1000.0");
    let min_quantity_tick_size = FPDecimal::must_from_str("1000000000000000");
    let quote_denom = QUOTE_DENOM.to_string();
    let maker_fee_rate = FPDecimal::ZERO;
    let taker_fee_rate = FPDecimal::ZERO;

    exchange
        .instant_perpetual_market_launch(
            MsgInstantPerpetualMarketLaunch {
                sender: env.signer.address(),
                ticker: ticker.to_owned(),
                quote_denom: quote_denom.to_owned(),
                oracle_base: BASE_DENOM.to_owned(),
                oracle_quote: quote_denom.to_owned(),
                oracle_scale_factor: 6u32,
                oracle_type: 2i32,
                maker_fee_rate: dec_to_proto(maker_fee_rate).to_string(),
                taker_fee_rate: dec_to_proto(taker_fee_rate),
                initial_margin_ratio: dec_to_proto(initial_margin_ratio),
                maintenance_margin_ratio: dec_to_proto(maintenance_margin_ratio),
                min_price_tick_size: dec_to_proto(min_price_tick_size),
                min_quantity_tick_size: dec_to_proto(min_quantity_tick_size),
                min_notional: "1".to_string(),
            },
            &env.signer,
        )
        .unwrap();

    let derivative_market_id = get_perpetual_market_id(&exchange, ticker.to_owned());

    let market_id = MarketId::new(derivative_market_id.clone()).unwrap();
    let query_msg = QueryMsg::TestDerivativeMarketQuery { market_id };
    let res: DerivativeMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();

    let response_market = res.market.unwrap().market.unwrap();
    assert_eq!(response_market.market_id.as_str(), derivative_market_id);
    assert_eq!(response_market.ticker, ticker);
    assert_eq!(response_market.quote_denom, quote_denom);
    assert_eq!(response_market.min_price_tick_size, min_price_tick_size);
    assert_eq!(response_market.min_quantity_tick_size, min_quantity_tick_size);
    assert_eq!(response_market.maker_fee_rate, maker_fee_rate);
    assert_eq!(response_market.taker_fee_rate, taker_fee_rate);
    assert_eq!(response_market.initial_margin_ratio, initial_margin_ratio);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_effective_subaccount_position() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_perp_initial_liquidity(&env.app, market_id.clone());

    let (price, quantity, margin) = scale_price_quantity_perp_market("9.7", "1", "2", &QUOTE_DECIMALS);

    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(env.users[1].account.address()))
        .as_str()
        .to_string();

    add_derivative_order_as(
        &env.app,
        market_id.to_owned(),
        &env.users[1].account,
        price,
        quantity,
        OrderType::Sell,
        margin,
    );

    let query_msg = QueryMsg::TestEffectiveSubaccountPosition {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id.to_owned()).unwrap(),
    };
    let res: SubaccountEffectivePositionInMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(res.state.is_some());
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_vanilla_subaccount_position() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_perp_initial_liquidity(&env.app, market_id.clone());

    let (price, quantity, margin) = scale_price_quantity_perp_market("9.7", "1", "2", &QUOTE_DECIMALS);
    let trader = &env.users[1];
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.account.address()))
        .as_str()
        .to_string();

    add_derivative_order_as(
        &env.app,
        market_id.to_owned(),
        &env.users[1].account,
        price,
        quantity,
        OrderType::Sell,
        margin,
    );

    let query_msg = QueryMsg::TestVanillaSubaccountPosition {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id.to_owned()).unwrap(),
    };
    let res: SubaccountPositionInMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(res.state.is_some());

    let liquidity_orders: Vec<HumanOrder> = vec![HumanOrder {
        price: "9.7".to_string(),
        quantity: "10".to_string(),
        order_type: OrderType::Sell,
    }];
    add_derivative_orders(&env.app, market_id.clone(), liquidity_orders.to_owned(), None);

    let _res: SubaccountPositionInMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_derivative_orders() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let (price, quantity, margin) = scale_price_quantity_perp_market("10.1", "1", "2", &QUOTE_DECIMALS);
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(env.users[1].account.address()))
        .as_str()
        .to_string();

    add_derivative_order_as(
        &env.app,
        market_id.to_owned(),
        &env.users[1].account,
        price,
        quantity,
        OrderType::Sell,
        margin,
    );

    let query_msg = QueryMsg::TestTraderDerivativeOrders {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id).unwrap(),
    };
    let res: TraderDerivativeOrdersResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(res.orders.is_some());
    let orders = res.orders.clone().unwrap();
    assert_eq!(orders.len(), 1);
    let expected_order = TrimmedDerivativeLimitOrder {
        price: human_to_dec("10.1", QUOTE_DECIMALS),
        quantity: FPDecimal::must_from_str("1"),
        margin: human_to_dec("20.2", QUOTE_DECIMALS),
        fillable: FPDecimal::must_from_str("1"),
        isBuy: false,
        order_hash: "".to_string(),
    };
    assert_eq!(orders[0].price, expected_order.price);
    assert_eq!(orders[0].quantity, expected_order.quantity);
    assert_eq!(orders[0].fillable, expected_order.fillable);
    assert_eq!(orders[0].isBuy, expected_order.isBuy);
    assert_eq!(orders[0].margin, expected_order.margin);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_derivative_orders_to_cancel_up_to_amount() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(env.users[1].account.address()))
        .as_str()
        .to_string();

    let (price, quantity, margin) = scale_price_quantity_perp_market("10.1", "1", "2", &QUOTE_DECIMALS);
    add_derivative_order_as(
        &env.app,
        market_id.to_owned(),
        &env.users[1].account,
        price,
        quantity,
        OrderType::Sell,
        margin,
    );
    let (price, quantity, margin) = scale_price_quantity_perp_market("12", "1", "2", &QUOTE_DECIMALS);
    add_derivative_order_as(
        &env.app,
        market_id.to_owned(),
        &env.users[1].account,
        price,
        quantity,
        OrderType::Sell,
        margin,
    );
    let query_msg_derivative = QueryMsg::TestTraderDerivativeOrders {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id.to_owned()).unwrap(),
    };
    let res: TraderDerivativeOrdersResponse = wasm.query(&env.contract_address, &query_msg_derivative).unwrap();
    assert!(res.orders.is_some());
    let orders = res.orders.clone().unwrap();
    assert_eq!(orders.len(), 2);
    let query_derivative_order_cancel_msg = QueryMsg::TestDerivativeOrdersToCancelUpToAmount {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id).unwrap(),
        quote_amount: human_to_dec("0.2", QUOTE_DECIMALS),
        strategy: CancellationStrategy::UnspecifiedOrder,
        reference_price: None,
    };
    let res: TraderDerivativeOrdersResponse = wasm.query(&env.contract_address, &query_derivative_order_cancel_msg).unwrap();
    assert!(res.orders.is_some());
    let orders = res.orders.clone().unwrap();
    assert_eq!(orders.len(), 1);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_perpetual_market_funding() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let ticker = "INJ/USDT".to_string();
    let derivative_market_id = get_perpetual_market_id(&exchange, ticker.to_owned());
    let market_id = MarketId::new(derivative_market_id.clone()).unwrap();
    let query_msg = QueryMsg::TestPerpetualMarketFunding {
        market_id: market_id.clone(),
    };
    let res: PerpetualMarketFundingResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(res.state.is_some());
    let state = res.state.unwrap();
    assert_eq!(state.cumulative_funding, FPDecimal::ZERO);
    assert_eq!(state.cumulative_price, FPDecimal::ZERO);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_derivative_market_mid_price_and_tob() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_perp_initial_liquidity(&env.app, market_id.to_owned());

    let query_msg = QueryMsg::TestDerivativeMarketMidPriceAndTob {
        market_id: MarketId::new(market_id.clone()).unwrap(),
    };

    let res: MarketMidPriceAndTOBResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.mid_price, Some(human_to_dec("10", QUOTE_DECIMALS)));
    assert_eq!(res.best_buy_price, Some(human_to_dec("9.9", QUOTE_DECIMALS)));
    assert_eq!(res.best_sell_price, Some(human_to_dec("10.1", QUOTE_DECIMALS)));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_derivative_market_orderbook() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders = get_initial_perp_liquidity_orders_vector();
    add_derivative_orders(&env.app, market_id.clone(), liquidity_orders.to_owned(), None);

    let query_msg = QueryMsg::TestDerivativeMarketOrderbook {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        limit_cumulative_notional: FPDecimal::MAX,
    };
    let res: QueryOrderbookResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let buys_price_level = res.buys_price_level;
    let sells_price_level = res.sells_price_level;
    assert_eq!(buys_price_level.len(), 2);
    assert_eq!(sells_price_level.len(), 2);
    assert_eq!(
        buys_price_level[0],
        PriceLevel {
            p: human_to_dec(liquidity_orders[2].price.as_str(), QUOTE_DECIMALS),
            q: FPDecimal::must_from_str(liquidity_orders[2].quantity.as_str()),
        }
    );
    assert_eq!(
        buys_price_level[1],
        PriceLevel {
            p: human_to_dec(liquidity_orders[3].price.as_str(), QUOTE_DECIMALS),
            q: FPDecimal::must_from_str(liquidity_orders[3].quantity.as_str()),
        }
    );
    assert_eq!(
        sells_price_level[0],
        PriceLevel {
            p: human_to_dec(liquidity_orders[1].price.as_str(), QUOTE_DECIMALS),
            q: FPDecimal::must_from_str(liquidity_orders[1].quantity.as_str()),
        }
    );
    assert_eq!(
        sells_price_level[1],
        PriceLevel {
            p: human_to_dec(liquidity_orders[0].price.as_str(), QUOTE_DECIMALS),
            q: FPDecimal::must_from_str(liquidity_orders[0].quantity.as_str()),
        }
    );
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_transient_derivative_orders() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();
    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(env.users[0].account.address()), 0u32);
    execute_all_authorizations(&env.app, &env.users[0].account, env.contract_address.clone());
    add_perp_initial_liquidity(&env.app, market_id.clone());
    let (price, quantity, margin) = scale_price_quantity_perp_market("9.7", "0.5", "2", &QUOTE_DECIMALS);
    add_derivative_order_as(
        &env.app,
        market_id.to_owned(),
        &env.users[0].account,
        price,
        quantity,
        OrderType::Sell,
        margin,
    );
    let (scale_price, scale_quantity, scaled_margin) = scale_price_quantity_perp_market_dec("9.7", "0.1", "2", &QUOTE_DECIMALS);
    let res = wasm
        .execute(
            &env.contract_address,
            &ExecuteMsg::TestTraderTransientDerivativeOrders {
                market_id: MarketId::new(market_id).unwrap(),
                subaccount_id: subaccount_id.clone(),
                price: scale_price.to_string(),
                quantity: scale_quantity.to_string(),
                margin: scaled_margin.to_string(),
            },
            &[],
            &env.users[0].account,
        )
        .unwrap();

    let transient_query = res
        .events
        .iter()
        .find(|e| e.ty == "wasm-transient_derivative_order")
        .and_then(|event| event.attributes.iter().find(|a| a.key == "query_str"));

    assert!(transient_query.is_some());
    let expected_order_info = "Ok(TraderDerivativeOrdersResponse { orders: Some([TrimmedDerivativeLimitOrder { price: FPDecimal { num: 9700000000000000000000000, sign: 1 }, quantity: FPDecimal { num: 100000000000000000, sign: 1 }, margin: FPDecimal { num: 1940000000000000000000000, sign: 1 }, fillable: FPDecimal { num: 100000000000000000, sign: 1 }, isBuy: true";
    assert!(transient_query.unwrap().value.contains(expected_order_info));
}
