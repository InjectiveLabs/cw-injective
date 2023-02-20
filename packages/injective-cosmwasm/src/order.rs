use cosmwasm_std::Addr;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{MarketId, SubaccountId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[repr(u8)]
pub enum OrderType {
    Undefined = 0,
    Buy = 1,
    Sell = 2,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}

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

pub trait GenericOrder {
    fn get_order_type(&self) -> &OrderType;
    fn get_order_info(&self) -> &OrderInfo;
    fn get_trigger_price(&self) -> Option<FPDecimal>;
}
