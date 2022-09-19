use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MetadataStatistics {
    pub group_count: u32,
    pub records_sample_size: u32,
    pub mean: FPDecimal,
    pub twap: FPDecimal,
    pub first_timestamp: i64,
    pub last_timestamp: i64,
    pub min_price: FPDecimal,
    pub max_price: FPDecimal,
    pub median_price: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TradeHistoryOptions {
    pub trade_grouping_sec: u64,
    pub max_age: u64,
    pub include_raw_history: bool,
    pub include_metadata: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PriceRecord {
    timestamp: i64,
    price: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TradeRecord {
    timestamp: i64,
    price: FPDecimal,
    quantity: FPDecimal,
}
