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
use crate::utils::{BASE_DECIMALS, BASE_DENOM, str_coin};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_msg_deposit() {
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
    // let response: Value = serde_json::from_str(&contract_response).unwrap();
    // let response = response.as_object().unwrap();
    // println!("{:?}", response);
    // assert_eq!(response.get("params").unwrap().as_object().unwrap().get("max_memo_characters").unwrap().as_str().unwrap(), "256");
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ParamResponse<T> {
    pub params: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AuthParams {
    pub max_memo_characters: String,
    pub sig_verify_cost_ed25519: String,
}


#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_exchange_param() {
    let env = Setup::new(ExchangeType::None);

    let wasm = Wasm::new(&env.app);
    let user = &env.users[0];

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(user.account.address()), 1u32);
    // Execute contract

    let query_msg = QueryMsg::QueryStargate {
        path: "/injective.exchange.v1beta1.Query/QueryExchangeParams".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response =  contract_response.value;
    println!("{:?}", contract_response);
    let response: ParamResponse<ExchangeParams> = from_json(&contract_response).unwrap();
    println!("{:?}", response);
    let listing_fee_coin = str_coin("1000", BASE_DENOM, BASE_DECIMALS);
    assert_eq!(response.params.spot_market_instant_listing_fee, listing_fee_coin);
    assert_eq!(1,2)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExchangeParams {
    pub spot_market_instant_listing_fee: Coin,
}
