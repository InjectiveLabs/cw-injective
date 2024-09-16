use crate::{
    encode_helper::encode_proto_message,
    msg::{QueryMsg, QueryStargateResponse},
    testing::type_helpers::{AuthParams, CosmosAuthQueryAccountsResponse, ParamResponse},
    utils::{ExchangeType, Setup},
};
use cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest;
use injective_test_tube::{Account, Module, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_auth_params() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/cosmos.auth.v1beta1.Query/Params".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: ParamResponse<AuthParams> = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(response.params.max_memo_characters, "256");
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_auth_account() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    let user_address = env.users[0].account.address().to_string();
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/cosmos.auth.v1beta1.Query/Account".to_string(),
        query_request: encode_proto_message(QueryAccountRequest {
            address: user_address.to_owned(),
        }),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: CosmosAuthQueryAccountsResponse = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(response.account.base_account.address, user_address);
}
