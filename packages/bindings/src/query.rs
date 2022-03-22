use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::InjectiveRoute;
use cosmwasm_std::{CustomQuery, Decimal256 as Decimal};

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
    SubaccountDeposit { subaccount_id: String, denom: String },
    // DerivativeMarket will return the derivative market for a given id
    DerivativeMarket { market_id: String },
    SubaccountPositions { subaccount_id: String },
    SubaccountPositionInMarket { market_id: String, subaccount_id: String },
    TraderDerivativeOrders { market_id: String, subaccount_id: String },
    PerpetualMarketInfo { market_id: String },
    PerpetualMarketFunding { market_id: String },
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
    pub isLong: bool,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub margin: Decimal,
    pub cumulative_funding_entry: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativePosition {
    pub subaccount_id: String,
    pub market_id: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubaccountPositionInMarketResponse {
    pub state: Option<Position>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrimmedDerivativeLimitOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub margin: Decimal,
    pub fillable: Decimal,
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
pub struct DerivativeMarketResponse {
    pub market: FullDerivativeMarket,
}

/// Deposit is data format for the subaccount deposit
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub available_balance: Decimal,
    pub total_balance: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketInfo {
    pub market_id: String,
    pub hourly_funding_rate_cap: Decimal,
    pub hourly_interest_rate: Decimal,
    pub next_funding_timestamp: i64,
    pub funding_interval: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketFunding {
    pub cumulative_funding: Decimal,
    pub cumulative_price: Decimal,
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
    pub market: DerivativeMarket,
    pub info: FullDerivativeMarketPerpetualInfo,
    pub mark_price: Decimal,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeMarket {
    pub ticker: String,
    pub oracle_base: String,
    pub oracle_quote: String,
    pub oracle_type: i32,
    pub oracle_scale_factor: u32,
    pub quote_denom: String,
    pub market_id: String,
    pub initial_margin_ratio: Decimal,
    pub maintenance_margin_ratio: Decimal,
    pub maker_fee_rate: Decimal,
    pub taker_fee_rate: Decimal,
    pub isPerpetual: bool,
    pub status: i32,
    pub min_price_tick_size: Decimal,
    pub min_quantity_tick_size: Decimal,
}
