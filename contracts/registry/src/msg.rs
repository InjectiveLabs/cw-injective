use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {
        contract_address: Addr,
        gas_limit: u64,
        gas_price: String,
        is_executable: bool,
    },
    Update {
        contract_address: Addr,
        gas_limit: u64,
        gas_price: String,      
    },    
    Activate {
        contract_address: Addr,        
    },
    DeActivate {
        contract_address: Addr,        
    },
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetContracts returns the registered contracts as a json-encoded number
    GetContract {contract_address: Addr},
    GetContracts {},
    GetActiveContracts {},
}

// Contract struct defines begin blocker contract execution params.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractExecutionParams {
    pub address: Addr,
    pub gas_limit: u64,
    pub gas_price: String,
    pub is_executable: bool,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractsResponse {
    pub contracts: Vec<ContractExecutionParams>,
}


// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractResponse {
    pub contract: ContractExecutionParams,
}
