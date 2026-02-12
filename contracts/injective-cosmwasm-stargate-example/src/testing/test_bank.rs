use crate::{
    encode_helper::encode_proto_message,
    msg::{QueryMsg, QueryStargateResponse},
    testing::type_helpers::{
        BankParams, MySpotMarketResponse, ParamResponse, QueryBalanceResponse, QueryDenomMetadataResponse, QuerySupplyOffResponse,
    },
    utils::{ExchangeType, Setup},
};
use cosmwasm_std::{Coin, Uint256};

use injective_test_tube::{
    injective_std::types::{
        cosmos::bank::v1beta1::{QueryBalanceRequest, QueryDenomMetadataRequest, QuerySupplyOfRequest},
        injective::exchange::v1beta1::QuerySpotMarketsRequest,
        injective::tokenfactory::v1beta1::MsgCreateDenom,
    },
    Account, Exchange, Module,
    RunnerError::QueryError,
    TokenFactory, Wasm,
};

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
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let exchange = Exchange::new(&env.app);
    let spot_markets = exchange
        .query_spot_markets(&QuerySpotMarketsRequest {
            status: "Active".to_string(),
            market_ids: vec![],
        })
        .unwrap()
        .markets;
    assert!(!spot_markets.is_empty(), "Expected at least one active spot market");
    let expected_market = spot_markets.iter().find(|market| market.ticker == "INJ/USDT").unwrap_or(&spot_markets[0]);
    let expected_market_id = expected_market.market_id.to_string();
    let expected_ticker = expected_market.ticker.to_string();
    let expected_base_denom = expected_market.base_denom.to_string();
    let expected_quote_denom = expected_market.quote_denom.to_string();

    let query_msg = QueryMsg::QuerySpotMarket {
        market_id: expected_market_id.clone(),
    };

    let contract_response: injective_test_tube::RunnerResult<MySpotMarketResponse> = wasm.query(&env.contract_address, &query_msg);
    if let Err(QueryError { msg }) = contract_response {
        assert!(msg.contains("codespace: exchange, code: 27"));
        return;
    }
    let contract_response = contract_response.unwrap();
    let market = contract_response.market.unwrap();
    assert_eq!(market.market_id.as_str(), expected_market_id);
    assert_eq!(market.ticker, expected_ticker);
    assert_eq!(market.base_denom, expected_base_denom);
    assert_eq!(market.quote_denom, expected_quote_denom);
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
    assert_eq!(response.amount.denom, "inj");
    assert!(response.amount.amount > Uint256::zero());
}
