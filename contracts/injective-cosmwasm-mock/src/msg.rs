use cosmwasm_std::Coin;
use injective_cosmwasm::{CancellationStrategy, MarketId, OracleInfo, OracleType, OrderSide, SubaccountId};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub const MSG_CREATE_SPOT_LIMIT_ORDER_ENDPOINT: &str = "/injective.exchange.v1beta1.MsgCreateSpotLimitOrder";
pub const MSG_CREATE_DERIVATIVE_LIMIT_ORDER_ENDPOINT: &str = "/injective.exchange.v1beta1.MsgCreateDerivativeLimitOrder";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TestDepositMsg {
        subaccount_id: SubaccountId,
        amount: Coin,
    },
    TestTraderTransientSpotOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        price: String,
        quantity: String,
    },
    TestTraderTransientDerivativeOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        price: String,
        quantity: String,
        margin: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TestExchangeParamsQuery {},
    TestSpotMarketQuery {
        market_id: MarketId,
    },
    TestDerivativeMarketQuery {
        market_id: MarketId,
    },
    TestSubAccountDepositQuery {
        subaccount_id: SubaccountId,
        denom: String,
    },
    TestEffectiveSubaccountPosition {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TestVanillaSubaccountPosition {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TestTraderDerivativeOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TestTraderSpotOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TestSpotOrdersToCancelUpToAmount {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: CancellationStrategy,
        reference_price: Option<FPDecimal>,
    },
    TestDerivativeOrdersToCancelUpToAmount {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        quote_amount: FPDecimal,
        strategy: CancellationStrategy,
        reference_price: Option<FPDecimal>,
    },
    TestPerpetualMarketInfo {
        market_id: MarketId,
    },
    TestPerpetualMarketFunding {
        market_id: MarketId,
    },
    TestMarketVolatility {
        market_id: MarketId,
        trade_grouping_sec: u64,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    },
    TestDerivativeMarketMidPriceAndTob {
        market_id: MarketId,
    },
    TestAggregateMarketVolume {
        market_id: MarketId,
    },
    TestAggregateAccountVolume {
        account_id: String,
    },
    TestSpotMarketMidPriceAndTob {
        market_id: MarketId,
    },
    TestSpotMarketOrderbook {
        market_id: MarketId,
        side: OrderSide,
        limit_cumulative_quantity: Option<FPDecimal>,
        limit_cumulative_notional: Option<FPDecimal>,
    },
    TestDerivativeMarketOrderbook {
        market_id: MarketId,
        limit_cumulative_notional: FPDecimal,
    },
    TestMarketAtomicExecutionFeeMultiplier {
        market_id: MarketId,
    },
    TestQueryOracleVolatility {
        base_info: Option<OracleInfo>,
        quote_info: Option<OracleInfo>,
        max_age: u64,
        include_raw_history: bool,
        include_metadata: bool,
    },
    TestQueryOraclePrice {
        oracle_type: OracleType,
        base: String,
        quote: String,
    },
    TestQueryPythPrice {
        price_id: String,
    },
    TestQueryStakedAmount {
        delegator_address: String,
        max_delegations: u16,
    },
    TestQueryTokenFactoryDenomTotalSupply {
        denom: String,
    },
    TestQueryTokenFactoryCreationFee {},
    TestQueryContractRegistrationInfo {
        contract_address: String,
    },
}
