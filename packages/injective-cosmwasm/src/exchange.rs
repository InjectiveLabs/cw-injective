use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Deposit is data format for the subaccount deposit
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    #[serde(default)]
    pub available_balance: FPDecimal,
    #[serde(default)]
    pub total_balance: FPDecimal,
}
