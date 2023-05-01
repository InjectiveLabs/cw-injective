use cosmwasm_std::{Coin, CustomQuery, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::exchange::{MarketVolume, VolumeByType};
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
    AggregateMarketVolume {
        market_id: MarketId,
    },
    AggregateAccountVolume {
        account: String,
    },
    DenomDecimal {
        denom: String,
    },
    DenomDecimals {
        denoms: Vec<String>,
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
    PythPrice {
        price_id: String,
    },
    TokenFactoryDenomTotalSupply {
        denom: String,
    },
    TokenFactoryDenomCreationFee {},
    // wasmx
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
    pub price_pair_state: Option<PricePairState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PythPriceResponse {
    pub pyth_price_state: Option<PythPriceState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PricePairState {
    #[serde(default)]
    pub pair_price: FPDecimal,
    #[serde(default)]
    pub base_price: FPDecimal,
    #[serde(default)]
    pub quote_price: FPDecimal,
    #[serde(default)]
    pub base_cumulative_price: FPDecimal,
    #[serde(default)]
    pub quote_cumulative_price: FPDecimal,
    #[serde(default)]
    pub base_timestamp: i64,
    #[serde(default)]
    pub quote_timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PriceState {
    #[serde(default)]
    pub price: FPDecimal,
    #[serde(default)]
    pub cumulative_price: FPDecimal,
    #[serde(default)]
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PythPriceState {
    #[serde(default)]
    pub price_id: String,
    #[serde(default)]
    pub ema_price: FPDecimal,
    #[serde(default)]
    pub ema_conf: FPDecimal,
    #[serde(default)]
    pub conf: FPDecimal,
    #[serde(default)]
    pub publish_time: i64,
    pub price_state: PriceState,
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
pub struct MarketMidPriceAndTOBResponse {
    pub mid_price: Option<FPDecimal>,
    pub best_buy_price: Option<FPDecimal>,
    pub best_sell_price: Option<FPDecimal>,
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

/// Response to query for aggregate volume for a given market
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DenomDecimals {
    pub denom: String,
    pub decimals: u64,
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
