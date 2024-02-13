use crate::{
    msg::{ExecuteMsg, QueryMsg},
    utils::{human_to_dec, human_to_proto, str_coin, Setup, BASE_DECIMALS, BASE_DENOM, QUOTE_DECIMALS, QUOTE_DENOM},
};

use cosmwasm_std::Coin;
use injective_cosmwasm::{DerivativeMarketResponse, ExchangeParamsResponse, MarketId, SubaccountDepositResponse};
use injective_math::{scale::Scaled, FPDecimal};
use injective_std::types::injective::exchange::v1beta1::{
    Deposit, MsgDeposit, MsgInstantPerpetualMarketLaunch, MsgInstantSpotMarketLaunch, QueryDerivativeMarketsRequest, QuerySpotMarketsRequest,
    QuerySubaccountDepositsRequest,
};
use injective_test_tube::{injective_cosmwasm::SpotMarketResponse, Account, Exchange, Module, RunnerResult, Wasm};

pub fn dec_to_proto(val: FPDecimal) -> String {
    val.scaled(18).to_string()
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_msg_deposit() {
    let env = Setup::new();

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
    let env = Setup::new();
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
    let env = Setup::new();
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
    let env = Setup::new();
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
    let env = Setup::new();
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

    let spot_markets = exchange
        .query_spot_markets(&QuerySpotMarketsRequest {
            status: "Active".to_string(),
            market_ids: vec![],
        })
        .unwrap()
        .markets;

    let market = spot_markets.iter().find(|m| m.ticker == ticker).unwrap();
    let spot_market_id = market.market_id.to_string();

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
    let env = Setup::new();
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);

    let ticker = "INJ/USDT".to_string();
    let initial_margin_ratio = FPDecimal::must_from_str("0.195");
    let maintenance_margin_ratio = FPDecimal::must_from_str("0.05");
    let min_price_tick_size = FPDecimal::must_from_str("1000000000000000000000");
    let min_quantity_tick_size = FPDecimal::must_from_str("1000000000000000");
    let quote_denom = QUOTE_DENOM.to_string();
    let maker_fee_rate = FPDecimal::ZERO;
    let taker_fee_rate = FPDecimal::ZERO;

    println!("{} {}", initial_margin_ratio, dec_to_proto(initial_margin_ratio));
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

    let derivative_markets = exchange
        .query_derivative_markets(&QueryDerivativeMarketsRequest {
            status: "Active".to_string(),
            market_ids: vec![],
            with_mid_price_and_tob: false,
        })
        .unwrap()
        .markets;

    let market = derivative_markets
        .iter()
        .filter(|m| m.market.is_some())
        .find(|m| m.market.as_ref().unwrap().ticker == ticker)
        .unwrap()
        .market
        .as_ref()
        .unwrap();

    let derivative_market_id = market.market_id.to_string();

    let market_id = MarketId::new(derivative_market_id.clone()).unwrap();
    let query_msg = QueryMsg::TestDerivativeMarketQuery { market_id };
    let res: DerivativeMarketResponse = wasm.query(&env.contract_address, &query_msg).unwrap();

    let response_market = res.market.unwrap().market.unwrap();
    println!("{:?}", response_market);
    assert_eq!(response_market.market_id.as_str(), derivative_market_id);
    assert_eq!(response_market.ticker, ticker);
    assert_eq!(response_market.quote_denom, quote_denom);
    assert_eq!(response_market.min_price_tick_size, min_price_tick_size);
    assert_eq!(response_market.min_quantity_tick_size, min_quantity_tick_size);
    assert_eq!(response_market.maker_fee_rate, maker_fee_rate);
    assert_eq!(response_market.taker_fee_rate, taker_fee_rate);
    assert_eq!(response_market.initial_margin_ratio, initial_margin_ratio);
}
