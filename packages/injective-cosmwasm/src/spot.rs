use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_math::FPDecimal;

use crate::order::OrderInfo;
use crate::OrderType;
use crate::{MarketId, SubaccountId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SpotOrder {
    pub market_id: MarketId,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub trigger_price: Option<String>,
}

impl SpotOrder {
    pub fn new(
        price: FPDecimal,
        quantity: FPDecimal,
        is_buy: bool,
        is_po: bool,
        is_atomic: bool,
        market_id: &MarketId,
        subaccount_id: &SubaccountId,
        fee_recipient: &str,
    ) -> Self {
        SpotOrder {
            market_id: market_id.clone(),
            order_info: OrderInfo {
                subaccount_id: subaccount_id.clone(),
                fee_recipient: fee_recipient.to_string(),
                price,
                quantity,
            },
            order_type: match (is_buy, is_po, is_atomic) {
                (true, false, false) => OrderType::Buy as i32,
                (true, true, _) => OrderType::BuyPo as i32,
                (true, _, true) => OrderType::BuyAtomic as i32,
                (false, false, false) => OrderType::Sell as i32,
                (false, true, _) => OrderType::SellPo as i32,
                (false, _, true) => OrderType::SellAtomic as i32,
            },
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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
