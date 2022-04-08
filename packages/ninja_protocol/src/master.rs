use cosmwasm_std::Uint128;
use injective_cosmwasm::InjectiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub distribution_contract: String, // collected rewards receiver
    pub ninja_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        distribution_contract: Option<String>,
        ninja_token: Option<String>,
    },
    MintToUser {
        pool_subaccount_id: String,
        subaccount_id_sender: String,
        amount: Uint128,
    },
    BurnFromUser {
        pool_subaccount_id: String,
        subaccount_id_sender: String,
        amount: Uint128,
    },
    ExecuteOrders {
        injective_messages: Vec<InjectiveMsg>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub distribution_contract: String, // collected rewards receiver
    pub ninja_token: String,
}
