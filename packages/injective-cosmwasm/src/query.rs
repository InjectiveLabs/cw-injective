use cosmwasm_std::{Coin, CustomQuery, Uint128};
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
    OracleType, Position, SpotMarket,
};
use crate::{MarketId, SubaccountId};

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
        subaccount_id: SubaccountId,
        denom: String,
    },
    SpotMarket {
        market_id: MarketId,
    },
    TraderSpotOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TraderSpotOrdersToCancelUpToAmount {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    },
    TraderDerivativeOrdersToCancelUpToAmount {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    },
    // DerivativeMarket will return the derivative market for a given id
    DerivativeMarket {
        market_id: MarketId,
    },
    SubaccountPositions {
        subaccount_id: SubaccountId,
    },
    SubaccountPositionInMarket {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    SubaccountEffectivePositionInMarket {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TraderDerivativeOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TraderTransientSpotOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TraderTransientDerivativeOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    PerpetualMarketInfo {
        market_id: MarketId,
    },
    PerpetualMarketFunding {
        market_id: MarketId,
    },
    MarketVolatility {
        market_id: MarketId,
        trade_history_options: TradeHistoryOptions,
    },
    SpotMarketMidPriceAndTob {
        market_id: MarketId,
    },
    DerivativeMarketMidPriceAndTob {
        market_id: MarketId,
    },
    OracleVolatility {
        base_info: Option<OracleInfo>,
        quote_info: Option<OracleInfo>,
        oracle_history_options: Option<OracleHistoryOptions>,
    },
    OraclePrice {
        oracle_type: OracleType,
        base: String,
        quote: String,
    },
    TokenFactoryDenomTotalSupply {
        denom: String,
    },
    TokenFactoryDenomCreationFee {},
    // wasxm
    WasmxRegisteredContractInfo {
        contract_address: String,
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
pub struct OraclePriceResponse {
    pub price: FPDecimal,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TokenFactoryCreateDenomFeeResponse {
    pub fee: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RegisteredContract {
    // limit of gas per BB execution
    pub gas_limit: u64,
    // gas price that contract is willing to pay for execution in BeginBlocker
    pub gas_price: u64,
    // is contract currently active
    pub is_executable: bool,
    // code_id that is allowed to be executed (to prevent malicious updates) - if nil/0 any code_id can be executed
    pub code_id: u64,
    // optional - admin addr that is allowed to update contract data
    pub admin_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryContractRegistrationInfoResponse {
    pub contract: Option<RegisteredContract>,
}
