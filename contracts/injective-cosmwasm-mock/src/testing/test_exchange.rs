use crate::{
    msg::{ExecuteMsg, QueryMsg},
    utils::{
        add_spot_initial_liquidity, add_spot_order_as, add_spot_orders, dec_to_proto, execute_all_authorizations,
        get_initial_liquidity_orders_vector, get_spot_market_id, human_to_dec, human_to_proto, scale_price_quantity_for_spot_market,
        scale_price_quantity_for_spot_market_dec, str_coin, ExchangeType, HumanOrder, Setup, BASE_DECIMALS, BASE_DENOM, QUOTE_DECIMALS, QUOTE_DENOM,
    },
};
use cosmwasm_std::{Addr, Coin};
use injective_cosmwasm::{
    checked_address_to_subaccount_id,
    exchange::{response::QueryOrderbookResponse, types::VolumeByType},
    CancellationStrategy, ExchangeParamsResponse, MarketId, MarketMidPriceAndTOBResponse, MarketVolatilityResponse, OrderSide, PriceLevel,
    QueryAggregateVolumeResponse, QueryMarketAtomicExecutionFeeMultiplierResponse, SpotMarketResponse, SubaccountDepositResponse, SubaccountId,
    TraderSpotOrdersResponse, TrimmedSpotLimitOrder,
};
use injective_math::FPDecimal;
use injective_std::types::injective::exchange::v1beta1::{
    Deposit, MsgDeposit, MsgInstantSpotMarketLaunch, OrderType, QueryAggregateMarketVolumeResponse, QuerySubaccountDepositsRequest,
};
use injective_test_tube::{injective_cosmwasm::get_default_subaccount_id_for_checked_address, Account, Exchange, Module, RunnerResult, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_msg_deposit() {
    let env = Setup::new(ExchangeType::None);

    let wasm = Wasm::new(&env.app);
    let user = &env.users[0];

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(user.account.address()), 1u32);
    // Execute contract
    let res = wasm.execute(
        &env.contract_address,
        &ExecuteMsg::TestDepositMsg {
            subaccount_id: subaccount_id.clone(),
            amount: Coin::new(100, "usdt"),
        },
        &[Coin::new(100, "usdt")],
        &user.account,
    );
    assert!(res.is_ok(), "Execution failed with error: {:?}", res.unwrap_err());
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_exchange_params() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let res: ExchangeParamsResponse = wasm.query(&env.contract_address, &QueryMsg::TestExchangeParamsQuery {}).unwrap();

    assert!(res.params.is_some());
    let params = res.params.unwrap();

    let listing_fee_coin = str_coin("1000", BASE_DENOM, BASE_DECIMALS);
    assert_eq!(params.spot_market_instant_listing_fee, listing_fee_coin);
    assert_eq!(params.derivative_market_instant_listing_fee, listing_fee_coin);
    assert_eq!(params.trading_rewards_vesting_duration, 604800);
    assert_eq!(params.is_instant_derivative_market_launch_enabled, Some(true));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_subaccount_deposit() {
    let env = Setup::new(ExchangeType::None);
    let exchange = Exchange::new(&env.app);
    let wasm = Wasm::new(&env.app);

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(env.users[0].account.address()), 1u32);

    let make_deposit = |amount: &str, denom_key: &str| {
        exchange
            .deposit(
                MsgDeposit {
                    sender: env.users[0].account.address(),
                    subaccount_id: subaccount_id.to_string(),
                    amount: Some(injective_std::types::cosmos::base::v1beta1::Coin {
                        amount: amount.to_string(),
                        denom: env.denoms[denom_key].clone(),
                    }),
                },
                &env.users[0].account,
            )
            .unwrap();
    };

    make_deposit("10000000000000000000", "base");
    make_deposit("100000000", "quote");

    let response = exchange
        .query_subaccount_deposits(&QuerySubaccountDepositsRequest {
            subaccount_id: subaccount_id.to_string(),
            subaccount: None,
        })
        .unwrap();

    assert_eq!(
        response.deposits[&env.denoms["base"].clone()],
        Deposit {
            available_balance: human_to_proto("10.0", BASE_DECIMALS),
            total_balance: human_to_proto("10.0", BASE_DECIMALS),
        }
    );
    assert_eq!(
        response.deposits[&env.denoms["quote"].clone()],
        Deposit {
            available_balance: human_to_proto("100.0", QUOTE_DECIMALS),
            total_balance: human_to_proto("100.0", QUOTE_DECIMALS),
        }
    );

    let query_msg = QueryMsg::TestSubAccountDepositQuery {
        subaccount_id: subaccount_id.clone(),
        denom: BASE_DENOM.to_string(),
    };
    let contract_response: SubaccountDepositResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(contract_response.deposits.total_balance, human_to_dec("10.0", BASE_DECIMALS));

    let query_msg = QueryMsg::TestSubAccountDepositQuery {
        subaccount_id: subaccount_id.clone(),
        denom: QUOTE_DENOM.to_string(),
    };
    let contract_response: SubaccountDepositResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(contract_response.deposits.available_balance, human_to_dec("100.0", QUOTE_DECIMALS));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_spot_market_no_market_on_exchange() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    // Query
    let market_id = MarketId::new("0xd5a22be807011d5e42d5b77da3f417e22676efae494109cd01c242ad46630115").unwrap();
    let query_msg = QueryMsg::TestSpotMarketQuery { market_id };
    let res: RunnerResult<SpotMarketResponse> = wasm.query(&env.contract_address, &query_msg);
    assert_eq!(res, Ok(SpotMarketResponse { market: None }));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_spot_market() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);

    // Instantiate spot market
    let ticker = "INJ/USDT".to_string();
    let min_price_tick_size = FPDecimal::must_from_str("0.000000000000001");
    let min_quantity_tick_size = FPDecimal::must_from_str("1000000000000000");

    exchange
        .instant_spot_market_launch(
            MsgInstantSpotMarketLaunch {
                sender: env.signer.address(),
                ticker: ticker.clone(),
                base_denom: BASE_DENOM.to_string(),
                quote_denom: QUOTE_DENOM.to_string(),
                min_price_tick_size: dec_to_proto(min_price_tick_size),
                min_quantity_tick_size: dec_to_proto(min_quantity_tick_size),
            },
            &env.signer,
        )
        .unwrap();

    let spot_market_id = get_spot_market_id(&exchange, ticker.to_owned());

    // Query
    let market_id = MarketId::new(spot_market_id.clone()).unwrap();
    let query_msg = QueryMsg::TestSpotMarketQuery { market_id };
    let res: SpotMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();

    let response_market = res.market.unwrap();
    assert_eq!(response_market.market_id.as_str(), spot_market_id);
    assert_eq!(response_market.ticker.as_str(), ticker);
    assert_eq!(response_market.base_denom.as_str(), BASE_DENOM);
    assert_eq!(response_market.quote_denom.as_str(), QUOTE_DENOM);
    assert_eq!(response_market.min_price_tick_size.clone().to_string(), min_price_tick_size.to_string());
    assert_eq!(response_market.min_quantity_tick_size.to_string(), min_quantity_tick_size.to_string());
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_spot_orders() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(env.users[0].account.address()))
        .as_str()
        .to_string();

    let query_msg = QueryMsg::TestTraderSpotOrders {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id).unwrap(),
    };

    {
        let (price, quantity) = scale_price_quantity_for_spot_market("10.01", "5.1", &BASE_DECIMALS, &QUOTE_DECIMALS);
        add_spot_order_as(&env.app, market_id.to_owned(), &env.users[0], price, quantity, OrderType::Sell);

        let res: TraderSpotOrdersResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
        let orders = res.orders.clone().unwrap();

        assert_eq!(orders.len(), 1, "Expected exactly one order in the response");
        let expected_orders = TrimmedSpotLimitOrder {
            price: human_to_dec("10.01", QUOTE_DECIMALS - BASE_DECIMALS),
            quantity: human_to_dec("5.1", BASE_DECIMALS),
            fillable: human_to_dec("5.1", BASE_DECIMALS),
            isBuy: false,
            order_hash: "".to_string(),
        };
        assert_eq!(orders[0].price, expected_orders.price);
        assert_eq!(orders[0].quantity, expected_orders.quantity);
        assert_eq!(orders[0].fillable, expected_orders.fillable);
        assert_eq!(orders[0].isBuy, expected_orders.isBuy);
    }

    {
        let (price, quantity) = scale_price_quantity_for_spot_market("9.90", "0.5", &BASE_DECIMALS, &QUOTE_DECIMALS);
        add_spot_order_as(&env.app, market_id.to_owned(), &env.users[0], price, quantity, OrderType::Buy);

        let res: TraderSpotOrdersResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
        let orders = res.orders.clone().unwrap();

        assert_eq!(orders.len(), 2);
        let expected_order = TrimmedSpotLimitOrder {
            price: human_to_dec("9.90", QUOTE_DECIMALS - BASE_DECIMALS),
            quantity: human_to_dec("0.5", BASE_DECIMALS),
            fillable: human_to_dec("0.5", BASE_DECIMALS),
            isBuy: true,
            order_hash: "".to_string(),
        };
        assert_eq!(orders[0].price, expected_order.price);
        assert_eq!(orders[0].quantity, expected_order.quantity);
        assert_eq!(orders[0].fillable, expected_order.fillable);
        assert_eq!(orders[0].isBuy, expected_order.isBuy);
    }
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_spot_market_mid_price_and_tob() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_spot_initial_liquidity(&env.app, market_id.clone());

    let query_msg = QueryMsg::TestSpotMarketMidPriceAndTob {
        market_id: MarketId::new(market_id.clone()).unwrap(),
    };

    let res: MarketMidPriceAndTOBResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.mid_price, Some(human_to_dec("10", QUOTE_DECIMALS - BASE_DECIMALS)));
    assert_eq!(res.best_buy_price, Some(human_to_dec("9.9", QUOTE_DECIMALS - BASE_DECIMALS)));
    assert_eq!(res.best_sell_price, Some(human_to_dec("10.1", QUOTE_DECIMALS - BASE_DECIMALS)));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_spot_market_orderbook() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders = get_initial_liquidity_orders_vector();
    add_spot_orders(&env.app, market_id.clone(), liquidity_orders.clone());

    let query_msg = QueryMsg::TestSpotMarketOrderbook {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        side: OrderSide::Unspecified,
        limit_cumulative_quantity: None,
        limit_cumulative_notional: None,
    };

    let res: QueryOrderbookResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let buys_price_level = res.buys_price_level;
    let sells_price_level = res.sells_price_level;
    assert_eq!(buys_price_level.len(), 4);
    assert_eq!(sells_price_level.len(), 4);
    assert_eq!(
        buys_price_level[0],
        PriceLevel {
            p: human_to_dec(liquidity_orders[sells_price_level.len()].price.as_str(), QUOTE_DECIMALS - BASE_DECIMALS),
            q: human_to_dec(liquidity_orders[sells_price_level.len()].quantity.as_str(), BASE_DECIMALS),
        }
    );
    assert_eq!(
        buys_price_level[1],
        PriceLevel {
            p: human_to_dec(
                liquidity_orders[sells_price_level.len() + 1].price.as_str(),
                QUOTE_DECIMALS - BASE_DECIMALS
            ),
            q: human_to_dec(liquidity_orders[sells_price_level.len() + 1].quantity.as_str(), BASE_DECIMALS),
        }
    );

    assert_eq!(
        sells_price_level[0],
        PriceLevel {
            p: human_to_dec(
                liquidity_orders[sells_price_level.len() - 1].price.as_str(),
                QUOTE_DECIMALS - BASE_DECIMALS
            ),
            q: human_to_dec(liquidity_orders[sells_price_level.len() - 1].quantity.as_str(), BASE_DECIMALS),
        }
    );
    assert_eq!(
        sells_price_level[1],
        PriceLevel {
            p: human_to_dec(
                liquidity_orders[sells_price_level.len() - 2].price.as_str(),
                QUOTE_DECIMALS - BASE_DECIMALS
            ),
            q: human_to_dec(liquidity_orders[sells_price_level.len() - 2].quantity.as_str(), BASE_DECIMALS),
        }
    );
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_market_volatility() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_spot_initial_liquidity(&env.app, market_id.clone());

    let query_msg = QueryMsg::TestMarketVolatility {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        trade_grouping_sec: 0,
        max_age: 0,
        include_raw_history: false,
        include_metadata: true,
    };

    let res: MarketVolatilityResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.volatility, None);

    let new_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.4".to_string(),
            quantity: "15".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.0".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
    ];

    add_spot_orders(&env.app, market_id.clone(), new_orders.clone());
    let res: MarketVolatilityResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.volatility, Some(FPDecimal::ZERO));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_aggregate_market_volume() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_spot_initial_liquidity(&env.app, market_id.clone());

    let query_msg = QueryMsg::TestAggregateMarketVolume {
        market_id: MarketId::new(market_id.clone()).unwrap(),
    };

    let res: QueryAggregateMarketVolumeResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.volume.clone().unwrap().maker_volume, "0");
    assert_eq!(res.volume.clone().unwrap().taker_volume, "0");

    // consume liquidity
    let new_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
    ];

    add_spot_orders(&env.app, market_id.clone(), new_orders);
    let res: QueryAggregateMarketVolumeResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.volume.clone().unwrap().maker_volume, "150500000");
    assert_eq!(res.volume.clone().unwrap().taker_volume, "150500000");
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_aggregate_account_volume() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    add_spot_initial_liquidity(&env.app, market_id.clone());

    let query_msg = QueryMsg::TestAggregateAccountVolume {
        account_id: env.users[1].subaccount_id.to_string(),
    };

    let res: QueryAggregateVolumeResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(res.aggregate_volumes.is_none());

    let (price, quantity) = scale_price_quantity_for_spot_market("9.9", "1", &BASE_DECIMALS, &QUOTE_DECIMALS);
    add_spot_order_as(&env.app, market_id.clone(), &env.users[1], price, quantity, OrderType::Sell);

    let res: QueryAggregateVolumeResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let volume: &VolumeByType = &res.aggregate_volumes.unwrap()[0].volume;
    assert_eq!(volume.maker_volume, FPDecimal::ZERO);
    assert_eq!(volume.taker_volume, human_to_dec("9.9", QUOTE_DECIMALS));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_market_atomic_execution_fee_multiplier() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();
    let query_msg = QueryMsg::TestMarketAtomicExecutionFeeMultiplier {
        market_id: MarketId::new(market_id.clone()).unwrap(),
    };
    let res: QueryMarketAtomicExecutionFeeMultiplierResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.multiplier, human_to_dec("0.0000025", QUOTE_DECIMALS));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_spot_orders_to_cancel_up_to_amount() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(env.users[0].account.address()))
        .as_str()
        .to_string();

    let query_spot_msg = QueryMsg::TestTraderSpotOrders {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        subaccount_id: SubaccountId::new(subaccount_id.to_owned()).unwrap(),
    };

    {
        let (price, quantity) = scale_price_quantity_for_spot_market("9.90", "1", &BASE_DECIMALS, &QUOTE_DECIMALS);
        add_spot_order_as(&env.app, market_id.to_owned(), &env.users[0], price, quantity, OrderType::Buy);

        let res: TraderSpotOrdersResponse = wasm.query(&env.contract_address, &query_spot_msg).unwrap();
        let orders = res.orders.clone().unwrap();

        let expected_order = TrimmedSpotLimitOrder {
            price: human_to_dec("9.90", QUOTE_DECIMALS - BASE_DECIMALS),
            quantity: human_to_dec("1", BASE_DECIMALS),
            fillable: human_to_dec("1", BASE_DECIMALS),
            isBuy: true,
            order_hash: "".to_string(),
        };
        assert_eq!(orders[0].price, expected_order.price);
        assert_eq!(orders[0].quantity, expected_order.quantity);
        assert_eq!(orders[0].fillable, expected_order.fillable);
        assert_eq!(orders[0].isBuy, expected_order.isBuy);
    }

    {
        let query_spot_cancel_msg = QueryMsg::TestSpotOrdersToCancelUpToAmount {
            market_id: MarketId::new(market_id.clone()).unwrap(),
            subaccount_id: SubaccountId::new(subaccount_id).unwrap(),
            base_amount: human_to_dec("0", BASE_DECIMALS),
            quote_amount: human_to_dec("0.2", QUOTE_DECIMALS),
            strategy: CancellationStrategy::UnspecifiedOrder,
            reference_price: None,
        };
        let res: TraderSpotOrdersResponse = wasm.query(&env.contract_address, &query_spot_cancel_msg).unwrap();
        let orders = res.orders.clone().unwrap();
        assert_eq!(orders.len(), 1);
    }
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_transient_spot_orders() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(env.users[0].account.address()), 0u32);

    execute_all_authorizations(&env.app, &env.users[0].account, env.contract_address.clone());
    add_spot_initial_liquidity(&env.app, market_id.clone());

    let (scale_price, scale_quantity) = scale_price_quantity_for_spot_market_dec("9.8", "1", &BASE_DECIMALS, &QUOTE_DECIMALS);

    let res = wasm
        .execute(
            &env.contract_address,
            &ExecuteMsg::TestTraderTransientSpotOrders {
                market_id: MarketId::new(market_id).unwrap(),
                subaccount_id: subaccount_id.clone(),
                price: scale_price.to_string(),
                quantity: scale_quantity.to_string(),
            },
            &[],
            &env.users[0].account,
        )
        .unwrap();

    let transient_query = res
        .events
        .iter()
        .find(|e| e.ty == "wasm-transient_order")
        .and_then(|event| event.attributes.iter().find(|a| a.key == "query_str"));

    assert!(transient_query.is_some());
    let expected_order_info = "TraderSpotOrdersResponse { orders: Some([TrimmedSpotLimitOrder { price: FPDecimal { num: 9800000, sign: 1 }, quantity: FPDecimal { num: 1000000000000000000000000000000000000, sign: 1 }";
    assert!(transient_query.unwrap().value.contains(expected_order_info));
}
