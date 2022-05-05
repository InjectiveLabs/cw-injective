use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::order::OrderInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl SpotLimitOrder {
    pub fn new(order_info: OrderInfo, order_type: i32, fillable: FPDecimal, trigger_price: Option<FPDecimal>, order_hash: String) -> Self {
        SpotLimitOrder {
            order_info,
            order_type,
            fillable,
            trigger_price,
            order_hash,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub trigger_price: Option<String>,
}
impl SpotOrder {
    pub fn new(price: FPDecimal, quantity: FPDecimal, is_buy: bool, market_id: &str, subaccount_id: &str, fee_recipient: &str) -> Self {
        SpotOrder {
            market_id: market_id.to_string(),
            order_info: OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient: fee_recipient.to_string(),
                price,
                quantity,
            },
            order_type: if is_buy { 1 } else { 2 },
            trigger_price: None,
        }
    }

    pub fn get_price(&self) -> FPDecimal {
        self.order_info.price
    }
    pub fn get_qty(&self) -> FPDecimal {
        self.order_info.quantity
    }
    pub fn get_val(&self) -> FPDecimal {
        self.get_price() * self.get_qty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotMarketOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl SpotMarketOrder {
    pub fn new(order_info: OrderInfo, order_type: i32, fillable: FPDecimal, trigger_price: Option<FPDecimal>, order_hash: String) -> Self {
        SpotMarketOrder {
            order_info,
            order_type,
            fillable,
            trigger_price,
            order_hash,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrimmedSpotLimitOrder {
    pub price: FPDecimal,
    pub quantity: FPDecimal,
    pub fillable: FPDecimal,
    pub isBuy: bool,
    pub order_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TraderSpotOrdersResponse {
    pub orders: Option<Vec<TrimmedSpotLimitOrder>>,
}
