use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{msg::OrderInfo, route::InjectiveRoute};
use cosmwasm_std::CustomQuery;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InjectiveQueryWrapper {
    pub route: InjectiveRoute,
    pub query_data: InjectiveQuery,
}

/// InjectiveQuery is an override of QueryRequest::Custom to access Injective-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InjectiveQuery {
    // SubaccountDeposit will return the subaccount deposits for a given subaccount_id and denom
    SubaccountDeposit {
        subaccount_id: String,
        denom: String,
    },
    // DerivativeMarket will return the derivative market for a given id
    DerivativeMarket {
        market_id: String,
    },
    SubaccountPositions {
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
    PerpetualMarketInfo {
        market_id: String,
    },
    PerpetualMarketFunding {
        market_id: String,
    },
    DerivativeMarketVolatility {
        market_id: String,
        from: i64,
        only_trade_history: bool,
    },
    SpotMarketVolatility {
        market_id: String,
        from: i64,
    },
}

impl CustomQuery for InjectiveQueryWrapper {}

/// SubaccountDepositResponse is data format returned from ExchangeQuery::SubaccountDeposit query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubaccountDepositResponse {
    pub deposits: Deposit,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
    #[serde(default)]
    pub isLong: bool,
    pub quantity: FPDecimal,
    pub entry_price: FPDecimal,
    #[serde(default)]
    pub margin: FPDecimal,
    pub cumulative_funding_entry: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EffectivePosition {
    #[serde(default)]
    pub is_long: bool,
    pub quantity: FPDecimal,
    pub entry_price: FPDecimal,
    #[serde(default)]
    pub effective_margin: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: FPDecimal,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl DerivativeLimitOrder {
    pub fn new(
        margin: FPDecimal,
        fillable: FPDecimal,
        order_hash: String,
        trigger_price: Option<FPDecimal>,
        order_type: i32,
        order_info: OrderInfo,
    ) -> DerivativeLimitOrder {
        DerivativeLimitOrder {
            margin,
            fillable,
            order_hash,
            trigger_price,
            order_type,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativePosition {
    pub subaccount_id: String,
    pub market_id: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubaccountEffectivePositionInMarketResponse {
    pub state: Option<EffectivePosition>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrimmedDerivativeLimitOrder {
    pub price: FPDecimal,
    pub quantity: FPDecimal,
    pub margin: FPDecimal,
    pub fillable: FPDecimal,
    pub isBuy: bool,
    pub order_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TraderDerivativeOrdersResponse {
    pub orders: Option<Vec<TrimmedDerivativeLimitOrder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketInfoResponse {
    pub info: Option<PerpetualMarketInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketFundingResponse {
    pub state: Option<PerpetualMarketFunding>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeMarketVolatilityResponse {
    pub volatility: Option<FPDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotMarketVolatilityResponse {
    pub volatility: Option<FPDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeMarketResponse {
    pub market: FullDerivativeMarket,
}

/// Deposit is data format for the subaccount deposit
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    #[serde(default)]
    pub available_balance: FPDecimal,
    #[serde(default)]
    pub total_balance: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketInfo {
    pub market_id: String,
    #[serde(default)]
    pub hourly_funding_rate_cap: FPDecimal,
    #[serde(default)]
    pub hourly_interest_rate: FPDecimal,
    #[serde(default)]
    pub next_funding_timestamp: i64,
    pub funding_interval: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketFunding {
    #[serde(default)]
    pub cumulative_funding: FPDecimal,
    #[serde(default)]
    pub cumulative_price: FPDecimal,
    #[serde(default)]
    pub last_timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketState {
    pub market_info: PerpetualMarketInfo,
    pub funding_info: PerpetualMarketFunding,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullDerivativeMarketPerpetualInfo {
    pub perpetual_info: PerpetualMarketState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullDerivativeMarket {
    pub market: Option<DerivativeMarket>,
    pub info: Option<FullDerivativeMarketPerpetualInfo>,
    pub mark_price: FPDecimal,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeMarket {
    pub ticker: String,
    pub oracle_base: String,
    pub oracle_quote: String,
    #[serde(default)]
    pub oracle_type: i32,
    #[serde(default)]
    pub oracle_scale_factor: u32,
    pub quote_denom: String,
    pub market_id: String,
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
