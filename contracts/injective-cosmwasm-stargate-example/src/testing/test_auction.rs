use cosmwasm_std::{from_json, Coin, Int64, Uint128, Uint64};
use injective_math::FPDecimal;
use injective_test_tube::{Module, Wasm};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::QueryStargateResponse;
use crate::{
    msg::QueryMsg,
    utils::{ExchangeType, Setup},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryCurrentAuctionBasketResponse {
    pub amount: Vec<Coin>,
    pub auction_round: Uint64,
    pub auction_closing_time: Int64,
    pub highest_bidder: String,
    pub highest_bid_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryAuctionParamsResponse {
    pub params: AuctionParams,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AuctionParams {
    pub auction_period: Int64,
    pub min_next_bid_increment_rate: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryLastAuctionResultResponse {
    pub last_auction_result: LastAuctionResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LastAuctionResult {
    pub winner: String,
    pub amount: Coin,
    pub round: Uint64,
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_current_auction_basket() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/injective.auction.v1beta1.Query/CurrentAuctionBasket".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: QueryCurrentAuctionBasketResponse = from_json(contract_response).unwrap();

    assert_eq!(response.amount, vec![]);
    assert_eq!(response.auction_closing_time, Int64::from(-62121081600i64));
    assert_eq!(response.highest_bid_amount, Uint128::zero());
    assert_eq!(response.auction_round, Uint64::from(23u64));
    assert_eq!(response.highest_bidder, "".to_string());
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_auction_params() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/injective.auction.v1beta1.Query/AuctionParams".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: QueryAuctionParamsResponse = from_json(contract_response).unwrap();

    assert_eq!(response.params.auction_period, Int64::from(604800i64));
    assert_eq!(response.params.min_next_bid_increment_rate, FPDecimal::must_from_str("0.0025"));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_last_auction_result() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/injective.auction.v1beta1.Query/LastAuctionResult".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let response: QueryLastAuctionResultResponse = from_json(contract_response).unwrap();

    assert_eq!(response.last_auction_result.winner, "".to_string());
    assert_eq!(response.last_auction_result.round, Uint64::zero());
    assert_eq!(response.last_auction_result.amount, Coin::new(0u128, "inj"));
}
