use cosmwasm_std::Addr;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::exchange::types::{MarketId, SubaccountId};

use super::types::ShortSubaccountId;

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[repr(u8)]
pub enum OrderSide {
    Unspecified = 0,
    Buy = 1,
    Sell = 2,
}

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
pub struct ShortOrderData {
    pub market_id: MarketId,
    pub subaccount_id: ShortSubaccountId,
    pub order_hash: String,
}

impl From<OrderData> for ShortOrderData {
    fn from(order: OrderData) -> Self {
        ShortOrderData {
            market_id: order.market_id,
            subaccount_id: order.subaccount_id.into(),
            order_hash: order.order_hash,
        }
    }
}

pub fn order_data_to_short(order_data: Vec<OrderData>) -> Vec<ShortOrderData> {
    order_data.into_iter().map(|item| item.into()).collect()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OrderInfo {
    pub subaccount_id: SubaccountId,
    #[serde(default)]
    pub fee_recipient: Option<Addr>,
    pub price: FPDecimal,
    pub quantity: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ShortOrderInfo {
    pub subaccount_id: ShortSubaccountId,
    #[serde(default)]
    pub fee_recipient: Option<Addr>,
    pub price: FPDecimal,
    pub quantity: FPDecimal,
}

impl From<OrderInfo> for ShortOrderInfo {
    fn from(order_info: OrderInfo) -> Self {
        ShortOrderInfo {
            subaccount_id: order_info.subaccount_id.into(),
            fee_recipient: order_info.fee_recipient,
            price: order_info.price,
            quantity: order_info.quantity,
        }
    }
}

pub trait GenericOrder {
    fn get_order_type(&self) -> &OrderType;
    fn get_order_info(&self) -> &OrderInfo;
    fn get_trigger_price(&self) -> Option<FPDecimal>;
    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;
}

pub trait GenericTrimmedOrder {
    fn get_price(&self) -> FPDecimal;
    fn get_fillable_quantity(&self) -> FPDecimal;
    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;
    fn get_order_hash(&self) -> String;
}

#[cfg(test)]
mod tests {
    use crate::exchange::order::OrderType;

    #[test]
    fn order_type_serialization() {
        let types = vec![OrderType::Undefined, OrderType::Buy, OrderType::SellPo, OrderType::SellAtomic];
        assert_eq!(serde_json_wasm::to_string(&types).unwrap(), "[0,1,8,10]");
    }
}
