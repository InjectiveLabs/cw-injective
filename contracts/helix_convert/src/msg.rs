use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_cosmwasm::MarketId;
use injective_math::FPDecimal;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum FeeRecipient {
    Address(Addr),
    SwapContract,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub fee_recipient: FeeRecipient,
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetRoute {
        denom_1: String,
        denom_2: String,
        route: Vec<MarketId>,
    },
    Swap {
        target_denom: String,
        min_quantity: FPDecimal,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRoute {
        denom_1: String,
        denom_2: String,
    },
    GetExecutionQuantity {
        from_quantity: FPDecimal,
        from_denom: String,
        to_denom: String,
    },
}
