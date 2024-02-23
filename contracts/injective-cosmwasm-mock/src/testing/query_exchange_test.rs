use crate::{
    msg::{ExecuteMsg, QueryMsg},
    utils::{
        dec_to_proto, get_spot_market_id, human_to_dec, human_to_proto, str_coin, Setup, BASE_DECIMALS, BASE_DENOM, QUOTE_DECIMALS, QUOTE_DENOM,
    },
};

use crate::utils::{
    add_derivative_orders, add_spot_orders, get_perpetual_market_id, scale_price_quantity_for_spot_market, scale_price_quantity_perp_market,
    ExchangeType, HumanOrder,
};
use cosmwasm_std::{Addr, Coin};
use injective_cosmwasm::exchange::response::{QueryAggregateVolumeResponse, QueryOrderbookResponse};
use injective_cosmwasm::{
    DerivativeMarketResponse, ExchangeParamsResponse, MarketId, MarketMidPriceAndTOBResponse, MarketVolatilityResponse, OrderSide,
    PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, PriceLevel, QueryMarketAtomicExecutionFeeMultiplierResponse, SpotMarketResponse,
    SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse, SubaccountId, SubaccountPositionInMarketResponse,
    TraderDerivativeOrdersResponse, TraderSpotOrdersResponse, TrimmedDerivativeLimitOrder, TrimmedSpotLimitOrder,
};
use injective_math::FPDecimal;
use injective_std::types::injective::exchange::v1beta1::{
    Deposit, DerivativeOrder, MsgCreateDerivativeLimitOrder, MsgCreateSpotLimitOrder, MsgDeposit, MsgInstantPerpetualMarketLaunch,
    MsgInstantSpotMarketLaunch, OrderInfo, OrderType, QueryAggregateMarketVolumeResponse, QuerySubaccountDepositsRequest, SpotOrder,
};
use injective_test_tube::injective_cosmwasm::get_default_subaccount_id_for_checked_address;
use injective_test_tube::{Account, Exchange, Module, RunnerResult, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_msg_deposit() {
    let env = Setup::new(ExchangeType::None);

    let wasm = Wasm::new(&env.app);
    let user = &env.users[0];

    // Execute contract
    let res = wasm.execute(
        &env.contract_address,
        &ExecuteMsg::TestDepositMsg {
            subaccount_id: user.subaccount_id.clone(),
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

    {
        exchange
            .deposit(
                MsgDeposit {
                    sender: env.users[0].account.address(),
                    subaccount_id: env.users[0].subaccount_id.to_string(),
                    amount: Some(injective_std::types::cosmos::base::v1beta1::Coin {
                        amount: "10000000000000000000".to_string(),
                        denom: env.denoms["base"].clone(),
                    }),
                },
                &env.users[0].account,
            )
            .unwrap();
    }

    {
        exchange
            .deposit(
                MsgDeposit {
                    sender: env.users[0].account.address(),
                    subaccount_id: env.users[0].subaccount_id.to_string(),
                    amount: Some(injective_std::types::cosmos::base::v1beta1::Coin {
                        amount: "100000000".to_string(),
                        denom: env.denoms["quote"].clone(),
                    }),
                },
                &env.users[0].account,
            )
            .unwrap();
    }

    let response = exchange
        .query_subaccount_deposits(&QuerySubaccountDepositsRequest {
            subaccount_id: env.users[0].subaccount_id.clone().to_string(),
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
        subaccount_id: env.users[0].subaccount_id.clone(),
        denom: BASE_DENOM.to_string(),
    };
    let contract_response: SubaccountDepositResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(contract_response.deposits.total_balance, human_to_dec("10.0", BASE_DECIMALS));

    let query_msg = QueryMsg::TestSubAccountDepositQuery {
        subaccount_id: env.users[0].subaccount_id.clone(),
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
    let exchange = Exchange::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Buy,
        },
    ];
    add_derivative_orders(&env.app, market_id.clone(), liquidity_orders.to_owned(), None);

    let (price, quantity, margin) = scale_price_quantity_perp_market("9.7", "1", "2", &QUOTE_DECIMALS);

    let trader = &env.users[1];
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.account.address()))
        .as_str()
        .to_string();

    exchange
        .create_derivative_limit_order(
            MsgCreateDerivativeLimitOrder {
                sender: trader.account.address(),
                order: Some(DerivativeOrder {
                    market_id: market_id.to_owned(),
                    order_info: Some(OrderInfo {
                        subaccount_id: subaccount_id.to_owned(),
                        fee_recipient: trader.account.address(),
                        price,
                        quantity,
                    }),
                    margin,
                    order_type: OrderType::Sell.into(),
                    trigger_price: "".to_string(),
                }),
            },
            &trader.account,
        )
        .unwrap();

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
    let exchange = Exchange::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Buy,
        },
    ];
    add_derivative_orders(&env.app, market_id.clone(), liquidity_orders.to_owned(), None);

    let (price, quantity, margin) = scale_price_quantity_perp_market("9.7", "1", "2", &QUOTE_DECIMALS);

    let trader = &env.users[1];
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.account.address()))
        .as_str()
        .to_string();

    exchange
        .create_derivative_limit_order(
            MsgCreateDerivativeLimitOrder {
                sender: trader.account.address(),
                order: Some(DerivativeOrder {
                    market_id: market_id.to_owned(),
                    order_info: Some(OrderInfo {
                        subaccount_id: subaccount_id.to_owned(),
                        fee_recipient: trader.account.address(),
                        price,
                        quantity,
                    }),
                    margin,
                    order_type: OrderType::Sell.into(),
                    trigger_price: "".to_string(),
                }),
            },
            &trader.account,
        )
        .unwrap();

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

    let res: SubaccountPositionInMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_spot_orders() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let market_id = env.market_id.unwrap();

    {
        let (price, quantity) = scale_price_quantity_for_spot_market("10.01", "5.1", &BASE_DECIMALS, &QUOTE_DECIMALS);
        let trader = &env.users[0];

        let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.account.address()))
            .as_str()
            .to_string();

        exchange
            .create_spot_limit_order(
                MsgCreateSpotLimitOrder {
                    sender: trader.account.address(),
                    order: Some(SpotOrder {
                        market_id: market_id.clone(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.to_owned(),
                            fee_recipient: trader.account.address(),
                            price,
                            quantity,
                        }),
                        order_type: OrderType::Sell.into(),
                        trigger_price: "".to_string(),
                    }),
                },
                &trader.account,
            )
            .unwrap();

        let query_msg = QueryMsg::TestTraderSpotOrders {
            market_id: MarketId::new(market_id.clone()).unwrap(),
            subaccount_id: SubaccountId::new(subaccount_id).unwrap(),
        };
        let res: TraderSpotOrdersResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
        let orders = res.orders.clone().unwrap();

        assert_eq!(orders.len(), 1);
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
        let trader = &env.users[0];

        let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.account.address()))
            .as_str()
            .to_string();

        exchange
            .create_spot_limit_order(
                MsgCreateSpotLimitOrder {
                    sender: trader.account.address(),
                    order: Some(SpotOrder {
                        market_id: market_id.clone(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.to_owned(),
                            fee_recipient: trader.account.address(),
                            price,
                            quantity,
                        }),
                        order_type: OrderType::Buy.into(),
                        trigger_price: "".to_string(),
                    }),
                },
                &trader.account,
            )
            .unwrap();

        let query_msg = QueryMsg::TestTraderSpotOrders {
            market_id: MarketId::new(market_id.clone()).unwrap(),
            subaccount_id: SubaccountId::new(subaccount_id).unwrap(),
        };
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
fn test_query_trader_derivative_orders() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let market_id = env.market_id.unwrap();

    let (price, quantity, margin) = scale_price_quantity_perp_market("10.1", "1", "2", &QUOTE_DECIMALS);

    let trader = &env.users[0];
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.account.address()))
        .as_str()
        .to_string();

    exchange
        .create_derivative_limit_order(
            MsgCreateDerivativeLimitOrder {
                sender: trader.account.address(),
                order: Some(DerivativeOrder {
                    market_id: market_id.to_owned(),
                    order_info: Some(OrderInfo {
                        subaccount_id: subaccount_id.to_owned(),
                        fee_recipient: trader.account.address(),
                        price,
                        quantity,
                    }),
                    margin,
                    order_type: OrderType::Sell.into(),
                    trigger_price: "".to_string(),
                }),
            },
            &trader.account,
        )
        .unwrap();

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
fn test_query_spot_market_mid_price_and_tob() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
    ];

    add_spot_orders(&env.app, market_id.clone(), liquidity_orders);

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

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
    ];

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
    assert_eq!(buys_price_level.len(), 2);
    assert_eq!(sells_price_level.len(), 2);
    assert_eq!(
        buys_price_level[0],
        PriceLevel {
            p: human_to_dec(liquidity_orders[2].price.as_str(), QUOTE_DECIMALS - BASE_DECIMALS),
            q: human_to_dec(liquidity_orders[2].quantity.as_str(), BASE_DECIMALS),
        }
    );
    assert_eq!(
        buys_price_level[1],
        PriceLevel {
            p: human_to_dec(liquidity_orders[3].price.as_str(), QUOTE_DECIMALS - BASE_DECIMALS),
            q: human_to_dec(liquidity_orders[3].quantity.as_str(), BASE_DECIMALS),
        }
    );

    assert_eq!(
        sells_price_level[0],
        PriceLevel {
            p: human_to_dec(liquidity_orders[1].price.as_str(), QUOTE_DECIMALS - BASE_DECIMALS),
            q: human_to_dec(liquidity_orders[1].quantity.as_str(), BASE_DECIMALS),
        }
    );
    assert_eq!(
        sells_price_level[1],
        PriceLevel {
            p: human_to_dec(liquidity_orders[0].price.as_str(), QUOTE_DECIMALS - BASE_DECIMALS),
            q: human_to_dec(liquidity_orders[0].quantity.as_str(), BASE_DECIMALS),
        }
    );
}

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
fn test_query_market_volatility() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
    ];

    add_spot_orders(&env.app, market_id.clone(), liquidity_orders.clone());

    let query_msg = QueryMsg::TestMarketVolatility {
        market_id: MarketId::new(market_id.clone()).unwrap(),
        trade_grouping_sec: 0,
        max_age: 0,
        include_raw_history: false,
        include_metadata: true,
    };

    let res: MarketVolatilityResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(res.volatility, None);

    // consume liquidity
    let new_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
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
fn test_query_derivative_market_mid_price_and_tob() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Buy,
        },
    ];
    add_derivative_orders(&env.app, market_id.clone(), liquidity_orders, None);

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
fn test_query_aggregate_market_volume() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
    ];

    add_spot_orders(&env.app, market_id.clone(), liquidity_orders);

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

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
    ];

    add_spot_orders(&env.app, market_id.clone(), liquidity_orders);

    let query_msg = QueryMsg::TestAggregateAccountVolume {
        account_id: env.users[0].subaccount_id.to_string(),
    };

    let res: RunnerResult<QueryAggregateVolumeResponse> = wasm.query(&env.contract_address, &query_msg);
    println!("{:?}", res);
    assert_eq!(1, 2);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_derivative_market_orderbook() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let market_id = env.market_id.unwrap();

    let liquidity_orders: Vec<HumanOrder> = vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Buy,
        },
    ];
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
fn test_query_spot_orders_to_cancel_up_to_amount() {}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_derivative_orders_to_cancel_up_to_amount() {}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_transient_spot_orders() {}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_transient_derivative_orders() {}
