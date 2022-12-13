use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {
        contract_address: Addr,
        gas_limit: u64,
        gas_price: u64,
        is_executable: bool,
    },
    Deregister {
        contract_address: Addr,
    },
    Update {
        contract_address: Addr,
        gas_limit: u64,
        gas_price: u64,
    },
    Activate {
        contract_address: Addr,
    },
    Deactivate {
        contract_address: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetContracts returns the registered contracts as a json-encoded number
    GetContract {
        contract_address: Addr,
    },
    GetContracts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    GetActiveContracts {
        min_gas_price: Option<u64>,
        start_after: Option<(u64, String)>,
        limit: Option<u32>,
    },
}

// Contract struct defines begin blocker contract execution params.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractExecutionParams {
    pub address: Addr,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub is_executable: bool,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractsResponse {
    pub contracts: Vec<ContractExecutionParams>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractResponse {
    pub contract: ContractExecutionParams,
}
