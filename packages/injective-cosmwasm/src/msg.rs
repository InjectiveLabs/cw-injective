use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{derivative::DerivativeOrder, order::OrderData, route::InjectiveRoute, spot::SpotOrder};
use crate::{MarketId, SubaccountId};
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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

/// InjectiveMsg is an override of CosmosMsg::Custom to add support for Injective's custom message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InjectiveMsg {
    Deposit {
        sender: Addr,
        subaccount_id: SubaccountId,
        amount: Coin,
    },
    Withdraw {
        sender: Addr,
        subaccount_id: SubaccountId,
        amount: Coin,
    },
    SubaccountTransfer {
        sender: Addr,
        source_subaccount_id: SubaccountId,
        destination_subaccount_id: SubaccountId,
        amount: Coin,
    },
    ExternalTransfer {
        sender: Addr,
        source_subaccount_id: SubaccountId,
        destination_subaccount_id: SubaccountId,
        amount: Coin,
    },
    CreateSpotMarketOrder {
        sender: Addr,
        order: SpotOrder,
    },
    CreateDerivativeMarketOrder {
        sender: Addr,
        order: DerivativeOrder,
    },
    IncreasePositionMargin {
        sender: Addr,
        source_subaccount_id: SubaccountId,
        destination_subaccount_id: SubaccountId,
        market_id: MarketId,
        amount: Coin,
    },
    LiquidatePosition {
        sender: Addr,
        subaccount_id: SubaccountId,
        market_id: MarketId,
        order: Option<DerivativeOrder>,
    },
    RegisterAsDMM {
        sender: Addr,
        dmm_account: String,
    },
    BatchUpdateOrders {
        sender: Addr,
        subaccount_id: Option<SubaccountId>,
        spot_market_ids_to_cancel_all: Vec<MarketId>,
        derivative_market_ids_to_cancel_all: Vec<MarketId>,
        spot_orders_to_cancel: Vec<OrderData>,
        derivative_orders_to_cancel: Vec<OrderData>,
        spot_orders_to_create: Vec<SpotOrder>,
        derivative_orders_to_create: Vec<DerivativeOrder>,
    },
    /// Contracts can mint native tokens for an existing factory denom
    /// that they are the admin of.
    Mint {
        sender: Addr,
        amount: Coin,
        mint_to: String,
    },
    /// Contracts can burn native tokens for an existing factory denom
    /// that they are the admin of.
    /// Currently, the burn from address must be the admin contract.
    Burn {
        sender: Addr,
        amount: Coin,
    },
}

pub fn create_deposit_msg(sender: Addr, subaccount_id: SubaccountId, amount: Coin) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::Deposit {
            sender,
            subaccount_id,
            amount,
        },
    }
    .into()
}

pub fn create_withdraw_msg(sender: Addr, subaccount_id: SubaccountId, amount: Coin) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::Withdraw {
            sender,
            subaccount_id,
            amount,
        },
    }
    .into()
}

pub fn create_subaccount_transfer_msg(
    sender: Addr,
    source_subaccount_id: SubaccountId,
    destination_subaccount_id: SubaccountId,
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

pub fn create_external_transfer_msg(
    sender: Addr,
    source_subaccount_id: SubaccountId,
    destination_subaccount_id: SubaccountId,
    amount: Coin,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::ExternalTransfer {
            sender,
            source_subaccount_id,
            destination_subaccount_id,
            amount,
        },
    }
    .into()
}

pub fn create_spot_market_order_msg(sender: Addr, order: SpotOrder) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::CreateSpotMarketOrder { sender, order },
    }
    .into()
}

pub fn create_derivative_market_order_msg(sender: Addr, order: DerivativeOrder) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::CreateDerivativeMarketOrder { sender, order },
    }
    .into()
}

pub fn create_increase_position_margin_msg(
    sender: Addr,
    source_subaccount_id: SubaccountId,
    destination_subaccount_id: SubaccountId,
    market_id: MarketId,
    amount: Coin,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::IncreasePositionMargin {
            sender,
            source_subaccount_id,
            destination_subaccount_id,
            market_id,
            amount,
        },
    }
    .into()
}

pub fn create_liquidate_position_msg(
    sender: Addr,
    subaccount_id: SubaccountId,
    market_id: MarketId,
    order: Option<DerivativeOrder>,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::LiquidatePosition {
            sender,
            subaccount_id,
            market_id,
            order,
        },
    }
    .into()
}

pub fn create_register_as_dmm_msg(sender: Addr, dmm_account: String) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::RegisterAsDMM { sender, dmm_account },
    }
    .into()
}

pub fn create_batch_update_orders_msg(
    sender: Addr,
    subaccount_id: Option<SubaccountId>,
    spot_market_ids_to_cancel_all: Vec<MarketId>,
    derivative_market_ids_to_cancel_all: Vec<MarketId>,
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

pub fn create_mint_tokens_msg(sender: Addr, amount: Coin, mint_to: String) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Tokenfactory,
        msg_data: InjectiveMsg::Mint { sender, amount, mint_to },
    }
    .into()
}

pub fn create_burn_tokens_msg(sender: Addr, amount: Coin, _burn_from_address: String) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Tokenfactory,
        msg_data: InjectiveMsg::Burn { sender, amount },
    }
    .into()
}
