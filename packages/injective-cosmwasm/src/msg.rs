use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::InjectiveRoute;
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg, Decimal256 as Decimal};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InjectiveMsgWrapper {
    pub route: InjectiveRoute,
    pub msg_data: InjectiveMsg,
}

impl From<InjectiveMsgWrapper> for CosmosMsg<InjectiveMsgWrapper> {
    fn from(s: InjectiveMsgWrapper) -> CosmosMsg<InjectiveMsgWrapper> {
        CosmosMsg::Custom(s)
    }
}

impl CustomMsg for InjectiveMsgWrapper {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderData {
    pub market_id: String,
    pub subaccount_id: String,
    pub order_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderInfo {
    pub subaccount_id: String,
    pub fee_recipient: String,
    pub price: Decimal,
    pub quantity: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub trigger_price: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: Decimal,
    pub trigger_price: Option<String>,
}

impl DerivativeOrder {
    pub fn new(
        price: Decimal,
        quantity: Decimal,
        margin: Decimal,
        is_buy: bool,
        market_id: &str,
        subaccount_id: &str,
        fee_recipient: &str,
    ) -> DerivativeOrder {
        DerivativeOrder {
            market_id: market_id.to_string(),
            order_info: OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient: fee_recipient.to_string(),
                price,
                quantity,
            },
            order_type: if is_buy { 1 } else { 2 }, // TODO PO-orders
            margin,
            trigger_price: None,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
    pub fn get_price(&self) -> Decimal {
        self.order_info.price
    }
    pub fn get_qty(&self) -> Decimal {
        self.order_info.quantity
    }
    pub fn get_val(&self) -> Decimal {
        self.get_price() * self.get_qty()
    }
    pub fn get_margin(&self) -> Decimal {
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

/// InjectiveMsg is an override of CosmosMsg::Custom to add support for Injective's custom message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InjectiveMsg {
    SubaccountTransfer {
        sender: Addr,
        source_subaccount_id: String,
        destination_subaccount_id: String,
        amount: Coin,
    },
    BatchUpdateOrders {
        sender: String,
        subaccount_id: String,
        spot_market_ids_to_cancel_all: Vec<String>,
        derivative_market_ids_to_cancel_all: Vec<String>,
        spot_orders_to_cancel: Vec<OrderData>,
        derivative_orders_to_cancel: Vec<OrderData>,
        spot_orders_to_create: Vec<SpotOrder>,
        derivative_orders_to_create: Vec<DerivativeOrder>,
    },
    CreateDerivativeMarketOrder {
        sender: String,
        order: DerivativeOrder,
    },
}

pub fn create_subaccount_transfer_msg(
    sender: Addr,
    source_subaccount_id: String,
    destination_subaccount_id: String,
    amount: Coin,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::SubaccountTransfer {
            sender,
            source_subaccount_id,
            destination_subaccount_id,
            amount,
        },
    }
    .into()
}

pub fn create_batch_update_orders_msg(
    sender: String,
    subaccount_id: String,
    spot_market_ids_to_cancel_all: Vec<String>,
    derivative_market_ids_to_cancel_all: Vec<String>,
    spot_orders_to_cancel: Vec<OrderData>,
    derivative_orders_to_cancel: Vec<OrderData>,
    spot_orders_to_create: Vec<SpotOrder>,
    derivative_orders_to_create: Vec<DerivativeOrder>,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::BatchUpdateOrders {
            sender,
            subaccount_id,
            spot_market_ids_to_cancel_all,
            derivative_market_ids_to_cancel_all,
            spot_orders_to_cancel,
            derivative_orders_to_cancel,
            spot_orders_to_create,
            derivative_orders_to_create,
        },
    }
    .into()
}

pub fn create_derivative_market_order_msg(sender: String, order: DerivativeOrder) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::CreateDerivativeMarketOrder { sender, order },
    }
    .into()
}
