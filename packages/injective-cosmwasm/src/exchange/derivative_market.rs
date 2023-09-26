use crate::exchange::types::MarketId;
use crate::oracle::types::OracleType;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::market::{GenericMarket, MarketStatus};

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
    pub status: MarketStatus,
    pub min_price_tick_size: FPDecimal,
    pub min_quantity_tick_size: FPDecimal,
}

impl GenericMarket for DerivativeMarket {
    fn get_ticker(&self) -> &str {
        &self.ticker
    }

    fn get_quote_denom(&self) -> &str {
        &self.quote_denom
    }

    fn get_maker_fee_rate(&self) -> FPDecimal {
        self.maker_fee_rate
    }

    fn get_taker_fee_rate(&self) -> FPDecimal {
        self.taker_fee_rate
    }

    fn get_market_id(&self) -> &MarketId {
        &self.market_id
    }

    fn get_status(&self) -> MarketStatus {
        self.status
    }

    fn get_min_price_tick_size(&self) -> FPDecimal {
        self.min_price_tick_size
    }

    fn min_quantity_tick_size(&self) -> FPDecimal {
        self.min_quantity_tick_size
    }
}
