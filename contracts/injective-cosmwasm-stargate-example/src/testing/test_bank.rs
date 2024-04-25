use cosmwasm_std::{Addr, from_json, Coin};
use injective_test_tube::{Account, Module, Wasm};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use injective_cosmwasm::{checked_address_to_subaccount_id};

use crate::{
    msg::QueryMsg,
    utils::{
        ExchangeType, Setup,
    },
};
use crate::msg::QueryStargateResponse;
use serde_json::{Value, Map};
use crate::testing::type_helpers::{AuthParams, ParamResponse};
use crate::utils::{BASE_DECIMALS, BASE_DENOM, str_coin};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_exchange_params() {
    let env = Setup::new(ExchangeType::None);

    let wasm = Wasm::new(&env.app);
    let user = &env.users[0];

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(user.account.address()), 1u32);
    // Execute contract

    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.auth.v1beta1.Query/Params".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response =  contract_response.value;
    let response: ParamResponse<AuthParams> = serde_json::from_str(&contract_response).unwrap();
    assert_eq!(response.params.max_memo_characters, "256");
 }






