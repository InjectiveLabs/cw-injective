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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub trigger_price: Option<String>,
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
