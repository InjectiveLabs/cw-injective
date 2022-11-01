use cosmwasm_std::Addr;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{MarketId, SubaccountId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OrderData {
    pub market_id: MarketId,
    pub subaccount_id: SubaccountId,
    pub order_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OrderInfo {
    pub subaccount_id: SubaccountId,
    #[serde(default)]
    pub fee_recipient: Option<Addr>,
    pub price: FPDecimal,
    pub quantity: FPDecimal,
}
