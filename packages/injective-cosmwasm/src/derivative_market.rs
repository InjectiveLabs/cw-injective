use crate::oracle::OracleType;
use crate::MarketId;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PerpetualMarketInfo {
    pub market_id: MarketId,
    #[serde(default)]
    pub hourly_funding_rate_cap: FPDecimal,
    #[serde(default)]
    pub hourly_interest_rate: FPDecimal,
    #[serde(default)]
    pub next_funding_timestamp: i64,
    pub funding_interval: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PerpetualMarketFunding {
    #[serde(default)]
    pub cumulative_funding: FPDecimal,
    #[serde(default)]
    pub cumulative_price: FPDecimal,
    #[serde(default)]
    pub last_timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PerpetualMarketState {
    pub market_info: PerpetualMarketInfo,
    pub funding_info: PerpetualMarketFunding,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct FullDerivativeMarketPerpetualInfo {
    pub perpetual_info: PerpetualMarketState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct FullDerivativeMarket {
    pub market: Option<DerivativeMarket>,
    pub info: Option<FullDerivativeMarketPerpetualInfo>,
    pub mark_price: FPDecimal,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeMarket {
    pub ticker: String,
    pub oracle_base: String,
    pub oracle_quote: String,
    #[serde(default)]
    pub oracle_type: OracleType,
    #[serde(default)]
    pub oracle_scale_factor: u32,
    pub quote_denom: String,
    pub market_id: MarketId,
    pub initial_margin_ratio: FPDecimal,
    pub maintenance_margin_ratio: FPDecimal,
    pub maker_fee_rate: FPDecimal,
    pub taker_fee_rate: FPDecimal,
    #[serde(default)]
    pub isPerpetual: bool,
    #[serde(default)]
    pub status: i32,
    pub min_price_tick_size: FPDecimal,
    pub min_quantity_tick_size: FPDecimal,
}
