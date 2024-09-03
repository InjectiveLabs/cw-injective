use crate::{
    traits::general::{MarketId, SubaccountId},
    types::injective::exchange::v1beta1::{DerivativeLimitOrder, DerivativeOrder, OrderData, OrderInfo, SpotLimitOrder, SpotOrder},
};

use cosmwasm_std::Addr;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
    StopBuy = 3,
    StopSell = 4,
    TakeBuy = 5,
    TakeSell = 6,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}

impl OrderType {
    pub fn from_i32(value: i32) -> OrderType {
        match value {
            0 => OrderType::Undefined,
            1 => OrderType::Buy,
            2 => OrderType::Sell,
            3 => OrderType::StopBuy,
            4 => OrderType::StopSell,
            5 => OrderType::TakeBuy,
            6 => OrderType::TakeSell,
            7 => OrderType::BuyPo,
            8 => OrderType::SellPo,
            9 => OrderType::BuyAtomic,
            10 => OrderType::SellAtomic,
            _ => unimplemented!("Order type not supported!"),
        }
    }
}

pub trait GenericOrder {
    fn get_order_type(&self) -> OrderType;
    fn get_order_info(&self) -> &Option<OrderInfo>;
    fn get_trigger_price(&self) -> Option<FPDecimal>;
    fn is_buy(&self) -> bool;
    fn is_sell(&self) -> bool;
}

impl SpotLimitOrder {
    pub fn new(order_info: OrderInfo, order_type: OrderType, fillable: FPDecimal, trigger_price: FPDecimal, order_hash: String) -> Self {
        SpotLimitOrder {
            order_info: Some(order_info),
            order_type: order_type as i32,
            fillable: fillable.to_string(),
            trigger_price: trigger_price.to_string(),
            order_hash: order_hash.into(),
        }
    }
}

impl GenericOrder for SpotLimitOrder {
    fn is_buy(&self) -> bool {
        self.order_type == OrderType::Buy as i32 || self.order_type == OrderType::BuyPo as i32 || self.order_type == OrderType::BuyAtomic as i32
    }

    fn is_sell(&self) -> bool {
        self.order_type == OrderType::Sell as i32 || self.order_type == OrderType::SellPo as i32 || self.order_type == OrderType::SellAtomic as i32
    }

    fn get_order_type(&self) -> OrderType {
        OrderType::from_i32(self.order_type)
    }

    fn get_order_info(&self) -> &Option<OrderInfo> {
        &self.order_info
    }

    fn get_trigger_price(&self) -> Option<FPDecimal> {
        Some(FPDecimal::must_from_str(&self.trigger_price))
    }
}

impl SpotOrder {
    pub fn new(
        price: FPDecimal,
        quantity: FPDecimal,
        order_type: OrderType,
        market_id: &MarketId,
        subaccount_id: SubaccountId,
        fee_recipient: String,
        cid: Option<String>,
    ) -> Self {
        SpotOrder {
            market_id: market_id.to_string(),
            order_info: Some(OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient,
                price: price.to_string(),
                quantity: quantity.to_string(),
                cid: cid.unwrap_or_default(),
            }),
            order_type: order_type as i32,
            trigger_price: "".to_string(),
        }
    }
}

impl GenericOrder for SpotOrder {
    fn is_buy(&self) -> bool {
        self.order_type == OrderType::Buy as i32 || self.order_type == OrderType::BuyPo as i32 || self.order_type == OrderType::BuyAtomic as i32
    }

    fn is_sell(&self) -> bool {
        self.order_type == OrderType::Sell as i32 || self.order_type == OrderType::SellPo as i32 || self.order_type == OrderType::SellAtomic as i32
    }

    fn get_order_type(&self) -> OrderType {
        OrderType::from_i32(self.order_type)
    }

    fn get_order_info(&self) -> &Option<OrderInfo> {
        &self.order_info
    }

    fn get_trigger_price(&self) -> Option<FPDecimal> {
        Some(FPDecimal::must_from_str(&self.trigger_price))
    }
}
