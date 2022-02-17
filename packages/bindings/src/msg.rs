use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::InjectiveRoute;
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg};
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
            source_subaccount_id: source_subaccount_id.to_string(),
            destination_subaccount_id: destination_subaccount_id.to_string(),
            amount,
        },
    }
    .into()
}
