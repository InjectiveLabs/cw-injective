use cosmwasm_std::{Addr, from_json};
use injective_test_tube::{Account, Module, Wasm};
use injective_cosmwasm::checked_address_to_subaccount_id;
use crate::msg::{QueryMsg, QueryStargateResponse};
use crate::testing::type_helpers::ParamResponse;
use crate::testing::type_helpers::ExchangeParams;
use crate::utils::{BASE_DECIMALS, BASE_DENOM, ExchangeType, Setup, str_coin};

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
}