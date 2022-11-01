use crate::MarketId;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotMarket {
    pub ticker: String,
    pub base_denom: String,
    pub quote_denom: String,
    pub maker_fee_rate: FPDecimal,
    pub taker_fee_rate: FPDecimal,
    pub relayer_fee_share_rate: FPDecimal,
    pub market_id: MarketId,
    #[serde(default)]
    pub status: i32,
    pub min_price_tick_size: FPDecimal,
    pub min_quantity_tick_size: FPDecimal,
}
