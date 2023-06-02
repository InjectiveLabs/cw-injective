use crate::oracle::volatility::{MetadataStatistics, TradeRecord};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OracleInfo {
    pub symbol: String,
    pub oracle_type: OracleType,
    pub scale_factor: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OracleHistoryOptions {
    pub max_age: u64,
    pub include_raw_history: bool,
    pub include_metadata: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OracleVolatilityResponse {
    pub volatility: Option<FPDecimal>,
    pub history_metadata: Option<MetadataStatistics>,
    pub raw_history: Vec<TradeRecord>,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema, Copy)]
#[repr(i32)]
#[derive(Default)]
pub enum OracleType {
    #[default]
    Unspecified = 0,
    Band = 1,
    PriceFeed = 2,
    Coinbase = 3,
    Chainlink = 4,
    Razor = 5,
    Dia = 6,
    API3 = 7,
    Uma = 8,
    Pyth = 9,
    BandIBC = 10,
    Provider = 11,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PriceAttestation {
    pub product_id: String,
    // pub price_id: Hash,
    pub price_id: String,
    pub price: i64,
    pub conf: u64,
    pub expo: i32,
    pub ema_price: i64,
    pub ema_conf: u64,
    pub status: PythStatus,
    pub num_publishers: u32,
    pub max_num_publishers: u32,
    pub attestation_time: i64,
    pub publish_time: i64,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema, Copy)]
#[repr(i32)]
pub enum PythStatus {
    Unknown = 0,
    Trading = 1,
    Halted = 2,
    Auction = 3,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PricePairState {
    #[serde(default)]
    pub pair_price: FPDecimal,
    #[serde(default)]
    pub base_price: FPDecimal,
    #[serde(default)]
    pub quote_price: FPDecimal,
    #[serde(default)]
    pub base_cumulative_price: FPDecimal,
    #[serde(default)]
    pub quote_cumulative_price: FPDecimal,
    #[serde(default)]
    pub base_timestamp: i64,
    #[serde(default)]
    pub quote_timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PriceState {
    #[serde(default)]
    pub price: FPDecimal,
    #[serde(default)]
    pub cumulative_price: FPDecimal,
    #[serde(default)]
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PythPriceState {
    #[serde(default)]
    pub price_id: String,
    #[serde(default)]
    pub ema_price: FPDecimal,
    #[serde(default)]
    pub ema_conf: FPDecimal,
    #[serde(default)]
    pub conf: FPDecimal,
    #[serde(default)]
    pub publish_time: i64,
    pub price_state: PriceState,
}
