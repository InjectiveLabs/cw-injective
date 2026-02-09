use crate::{
    encode_helper::encode_proto_message,
    msg::{QueryMsg, QueryStargateResponse},
    testing::type_helpers::{BankParams, ParamResponse, QueryBalanceResponse, QueryDenomMetadataResponse, QuerySupplyOffResponse},
    utils::{get_perpetual_market_id, ExchangeType, Setup, BASE_DENOM, QUOTE_DENOM},
};
use cosmwasm_std::{Coin, Uint256};
use injective_math::FPDecimal;
use injective_std::types::injective::exchange::v2::{self, open_notional_cap::Cap, OpenNotionalCap, OpenNotionalCapUncapped};

use injective_test_tube::{
    injective_std::types::{
        cosmos::bank::v1beta1::{QueryBalanceRequest, QueryDenomMetadataRequest, QuerySupplyOfRequest},
        injective::tokenfactory::v1beta1::MsgCreateDenom,
    },
    Account, Exchange, Module, TokenFactory, Wasm,
};
use injective_testing::utils::dec_to_proto;

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_bank_params() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryBankParams {};

    let contract_response: ParamResponse<BankParams> = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(contract_response.params.default_send_enabled);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_spot_market() {
    let env = Setup::new(ExchangeType::Derivative);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let ticker = "INJ/USDT".to_string();
    let initial_margin_ratio = FPDecimal::must_from_str("0.195");
    let maintenance_margin_ratio = FPDecimal::must_from_str("0.05");
    let min_price_tick_size = FPDecimal::must_from_str("0.1");
    let min_quantity_tick_size = FPDecimal::must_from_str("1000000000000000");
    let min_notional = FPDecimal::must_from_str("0.001");
    let quote_denom = QUOTE_DENOM.to_string();
    let maker_fee_rate = FPDecimal::must_from_str("-0.0001");
    let taker_fee_rate = FPDecimal::must_from_str("0.001");

    // add_exchange_admin(&env.app, &env.validator, env.owner.address());

    exchange
        .instant_perpetual_market_launch_v2(
            v2::MsgInstantPerpetualMarketLaunch {
                sender: env.owner.address(),
                ticker: ticker.to_owned(),
                quote_denom: quote_denom.to_owned(),
                oracle_base: BASE_DENOM.to_string(),
                oracle_quote: quote_denom.to_owned(),
                oracle_scale_factor: 6u32,
                oracle_type: 2i32,
                maker_fee_rate: dec_to_proto(maker_fee_rate),
                taker_fee_rate: dec_to_proto(taker_fee_rate),
                initial_margin_ratio: dec_to_proto(initial_margin_ratio),
                maintenance_margin_ratio: dec_to_proto(maintenance_margin_ratio),
                min_price_tick_size: dec_to_proto(min_price_tick_size),
                min_quantity_tick_size: dec_to_proto(min_quantity_tick_size),
                min_notional: dec_to_proto(min_notional),
                reduce_margin_ratio: dec_to_proto(initial_margin_ratio),
                open_notional_cap: Some(OpenNotionalCap {
                    cap: Some(Cap::Uncapped(OpenNotionalCapUncapped {})),
                }),
            },
            &env.owner,
        )
        .unwrap();

    let derivative_market_id = get_perpetual_market_id(&exchange, ticker.to_owned());

    let query_msg = QueryMsg::QuerySpotMarket {
        market_id: derivative_market_id.to_string(),
    };

    let contract_response: ParamResponse<BankParams> = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(contract_response.params.default_send_enabled);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_bank_params_raw() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/cosmos.bank.v1beta1.Query/Params".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: ParamResponse<BankParams> = serde_json::from_str(&contract_response).unwrap();
    assert!(response.params.default_send_enabled);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_denom_metadata() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let token_factory = TokenFactory::new(&env.app);

    let create_denom_msg = MsgCreateDenom {
        sender: env.users[0].account.address().to_string(),
        subdenom: "cw".to_string(),
        name: "CosmWasm".to_string(),
        symbol: "CW".to_string(),
        decimals: 6u32,
        allow_admin_burn: true,
    };

    let denom = token_factory.create_denom(create_denom_msg, &env.users[0].account).unwrap();
    let denom_name = denom.data.new_token_denom;
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/cosmos.bank.v1beta1.Query/DenomMetadata".to_string(),
        query_request: encode_proto_message(QueryDenomMetadataRequest {
            denom: denom_name.to_owned(),
        }),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: QueryDenomMetadataResponse = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(response.metadatas[0].denom_units[0].denom, denom_name);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_bank_balance() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let user_address = env.users[0].account.address().to_string();
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        query_request: encode_proto_message(QueryBalanceRequest {
            address: user_address.to_owned(),
            denom: "inj".to_string(),
        }),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: QueryBalanceResponse = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(
        response.balance,
        Coin {
            denom: "inj".to_string(),
            amount: Uint256::new(1_000_000_000_000_000_000_000_000),
        }
    );
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_supply_of() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/cosmos.bank.v1beta1.Query/SupplyOf".to_string(),
        query_request: encode_proto_message(QuerySupplyOfRequest { denom: "inj".to_string() }),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: QuerySupplyOffResponse = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(
        response.amount,
        Coin {
            denom: "inj".to_string(),
            amount: Uint256::new(12000004078367203674350010),
        }
    );
}
