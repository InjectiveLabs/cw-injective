use cosmwasm_std::Uint128;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::exchange::{
    derivative::{EffectivePosition, Position, TrimmedDerivativeLimitOrder},
    derivative_market::{FullDerivativeMarket, PerpetualMarketFunding, PerpetualMarketInfo},
    spot::TrimmedSpotLimitOrder,
    spot_market::SpotMarket,
    types::{DenomDecimals, Deposit, MarketVolume, Params, PriceLevel, VolumeByType},
};
use crate::oracle::volatility::{MetadataStatistics, TradeRecord};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExchangeParamsResponse {
    pub params: Option<Params>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SubaccountDepositResponse {
    pub deposits: Deposit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SubaccountEffectivePositionInMarketResponse {
    pub state: Option<EffectivePosition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SubaccountPositionInMarketResponse {
    pub state: Option<Position>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PerpetualMarketInfoResponse {
    pub info: Option<PerpetualMarketInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PerpetualMarketFundingResponse {
    pub state: Option<PerpetualMarketFunding>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TraderDerivativeOrdersResponse {
    pub orders: Option<Vec<TrimmedDerivativeLimitOrder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TraderSpotOrdersResponse {
    pub orders: Option<Vec<TrimmedSpotLimitOrder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeMarketResponse {
    pub market: Option<FullDerivativeMarket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotMarketResponse {
    pub market: Option<SpotMarket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MarketMidPriceAndTOBResponse {
    pub mid_price: Option<FPDecimal>,
    pub best_buy_price: Option<FPDecimal>,
    pub best_sell_price: Option<FPDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MarketVolatilityResponse {
    pub volatility: Option<FPDecimal>,
    pub history_metadata: Option<MetadataStatistics>,
    pub raw_history: Option<Vec<TradeRecord>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct StakedAmountResponse {
    pub staked_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OracleVolatilityResponse {
    pub volatility: Option<FPDecimal>,
    pub history_metadata: Option<MetadataStatistics>,
    pub raw_history: Option<Vec<TradeRecord>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryOrderbookResponse {
    #[serde(default)]
    pub buys_price_level: Vec<PriceLevel>,
    #[serde(default)]
    pub sells_price_level: Vec<PriceLevel>,
}

/// Response to query for aggregate volumes of a given account/subaccount - divided by markets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryAggregateVolumeResponse {
    pub aggregate_volumes: Vec<MarketVolume>,
}

/// Response to query for aggregate volume for a given market
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryAggregateMarketVolumeResponse {
    pub volume: VolumeByType,
}

/// Response to query for decimals for a given denom
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryDenomDecimalResponse {
    pub decimals: u64,
}

/// Response to query for decimals for multiple denoms
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryDenomDecimalsResponse {
    pub denom_decimals: Vec<DenomDecimals>,
}

/// Response to query for fee multiplier for atomic order
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryMarketAtomicExecutionFeeMultiplierResponse {
    pub multiplier: FPDecimal,
}
