use crate::order::OrderInfo;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
    #[serde(default)]
    pub isLong: bool,
    pub quantity: FPDecimal,
    pub entry_price: FPDecimal,
    #[serde(default)]
    pub margin: FPDecimal,
    pub cumulative_funding_entry: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EffectivePosition {
    #[serde(default)]
    pub is_long: bool,
    pub quantity: FPDecimal,
    pub entry_price: FPDecimal,
    #[serde(default)]
    pub effective_margin: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativePosition {
    pub subaccount_id: String,
    pub market_id: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: FPDecimal,
    pub trigger_price: Option<String>,
}

impl DerivativeOrder {
    pub fn new(
        price: FPDecimal,
        quantity: FPDecimal,
        margin: FPDecimal,
        is_buy: bool,
        is_po: bool,
        market_id: &str,
        subaccount_id: &str,
        fee_recipient: &str,
    ) -> Self {
        DerivativeOrder {
            market_id: market_id.to_string(),
            order_info: OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient: fee_recipient.to_string(),
                price,
                quantity,
            },
            order_type: if is_buy && !is_po {
                1
            } else if !is_buy && !is_po {
                2
            } else if is_buy && is_po {
                7
            } else {
                8
            },
            margin,
            trigger_price: None,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
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
    pub fn get_margin(&self) -> FPDecimal {
        self.margin
    }
    pub fn non_reduce_only_is_invalid(&self) -> bool {
        self.get_margin().is_zero() || self.get_price().is_zero() || self.get_qty().is_zero()
    }
    pub fn reduce_only_price_is_invalid(&self) -> bool {
        self.get_price().is_zero()
    }
    pub fn reduce_only_qty_is_invalid(&self) -> bool {
        self.get_qty().is_zero()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: FPDecimal,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl DerivativeLimitOrder {
    pub fn new(
        margin: FPDecimal,
        fillable: FPDecimal,
        order_hash: String,
        trigger_price: Option<FPDecimal>,
        order_type: i32,
        order_info: OrderInfo,
    ) -> Self {
        DerivativeLimitOrder {
            margin,
            fillable,
            order_hash,
            trigger_price,
            order_type,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeMarketOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: FPDecimal,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl DerivativeMarketOrder {
    pub fn new(
        order_info: OrderInfo,
        order_type: i32,
        margin: FPDecimal,
        fillable: FPDecimal,
        trigger_price: Option<FPDecimal>,
        order_hash: String,
    ) -> Self {
        DerivativeMarketOrder {
            margin,
            fillable,
            order_hash,
            trigger_price,
            order_type,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrimmedDerivativeLimitOrder {
    pub price: FPDecimal,
    pub quantity: FPDecimal,
    #[serde(default)]
    pub margin: FPDecimal,
    pub fillable: FPDecimal,
    #[serde(default)]
    pub isBuy: bool,
    pub order_hash: String,
}
