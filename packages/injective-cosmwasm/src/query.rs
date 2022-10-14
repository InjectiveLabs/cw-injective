use cosmwasm_std::{CustomQuery, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::{
    derivative::EffectivePosition,
    derivative::TrimmedDerivativeLimitOrder,
    derivative_market::{FullDerivativeMarket, PerpetualMarketFunding, PerpetualMarketInfo},
    exchange::Deposit,
    oracle::{OracleHistoryOptions, OracleInfo},
    route::InjectiveRoute,
    spot::TrimmedSpotLimitOrder,
    volatility::{MetadataStatistics, TradeHistoryOptions, TradeRecord},
    Position, SpotMarket,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InjectiveQueryWrapper {
    pub route: InjectiveRoute,
    pub query_data: InjectiveQuery,
}

/// InjectiveQuery is an override of QueryRequest::Custom to access Injective-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InjectiveQuery {
    // SubaccountDeposit will return the subaccount deposits for a given subaccount_id and denom
    SubaccountDeposit {
        subaccount_id: String,
        denom: String,
    },
    SpotMarket {
        market_id: String,
    },
    TraderSpotOrders {
        market_id: String,
        subaccount_id: String,
    },
    TraderSpotOrdersToCancelUpToAmount {
        market_id: String,
        subaccount_id: String,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    },
    TraderDerivativeOrdersToCancelUpToAmount {
        market_id: String,
        subaccount_id: String,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    },
    // DerivativeMarket will return the derivative market for a given id
    DerivativeMarket {
        market_id: String,
    },
    SubaccountPositions {
        subaccount_id: String,
    },
    SubaccountPositionInMarket {
        market_id: String,
        subaccount_id: String,
    },
    SubaccountEffectivePositionInMarket {
        market_id: String,
        subaccount_id: String,
    },
    TraderDerivativeOrders {
        market_id: String,
        subaccount_id: String,
    },
    TraderTransientSpotOrders {
        market_id: String,
        subaccount_id: String,
    },
    TraderTransientDerivativeOrders {
        market_id: String,
        subaccount_id: String,
    },
    PerpetualMarketInfo {
        market_id: String,
    },
    PerpetualMarketFunding {
        market_id: String,
    },
    MarketVolatility {
        market_id: String,
        trade_history_options: TradeHistoryOptions,
    },
    SpotMarketMidPriceAndTob {
        market_id: String,
    },
    DerivativeMarketMidPriceAndTob {
        market_id: String,
    },
    OracleVolatility {
        base_info: Option<OracleInfo>,
        quote_info: Option<OracleInfo>,
        oracle_history_options: Option<OracleHistoryOptions>,
    },
    TokenFactoryDenomTotalSupply {
        denom: String,
    },
}

impl CustomQuery for InjectiveQueryWrapper {}

pub const UNSORTED_CANCELLATION_STRATEGY: i32 = 0;

pub const FROM_WORST_TO_BEST_CANCELLATION_STRATEGY: i32 = 1;

/// SubaccountDepositResponse is data format returned from ExchangeQuery::SubaccountDeposit query
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
pub struct MarketVolatilityResponse {
    pub volatility: Option<FPDecimal>,
    pub history_metadata: Option<MetadataStatistics>,
    pub raw_history: Option<Vec<TradeRecord>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OracleVolatilityResponse {
    pub volatility: Option<FPDecimal>,
    pub history_metadata: Option<MetadataStatistics>,
    pub raw_history: Option<Vec<TradeRecord>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeMarketResponse {
    pub market: FullDerivativeMarket,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotMarketResponse {
    pub market: Option<SpotMarket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotMarketMidPriceAndTOBResponse {
    pub mid_price: Option<FPDecimal>,
    pub best_bid: Option<FPDecimal>,
    pub best_ask: Option<FPDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeMarketMidPriceAndTOBResponse {
    pub mid_price: Option<FPDecimal>,
    pub best_bid: Option<FPDecimal>,
    pub best_ask: Option<FPDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TokenFactoryDenomSupplyResponse {
    pub total_supply: Uint128,
}
