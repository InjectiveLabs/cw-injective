use osmosis_std_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.Params")]
pub struct Params {
    /// auction_period_duration defines the auction period duration
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_period: i64,
    /// min_next_bid_increment_rate defines the minimum increment rate for new bids
    #[prost(string, tag = "2")]
    pub min_next_bid_increment_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.Bid")]
pub struct Bid {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.LastAuctionResult")]
pub struct LastAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag = "1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventBid")]
pub struct EventBid {
    /// bidder describes the address of bidder
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    /// amount describes the amount the bidder put on the auction
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventAuctionResult")]
pub struct EventAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag = "1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventAuctionStart")]
pub struct EventAuctionStart {
    /// round defines the round number of auction
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
    /// ending_timestamp describes auction end time
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ending_timestamp: i64,
    /// new_basket describes auction module balance at the time of new auction
    /// start
    #[prost(message, repeated, tag = "3")]
    pub new_basket: ::prost::alloc::vec::Vec<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// GenesisState defines the auction module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to auction.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// current auction round
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_round: u64,
    /// current highest bid
    #[prost(message, optional, tag = "3")]
    pub highest_bid: ::core::option::Option<Bid>,
    /// auction ending timestamp
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_ending_timestamp: i64,
    /// last auction result
    #[prost(message, optional, tag = "5")]
    pub last_auction_result: ::core::option::Option<LastAuctionResult>,
}
/// QueryAuctionParamsRequest is the request type for the Query/AuctionParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryAuctionParamsRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/AuctionParams",
    response_type = QueryAuctionParamsResponse
)]
pub struct QueryAuctionParamsRequest {}
/// QueryAuctionParamsRequest is the response type for the Query/AuctionParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryAuctionParamsResponse")]
pub struct QueryAuctionParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryCurrentAuctionBasketRequest is the request type for the
/// Query/CurrentAuctionBasket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.auction.v1beta1.QueryCurrentAuctionBasketRequest"
)]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/CurrentAuctionBasket",
    response_type = QueryCurrentAuctionBasketResponse
)]
pub struct QueryCurrentAuctionBasketRequest {}
/// QueryCurrentAuctionBasketResponse is the response type for the
/// Query/CurrentAuctionBasket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.auction.v1beta1.QueryCurrentAuctionBasketResponse"
)]
pub struct QueryCurrentAuctionBasketResponse {
    /// amount describes the amount put on auction
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// auctionRound describes current auction round
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_round: u64,
    /// auctionClosingTime describes auction close time for the round
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_closing_time: i64,
    /// highestBidder describes highest bidder on current round
    #[prost(string, tag = "4")]
    pub highest_bidder: ::prost::alloc::string::String,
    /// highestBidAmount describes highest bid amount on current round
    #[prost(string, tag = "5")]
    pub highest_bid_amount: ::prost::alloc::string::String,
}
/// QueryModuleStateRequest is the request type for the Query/AuctionModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryModuleStateRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/AuctionModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateResponse is the response type for the
/// Query/AuctionModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryLastAuctionResultRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/LastAuctionResult",
    response_type = QueryLastAuctionResultResponse
)]
pub struct QueryLastAuctionResultRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryLastAuctionResultResponse")]
pub struct QueryLastAuctionResultResponse {
    #[prost(message, optional, tag = "1")]
    pub last_auction_result: ::core::option::Option<LastAuctionResult>,
}
/// Bid defines a SDK message for placing a bid for an auction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgBid")]
pub struct MsgBid {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// amount of the bid in INJ tokens
    #[prost(message, optional, tag = "2")]
    pub bid_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// the current auction round being bid on
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgBidResponse")]
pub struct MsgBidResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the ocr parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct AuctionQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AuctionQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn auction_params(
        &self,
    ) -> Result<QueryAuctionParamsResponse, cosmwasm_std::StdError> {
        QueryAuctionParamsRequest {}.query(self.querier)
    }
    pub fn current_auction_basket(
        &self,
    ) -> Result<QueryCurrentAuctionBasketResponse, cosmwasm_std::StdError> {
        QueryCurrentAuctionBasketRequest {
        }
            .query(self.querier)
    }
    pub fn auction_module_state(
        &self,
    ) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        QueryModuleStateRequest {}.query(self.querier)
    }
    pub fn last_auction_result(
        &self,
    ) -> Result<QueryLastAuctionResultResponse, cosmwasm_std::StdError> {
        QueryLastAuctionResultRequest {}.query(self.querier)
    }
}
