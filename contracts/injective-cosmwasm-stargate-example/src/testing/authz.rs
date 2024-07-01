use cosmos_sdk_proto::cosmos::authz::v1beta1::{QueryGranteeGrantsRequest, QueryGranterGrantsRequest, QueryGrantsRequest };
use crate::{
    encode_helper::encode_proto_message,
    msg::{QueryMsg, QueryStargateResponse},
    utils::{
        execute_all_authorizations,
        ExchangeType, Setup,
    },
};
use injective_test_tube::{Account, Module, RunnerResult, Wasm};
use injective_test_tube::RunnerError::QueryError;

use crate::testing::type_helpers::{Authorization, Grants, StargateQueryGranteeGrantsResponse, StargateQueryGranterGrantsResponse};
use crate::utils::get_stargate_query_result;


#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_grantee_grants() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    execute_all_authorizations(&env.app, &env.users[0].account, env.users[1].account.address().to_string());
    execute_all_authorizations(&env.app, &env.users[2].account, env.users[1].account.address().to_string());

    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.authz.v1beta1.Query/GranteeGrants".to_string(),
        query_request: encode_proto_message(QueryGranteeGrantsRequest {
            grantee: env.users[1].account.address().to_string(),
            pagination: None,
        }),
    };

    let messages = vec![
        "/injective.exchange.v1beta1.MsgBatchUpdateOrders",
        "/injective.exchange.v1beta1.MsgCreateDerivativeLimitOrder",
        "/injective.exchange.v1beta1.MsgCreateDerivativeMarketOrder",
        "/injective.exchange.v1beta1.MsgCreateSpotLimitOrder",
        "/injective.exchange.v1beta1.MsgWithdraw",
    ];

    let response_user0 = create_stargate_response(messages.clone(), env.users[0].account.address().to_string(), env.users[1].account.address().to_string());
    let response_user2 = create_stargate_response(messages, env.users[2].account.address().to_string(), env.users[1].account.address().to_string());

    let combined_grants = response_user0.grants.into_iter().chain(response_user2.grants.into_iter()).collect::<Vec<_>>();
    let query_result = get_stargate_query_result::<StargateQueryGranteeGrantsResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();

    let all_grants_present = combined_grants.iter().all(|grant| query_result.grants.contains(grant));
    let no_extra_grants = combined_grants.len() == query_result.grants.len();

    assert!(all_grants_present);
    assert!(no_extra_grants);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_granter_grants() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    execute_all_authorizations(&env.app, &env.users[0].account, env.users[1].account.address().to_string());

    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.authz.v1beta1.Query/GranterGrants".to_string(),
        query_request: encode_proto_message(QueryGranterGrantsRequest {
            granter: env.users[0].account.address().to_string(),
            pagination: None,
        }),
    };

    let query_result = get_stargate_query_result::<StargateQueryGranterGrantsResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();
    assert_eq!(query_result.grants.len(), 5);

    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.authz.v1beta1.Query/GranterGrants".to_string(),
        query_request: encode_proto_message(QueryGranterGrantsRequest {
            granter: env.users[2].account.address().to_string(),
            pagination: None,
        }),
    };

    let query_result = get_stargate_query_result::<StargateQueryGranterGrantsResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();
    assert_eq!(query_result.grants.len(), 0);

}


#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_grants() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    execute_all_authorizations(&env.app, &env.users[0].account, env.users[1].account.address().to_string());

    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.authz.v1beta1.Query/Grants".to_string(),
        query_request: encode_proto_message(QueryGrantsRequest {
            granter: env.users[0].account.address().to_string(),
            grantee: env.users[1].account.address().to_string(),
            msg_type_url: "/injective.exchange.v1beta1.MsgCreateDerivativeMarketOrder".to_string(),
            pagination: None,
        }),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    println!("{:?}", contract_response);
    // let query_result = get_stargate_query_result::<GranteeGrantsResponse>(wasm.query(&env.contract_address, &query_msg)).unwrap();
    // println!("{:?}", query_result);

    let query_msg = QueryMsg::QueryStargate {
        path: "/cosmos.authz.v1beta1.Query/Grants".to_string(),
        query_request: encode_proto_message(QueryGrantsRequest {
            granter: env.users[2].account.address().to_string(),
            grantee: env.users[1].account.address().to_string(),
            msg_type_url: "/injective.exchange.v1beta1.MsgCreateDerivativeMarketOrder".to_string(),
            pagination: None,
        }),
    };

    let contract_response: RunnerResult<QueryStargateResponse> = wasm.query(&env.contract_address, &query_msg);
    println!("{:?}", contract_response);

    if let Err(QueryError { msg }) = contract_response {
        assert_eq!(
            msg,
            "Generic error: Querier contract error: codespace: authz, code: 2: query wasm contract failed",
            "The error message does not match the expected value"
        );
    } else {
        assert!(false, "Expected an error, but got a success: {:?}", contract_response);
    }


}

fn create_stargate_response(messages: Vec<&str>, granter: String, grantee: String) -> StargateQueryGranteeGrantsResponse {
    let grants = messages.into_iter().map(|msg| {
        Grants {
            granter: granter.clone(),
            grantee: grantee.clone(),
            authorization: Authorization {
                type_str: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                msg: msg.to_string(),
            },
        }
    }).collect();

    StargateQueryGranteeGrantsResponse { grants }
}