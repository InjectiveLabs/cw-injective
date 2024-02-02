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
    TestSpotMarketQuery { market_id: MarketId },
    TestSubAccountDepositQuery { subaccount_id: SubaccountId, denom: String },
}
