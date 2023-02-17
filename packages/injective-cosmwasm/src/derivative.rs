use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::order::OrderInfo;
use crate::{GenericOrder, MarketId, SubaccountId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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

impl Position {
    pub fn get_position_value(&mut self, valuation_price: FPDecimal, cumulative_funding: FPDecimal) -> FPDecimal {
        if self.isLong {
            let pnl = self.quantity * (valuation_price - self.entry_price);
            let unrealized_funding = self.quantity * (self.cumulative_funding_entry - cumulative_funding);

            return self.margin + pnl + unrealized_funding;
        }

        let pnl = self.quantity * (self.entry_price - valuation_price);
        let unrealized_funding = self.quantity * (cumulative_funding - self.cumulative_funding_entry);

        self.margin + pnl + unrealized_funding
    }

    pub fn apply_funding(&mut self, cumulative_funding: FPDecimal) {
        let unrealized_funding = self.quantity
            * if self.isLong {
                self.cumulative_funding_entry - cumulative_funding
            } else {
                cumulative_funding - self.cumulative_funding_entry
            };

        self.margin += unrealized_funding;
        self.cumulative_funding_entry = cumulative_funding;
    }
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
    pub order_type: OrderType,
    pub margin: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
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
            order_type,
            margin,
            trigger_price: None,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
    pub fn is_post_only(&self) -> bool {
        self.order_type == OrderType::BuyPo || self.order_type == OrderType::SellPo
    }
    pub fn is_atomic(&self) -> bool {
        self.order_type == OrderType::BuyAtomic || self.order_type == OrderType::SellAtomic
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
        self.order_type.to_owned()
    }
}

impl GenericOrder for DerivativeOrder {
    fn get_order_type(&self) -> &OrderType {
        &self.order_type
    }

    fn get_order_info(&self) -> &OrderInfo {
        &self.order_info
    }

    fn get_trigger_price(&self) -> Option<FPDecimal> {
        self.trigger_price
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: OrderType,
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
            order_type,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }

    pub fn get_order_type(&self) -> OrderType {
        self.order_type.to_owned()
    }
}

impl GenericOrder for DerivativeLimitOrder {
    fn get_order_type(&self) -> &OrderType {
        &self.order_type
    }

    fn get_order_info(&self) -> &OrderInfo {
        &self.order_info
    }

    fn get_trigger_price(&self) -> Option<FPDecimal> {
        self.trigger_price
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DerivativeMarketOrder {
    pub order_info: OrderInfo,
    pub order_type: OrderType,
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
            order_type,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }

    pub fn get_order_type(&self) -> OrderType {
        self.order_type.to_owned()
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
