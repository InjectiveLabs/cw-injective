use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::volatility::{MetadataStatistics, TradeRecord};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleInfo {
    pub symbol: String,
    pub oracle_type: i32,
    pub scale_factor: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleHistoryOptions {
    pub max_age: u64,
    pub include_raw_history: bool,
    pub include_metadata: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleVolatilityResponse {
    pub volatility: Option<FPDecimal>,
    pub history_metadata: Option<MetadataStatistics>,
    pub raw_history: Vec<TradeRecord>,
}
