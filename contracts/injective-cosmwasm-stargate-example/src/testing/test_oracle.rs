use crate::{
    encode_helper::encode_proto_message,
    msg::QueryMsg,
    testing::type_helpers::{MyOraclePriceResponse, MyPythPriceResponse, MyVolatilityResponse},
    utils::{
        create_some_inj_price_attestation, get_stargate_query_result, relay_pyth_price, set_address_of_pyth_contract, str_coin, ExchangeType, Setup,
        BASE_DECIMALS, BASE_DENOM, INJ_PYTH_PRICE_ID,
    },
};
use injective_cosmwasm::OracleType;
use injective_math::{scale::Scaled, FPDecimal};
use injective_std::types::injective::oracle::v1beta1::{
    OracleHistoryOptions, OracleInfo, QueryOraclePriceRequest, QueryOracleVolatilityRequest, QueryPythPriceRequest,
};
use injective_test_tube::{Module, Oracle, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_oracle_price() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let oracle = Oracle::new(&env.app);

    let query_msg = QueryMsg::QueryStargate {
        path: "/injective.oracle.v1beta1.Query/OraclePrice".to_string(),
        query_request: encode_proto_message(QueryOraclePriceRequest {
            oracle_type: OracleType::PriceFeed as i32,
            base: env.denoms["base"].to_owned(),
            quote: env.denoms["quote"].to_owned(),
        }),
    };

    let query_oracle_price_request = QueryOraclePriceRequest {
        oracle_type: 2i32,
        base: env.denoms["base"].to_owned(),
        quote: env.denoms["quote"].to_owned(),
    };

    let oracle_response = oracle.query_oracle_price(&query_oracle_price_request);
    let contract_response = get_stargate_query_result::<MyOraclePriceResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();

    let oracle_response_pair_state = oracle_response.unwrap().price_pair_state;
    let contract_response_pair_state = contract_response.price_pair_state;

    assert!(contract_response_pair_state.is_some());
    assert!(oracle_response_pair_state.is_some());
    let oracle_response_pair_state = oracle_response_pair_state.unwrap();
    let contract_response_pair_state = contract_response_pair_state.unwrap();
    let oracle_response_pair_price = FPDecimal::must_from_str(oracle_response_pair_state.pair_price.as_str());
    assert_eq!(contract_response_pair_state.pair_price.scaled(18), oracle_response_pair_price);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_oracle_volatility() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    let base_info = Some(OracleInfo {
        symbol: env.denoms["base"].to_owned(),
        oracle_type: OracleType::PriceFeed as i32,
    });

    let quote_info = Some(OracleInfo {
        symbol: env.denoms["quote"].to_owned(),
        oracle_type: OracleType::PriceFeed as i32,
    });

    let oracle_history_options = Some(OracleHistoryOptions {
        max_age: 60u64,
        include_raw_history: true,
        include_metadata: true,
    });

    let query_msg = QueryMsg::QueryStargate {
        path: "/injective.oracle.v1beta1.Query/OracleVolatility".to_string(),
        query_request: encode_proto_message(QueryOracleVolatilityRequest {
            base_info,
            quote_info,
            oracle_history_options,
        }),
    };

    let res = get_stargate_query_result::<MyVolatilityResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();
    assert!(res.volatility.is_none());
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_pyth_oracle_price() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let oracle = Oracle::new(&env.app);

    let validator = env.app.get_first_validator_signing_account(BASE_DENOM.to_string(), 1.2f64).unwrap();
    let pyth_contract = env.app.init_account(&[str_coin("1000000", BASE_DENOM, BASE_DECIMALS)]).unwrap();

    set_address_of_pyth_contract(&env.app, &validator, &pyth_contract);
    let price_attestations = vec![create_some_inj_price_attestation("7", 5, env.app.get_block_time_seconds())];
    relay_pyth_price(&oracle, price_attestations, &pyth_contract);

    let price_pyth_oracle_response = oracle
        .query_pyth_price(&QueryPythPriceRequest {
            price_id: INJ_PYTH_PRICE_ID.to_string(),
        })
        .unwrap();
    let price_pyth_oracle_response = FPDecimal::must_from_str(price_pyth_oracle_response.price_state.unwrap().ema_price.as_str());

    let query_msg = QueryMsg::QueryStargate {
        path: "/injective.oracle.v1beta1.Query/PythPrice".to_string(),
        query_request: encode_proto_message(QueryPythPriceRequest {
            price_id: INJ_PYTH_PRICE_ID.to_string(),
        }),
    };

    let contract_response = get_stargate_query_result::<MyPythPriceResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();
    assert_eq!(
        contract_response.price_state.unwrap().ema_price.scaled(BASE_DECIMALS),
        price_pyth_oracle_response
    );
}
