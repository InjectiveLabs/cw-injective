use crate::{
    msg::{QueryMsg, QueryStargateResponse},
    testing::type_helpers::{AuthParams, ParamResponse},
    utils::{ExchangeType, Setup},
};
use injective_test_tube::{Module, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_exchange_params() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.auth.v1beta1.Query/Params".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: ParamResponse<AuthParams> = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(response.params.max_memo_characters, "256");
}
