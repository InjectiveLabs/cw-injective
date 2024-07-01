use crate::{
    encode_helper::encode_proto_message,
    msg::{QueryMsg, QueryStargateResponse},
    testing::type_helpers::{BankParams, ParamResponse, QueryBalanceResponse, QueryDenomMetadataResponse, QuerySupplyOffResponse},
    utils::{ExchangeType, Setup},
};
use cosmos_sdk_proto::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryDenomMetadataRequest, QuerySupplyOfRequest};
use cosmwasm_std::{Coin, Uint128};
use injective_std::types::injective::tokenfactory::v1beta1::MsgCreateDenom;
use injective_test_tube::{Account, Module, TokenFactory, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_bank_params() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.bank.v1beta1.Query/Params".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: ParamResponse<BankParams> = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(response.params.default_send_enabled, true);
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
        name: "TEST_DENOM".to_string(),
        symbol: "TDM".to_string(),
    };

    let denom = token_factory.create_denom(create_denom_msg, &env.users[0].account).unwrap();
    let denom_name = denom.data.new_token_denom;
    let query_msg = QueryMsg::QueryStargate {
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
    let query_msg = QueryMsg::QueryStargate {
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
            amount: Uint128::new(1_000_000_000_000_000_000_000_000),
        }
    );
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_supply_of() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargate {
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
            amount: Uint128::new(12000003336863671397639633),
        }
    );
}
