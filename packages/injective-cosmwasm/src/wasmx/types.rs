use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[repr(i32)]
pub enum FundingMode {
    Unspecified = 0,
    SelfFunded = 1,
    GrantOnly = 2,
    Dual = 3,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RegisteredContract {
    // limit of gas per BB execution
    pub gas_limit: u64,
    // gas price that contract is willing to pay for execution in BeginBlocker
    pub gas_price: u64,
    // is contract currently active
    pub is_executable: bool,
    // code_id that is allowed to be executed (to prevent malicious updates) - if nil/0 any code_id can be executed
    pub code_id: Option<u64>,
    // optional - admin addr that is allowed to update contract data
    pub admin_address: Option<String>,
    // optional -  address of the contract granting fee
    // must be set if fund_mode is GrantOnly
    pub granter_address: Option<String>,
    /// funding mode
    pub fund_mode: FundingMode,
}
