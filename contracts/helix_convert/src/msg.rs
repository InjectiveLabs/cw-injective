use injective_cosmwasm::MarketId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SwapSpot {
        quantity: FPDecimal,
        price: FPDecimal,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    WorstAndAverageMarketPrice{
        market_id: MarketId,

    }
}
