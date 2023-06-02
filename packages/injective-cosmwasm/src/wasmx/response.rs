use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::wasmx::types::RegisteredContract;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryContractRegistrationInfoResponse {
    pub contract: Option<RegisteredContract>,
}
