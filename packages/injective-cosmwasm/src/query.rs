use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    derivative::EffectivePosition, derivative::TrimmedDerivativeLimitOrder, derivative_market::FullDerivativeMarket,
    derivative_market::PerpetualMarketFunding, derivative_market::PerpetualMarketInfo, exchange::Deposit, route::InjectiveRoute,
};
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
    SpotMarket {
        market_id: String,
    },
    TraderSpotOrders {
        market_id: String,
        subaccount_id: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubaccountEffectivePositionInMarketResponse {
    pub state: Option<EffectivePosition>,
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
