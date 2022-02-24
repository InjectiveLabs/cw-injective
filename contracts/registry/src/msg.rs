use schemars::JsonSchema;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use crate::state::CONTRACT;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub receiver_address: Addr,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {contract_address : Addr, gas_limit: u64},    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetContracts returns the registered contracts as a json-encoded number
    GetContracts {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractsResponse {
    pub contracts: Vec<CONTRACT>,
}