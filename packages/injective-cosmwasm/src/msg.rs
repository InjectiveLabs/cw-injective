use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::InjectiveRoute;
use cosmwasm_std::{CosmosMsg, CustomMsg};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InjectiveMsgWrapper {
    pub route: InjectiveRoute,
    pub msg_data: InjectiveMsg,
}

impl Into<CosmosMsg<InjectiveMsgWrapper>> for InjectiveMsgWrapper {
    fn into(self) -> CosmosMsg<InjectiveMsgWrapper> {
        CosmosMsg::Custom(self)
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
    pub price: String,
    pub quantity: String,
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
    pub margin: String,
    pub trigger_price: Option<String>,
}

/// InjectiveMsg is an override of CosmosMsg::Custom to add support for Injective's custom message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InjectiveMsg {
    // SubaccountTransfer {
    //     sender: Addr,
    //     source_subaccount_id: String,
    //     destination_subaccount_id: String,
    //     amount: Coin,
    // },
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
}

// pub fn create_subaccount_transfer_msg(
//     sender: Addr,
//     source_subaccount_id: String,
//     destination_subaccount_id: String,
//     amount: Coin,
// ) -> CosmosMsg<InjectiveMsgWrapper> {
//     InjectiveMsgWrapper {
//         route: InjectiveRoute::Exchange,
//         msg_data: InjectiveMsg::SubaccountTransfer {
//             sender,
//             source_subaccount_id: source_subaccount_id.to_string(),
//             destination_subaccount_id: destination_subaccount_id.to_string(),
//             amount,
//         },
//     }
//     .into()
// }

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
            sender: sender.to_string(),
            subaccount_id: subaccount_id.to_string(),
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
