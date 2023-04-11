use cosmwasm_std::Addr;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{MarketId, SubaccountId};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema)]
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
    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::OrderType;

    #[test]
    fn order_type_serialization() {
        let types = vec![OrderType::Undefined, OrderType::Buy, OrderType::SellPo, OrderType::SellAtomic];
        assert_eq!(serde_json_wasm::to_string(&types).unwrap(), "[0,1,8,10]");
    }
}
