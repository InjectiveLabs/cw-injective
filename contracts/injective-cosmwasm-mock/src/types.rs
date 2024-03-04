#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    // CosmwasmExt,
)]
// #[proto_message(type_url = "/injective.exchange.v1beta1.OrderInfo")]
pub struct OrderInfo {
    /// bytes32 subaccount ID that created the order
    #[prost(string, tag = "1")]
    #[serde(alias = "subaccountID")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// address fee_recipient address that will receive fees for the order
    #[prost(string, tag = "2")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// price of the order
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag = "4")]
    pub quantity: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    // CosmwasmExt,
)]
// #[proto_message(type_url = "/injective.exchange.v1beta1.SpotOrder")]
pub struct SpotOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "3")]
    // #[serde(
    //     serialize_with = "crate::serde::as_str::serialize",
    //     deserialize_with = "crate::serde::as_str::deserialize"
    // )]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "4")]
    pub trigger_price: ::prost::alloc::string::String,
}

/// MsgCreateSpotMarketOrder defines a SDK message for creating a new spot market
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    // CosmwasmExt,
)]
// #[proto_message(type_url = "/injective.exchange.v1beta1.MsgCreateSpotMarketOrder")]
pub struct MsgCreateSpotMarketOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<SpotOrder>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    // CosmwasmExt,
)]
// #[proto_message(type_url = "/injective.exchange.v1beta1.MsgCreateSpotLimitOrder")]
pub struct MsgCreateSpotLimitOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<SpotOrder>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum OrderType {
    Unspecified = 0,
    Buy = 1,
    Sell = 2,
    StopBuy = 3,
    StopSell = 4,
    TakeBuy = 5,
    TakeSell = 6,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    //CosmwasmExt
)]
//#[proto_message(type_url = "/injective.exchange.v1beta1.MsgCreateDerivativeLimitOrder")]
pub struct MsgCreateDerivativeLimitOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    // CosmwasmExt,
)]
// #[proto_message(type_url = "/injective.exchange.v1beta1.DerivativeOrder")]
pub struct DerivativeOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "3")]
    // #[serde(
    //     serialize_with = "crate::serde::as_str::serialize",
    //     deserialize_with = "crate::serde::as_str::deserialize"
    // )]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag = "4")]
    pub margin: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "5")]
    pub trigger_price: ::prost::alloc::string::String,
}
