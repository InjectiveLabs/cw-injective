use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::order::OrderInfo;
use crate::{MarketId, SubaccountId};

pub enum OrderType {
    Undefined = 0,
    Buy = 1,
    Sell = 2,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Position {
    #[serde(default)]
    pub isLong: bool,
    pub quantity: FPDecimal,
    pub entry_price: FPDecimal,
    #[serde(default)]
    pub margin: FPDecimal,
    pub cumulative_funding_entry: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct EffectivePosition {
    #[serde(default)]
    pub is_long: bool,
    pub quantity: FPDecimal,
    pub entry_price: FPDecimal,
    #[serde(default)]
    pub effective_margin: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativePosition {
    pub subaccount_id: SubaccountId,
    pub market_id: MarketId,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeOrder {
    pub market_id: MarketId,
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
        order_type: OrderType,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        fee_recipient: Option<Addr>,
    ) -> Self {
        DerivativeOrder {
            market_id,
            order_info: OrderInfo {
                subaccount_id,
                fee_recipient,
                price,
                quantity,
            },
            order_type: order_type as i32,
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
    pub fn is_invalid(&self, is_reduce_only: bool) -> bool {
        if is_reduce_only && !self.margin.is_zero() {
            return true;
        }

        if !is_reduce_only && self.margin.is_zero() {
            return true;
        }

        self.get_price().is_zero() || self.get_qty().is_zero()
    }
    pub fn get_order_type(&self) -> OrderType {
        match self.order_type {
            1 => OrderType::Buy,
            2 => OrderType::Sell,
            7 => OrderType::BuyPo,
            8 => OrderType::SellPo,
            _ => OrderType::Undefined,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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
        order_type: OrderType,
        order_info: OrderInfo,
    ) -> Self {
        DerivativeLimitOrder {
            margin,
            fillable,
            order_hash,
            trigger_price,
            order_type: order_type as i32,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }

    pub fn get_order_type(&self) -> OrderType {
        match self.order_type {
            1 => OrderType::Buy,
            2 => OrderType::Sell,
            7 => OrderType::BuyPo,
            8 => OrderType::SellPo,
            _ => OrderType::Undefined,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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
        order_type: OrderType,
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
            order_type: order_type as i32,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }

    pub fn get_order_type(&self) -> OrderType {
        match self.order_type {
            1 => OrderType::Buy,
            2 => OrderType::Sell,
            7 => OrderType::BuyPo,
            8 => OrderType::SellPo,
            _ => OrderType::Undefined,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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
