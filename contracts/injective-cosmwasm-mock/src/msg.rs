use cosmwasm_std::Coin;
use injective_cosmwasm::{MarketId, SubaccountId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TestDepositMsg { subaccount_id: SubaccountId, amount: Coin },
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
    TestTraderTransientSpotOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    TestTraderTransientDerivativeOrders {
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
}
