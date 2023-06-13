use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::exchange::order::{GenericOrder, OrderInfo, OrderType};
use crate::exchange::types::{MarketId, SubaccountId};
use crate::ShortSubaccountId;

use super::order::{GenericTrimmedOrder, ShortOrderInfo};

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
    pub fn get_position_value_without_funding(&self, valuation_price: FPDecimal) -> FPDecimal {
        let pnl = if self.isLong {
            self.quantity * (valuation_price - self.entry_price)
        } else {
            self.quantity * (self.entry_price - valuation_price)
        };
        self.margin + pnl
    }

    pub fn get_position_value_with_funding(&self, valuation_price: FPDecimal, cumulative_funding: FPDecimal) -> FPDecimal {
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
    pub fn get_quantity(&self) -> FPDecimal {
        self.order_info.quantity
    }
    pub fn get_val(&self) -> FPDecimal {
        self.get_price() * self.get_quantity()
    }
    pub fn is_invalid(&self, is_reduce_only: bool) -> bool {
        if is_reduce_only && !self.margin.is_zero() {
            return true;
        }

        if !is_reduce_only && self.margin.is_zero() {
            return true;
        }

        self.get_price().is_zero() || self.get_quantity().is_zero()
    }

    pub fn get_order_type(&self) -> OrderType {
        self.order_type.to_owned()
    }
}

impl GenericOrder for DerivativeOrder {
    fn is_buy(&self) -> bool {
        self.order_type == OrderType::Buy || self.order_type == OrderType::BuyPo || self.order_type == OrderType::BuyAtomic
    }

    fn is_sell(&self) -> bool {
        self.order_type == OrderType::Sell || self.order_type == OrderType::SellPo || self.order_type == OrderType::SellAtomic
    }

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
pub struct ShortDerivativeOrder {
    pub market_id: MarketId,
    pub order_info: ShortOrderInfo,
    pub order_type: OrderType,
    pub margin: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
}

impl From<DerivativeOrder> for ShortDerivativeOrder {
    fn from(derivative_order: DerivativeOrder) -> Self {
        ShortDerivativeOrder {
            market_id: derivative_order.market_id,
            order_info: derivative_order.order_info.into(),
            order_type: derivative_order.order_type,
            trigger_price: derivative_order.trigger_price,
            margin: derivative_order.margin,
        }
    }
}

pub fn derivative_order_to_short(derivative_order: Vec<DerivativeOrder>) -> Vec<ShortDerivativeOrder> {
    derivative_order.into_iter().map(|item| item.into()).collect()
}

impl ShortDerivativeOrder {
    pub fn new(
        price: FPDecimal,
        quantity: FPDecimal,
        margin: FPDecimal,
        order_type: OrderType,
        market_id: MarketId,
        subaccount_id: ShortSubaccountId,
        fee_recipient: Option<Addr>,
    ) -> Self {
        ShortDerivativeOrder {
            market_id,
            order_info: ShortOrderInfo {
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
    pub fn get_quantity(&self) -> FPDecimal {
        self.order_info.quantity
    }
    pub fn get_val(&self) -> FPDecimal {
        self.get_price() * self.get_quantity()
    }
    pub fn is_invalid(&self, is_reduce_only: bool) -> bool {
        if is_reduce_only && !self.margin.is_zero() {
            return true;
        }

        if !is_reduce_only && self.margin.is_zero() {
            return true;
        }

        self.get_price().is_zero() || self.get_quantity().is_zero()
    }

    pub fn get_order_type(&self) -> OrderType {
        self.order_type.to_owned()
    }
    pub fn is_buy(&self) -> bool {
        self.order_type == OrderType::Buy || self.order_type == OrderType::BuyPo || self.order_type == OrderType::BuyAtomic
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
    fn is_buy(&self) -> bool {
        self.order_type == OrderType::Buy || self.order_type == OrderType::BuyPo || self.order_type == OrderType::BuyAtomic
    }

    fn is_sell(&self) -> bool {
        self.order_type == OrderType::Sell || self.order_type == OrderType::SellPo || self.order_type == OrderType::SellAtomic
    }

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

impl GenericTrimmedOrder for TrimmedDerivativeLimitOrder {
    fn is_buy(&self) -> bool {
        self.isBuy
    }

    fn is_sell(&self) -> bool {
        !self.isBuy
    }

    fn get_price(&self) -> FPDecimal {
        self.price
    }

    fn get_fillable_quantity(&self) -> FPDecimal {
        self.fillable
    }

    fn get_order_hash(&self) -> String {
        self.order_hash.to_owned()
    }
}
