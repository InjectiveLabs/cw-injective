use cosmwasm_std::{Addr, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::exchange::{
    order::OrderSide,
    types::{MarketId, SubaccountId},
};
use crate::oracle::{
    types::{OracleHistoryOptions, OracleInfo, OracleType},
    volatility::TradeHistoryOptions,
};
use crate::route::InjectiveRoute;

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
    // Authz
    Grants {
        granter: String,
        grantee: String,
        msg_type_url: String,
        pagination: Option<u32>,
    },
    GranteeGrants {
        grantee: String,
        pagination: Option<u32>,
    },
    GranterGrants {
        granter: String,
        pagination: Option<u32>,
    },
    // Exchange
    ExchangeParams {},
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
    SpotOrderbook {
        market_id: MarketId,
        limit: u64,
        order_side: OrderSide,
        limit_cumulative_quantity: Option<FPDecimal>,
        limit_cumulative_notional: Option<FPDecimal>,
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
    MarketAtomicExecutionFeeMultiplier {
        market_id: MarketId,
    },
    // Staking
    StakedAmount {
        delegator_address: Addr,
        max_delegations: u16,
    },
    // Oracle
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
    // Wasmx
    WasmxRegisteredContractInfo {
        contract_address: String,
    },
}

impl CustomQuery for InjectiveQueryWrapper {}
