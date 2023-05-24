use cosmwasm_std::{Addr, Coin};
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
    Swap {
        target_denom: String,
        min_quantity: FPDecimal,
    },
    SetRoute {
        denom_1: String,
        denom_2: String,
        route: Vec<MarketId>,
    },
    DeleteRoute {
        denom_1: String,
        denom_2: String,
    },
    UpdateConfig {
        admin: Option<Addr>,
        fee_recipient: Option<FeeRecipient>
    },
    WithdrawSupportFunds {
        coins: Vec<Coin>,
        target_address: Addr,
    }
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
