use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OrderData {
    pub market_id: String,
    pub subaccount_id: String,
    pub order_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OrderInfo {
    pub subaccount_id: String,
    pub fee_recipient: String,
    pub price: FPDecimal,
    pub quantity: FPDecimal,
}
