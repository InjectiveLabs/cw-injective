use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::order::{GenericOrder, OrderInfo};
use crate::OrderType;
use crate::{MarketId, SubaccountId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: OrderType,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl SpotLimitOrder {
    pub fn new(order_info: OrderInfo, order_type: OrderType, fillable: FPDecimal, trigger_price: Option<FPDecimal>, order_hash: String) -> Self {
        SpotLimitOrder {
            order_info,
            order_type,
            fillable,
            trigger_price,
            order_hash,
        }
    }
}

impl GenericOrder for SpotLimitOrder {
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
pub struct SpotOrder {
    pub market_id: MarketId,
    pub order_info: OrderInfo,
    pub order_type: OrderType,
    pub trigger_price: Option<FPDecimal>,
}

impl SpotOrder {
    pub fn new(
        price: FPDecimal,
        quantity: FPDecimal,
        order_type: OrderType,
        market_id: &MarketId,
        subaccount_id: SubaccountId,
        fee_recipient: Option<Addr>,
    ) -> Self {
        SpotOrder {
            market_id: market_id.clone(),
            order_info: OrderInfo {
                subaccount_id,
                fee_recipient,
                price,
                quantity,
            },
            order_type,
            trigger_price: None,
        }
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
    pub fn is_post_only(&self) -> bool {
        self.order_type == OrderType::BuyPo || self.order_type == OrderType::SellPo
    }
    pub fn is_atomic(&self) -> bool {
        self.order_type == OrderType::BuyAtomic || self.order_type == OrderType::SellAtomic
    }
}

impl GenericOrder for SpotOrder {
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
pub struct SpotMarketOrder {
    pub order_info: OrderInfo,
    pub order_type: OrderType,
    pub fillable: FPDecimal,
    pub trigger_price: Option<FPDecimal>,
    pub order_hash: String,
}

impl SpotMarketOrder {
    pub fn new(order_info: OrderInfo, order_type: OrderType, fillable: FPDecimal, trigger_price: Option<FPDecimal>, order_hash: String) -> Self {
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TrimmedSpotLimitOrder {
    pub price: FPDecimal,
    pub quantity: FPDecimal,
    pub fillable: FPDecimal,
    #[serde(default)]
    pub isBuy: bool,
    pub order_hash: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotMarketOrderResults {
    pub quantity: FPDecimal,
    pub price: FPDecimal,
    pub fee: FPDecimal,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MsgCreateSpotMarketOrderResponse {
    pub order_hash: String,
    pub results: SpotMarketOrderResults,
}
