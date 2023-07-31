use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, CustomMsg, Deps, StdError, StdResult};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::exchange::{
    derivative::{derivative_order_to_short, ShortDerivativeOrder},
    order::{order_data_to_short, OrderData, ShortOrderData},
    privileged_action::coins_to_string,
    spot::{spot_order_to_short, ShortSpotOrder, SpotOrder},
    subaccount::{is_default_subaccount, subaccount_id_to_injective_address},
    types::{MarketId, SubaccountId},
};
use crate::InjectiveQueryWrapper;
use crate::{exchange::derivative::DerivativeOrder, oracle::types::PriceAttestation, route::InjectiveRoute};

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
    CancelDerivativeOrder {
        sender: Addr,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        order_hash: String,
        order_mask: i32,
    },
    CancelSpotOrder {
        sender: Addr,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        order_hash: String,
    },
    IncreasePositionMargin {
        sender: Addr,
        source_subaccount_id: SubaccountId,
        destination_subaccount_id: SubaccountId,
        market_id: MarketId,
        amount: FPDecimal,
    },
    PrivilegedExecuteContract {
        sender: Addr,
        funds: String, // TODO consider adding custom Vec<Coin> type with custom serializer using coins_to_string
        contract_address: Addr,
        data: String,
    },
    LiquidatePosition {
        sender: Addr,
        subaccount_id: SubaccountId,
        market_id: MarketId,
        order: Option<DerivativeOrder>,
    },
    RewardsOptOut {
        sender: Addr,
    },
    BatchUpdateOrders {
        sender: Addr,
        subaccount_id: Option<SubaccountId>,
        spot_market_ids_to_cancel_all: Vec<MarketId>,
        derivative_market_ids_to_cancel_all: Vec<MarketId>,
        spot_orders_to_cancel: Vec<ShortOrderData>,
        derivative_orders_to_cancel: Vec<ShortOrderData>,
        spot_orders_to_create: Vec<ShortSpotOrder>,
        derivative_orders_to_create: Vec<ShortDerivativeOrder>,
    },
    RelayPythPrices {
        sender: Addr,
        price_attestations: Vec<PriceAttestation>,
    },
    CreateDenom {
        sender: String,
        subdenom: String,
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
    /// Sets metadata of token-factory token
    SetTokenMetadata {
        denom: String,
        name: String,
        symbol: String,
        decimals: u8,
    },
    /// Wasmx - update contract
    UpdateContract {
        sender: Addr,
        contract_address: Addr,
        gas_limit: u64,
        gas_price: u64,
        admin_address: String,
    },
    ActivateContract {
        sender: Addr,
        contract_address: Addr,
    },
    DeactivateContract {
        sender: Addr,
        contract_address: Addr,
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
    deps: &Deps<InjectiveQueryWrapper>,
    source_subaccount_id: &SubaccountId,
    destination_subaccount_id: &SubaccountId,
    amount: &Coin,
) -> StdResult<Vec<CosmosMsg<InjectiveMsgWrapper>>> {
    let sender = subaccount_id_to_injective_address(source_subaccount_id, deps)?;
    let to_address = subaccount_id_to_injective_address(destination_subaccount_id, deps)?;

    let is_external_transfer = sender != to_address;
    if is_external_transfer {
        return create_external_transfer_msg(deps, source_subaccount_id, destination_subaccount_id, amount);
    }

    if is_default_subaccount(destination_subaccount_id) {
        return Ok(vec![create_withdraw_msg(sender, source_subaccount_id.to_owned(), amount.to_owned())]);
    }

    Ok(vec![InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::SubaccountTransfer {
            sender,
            source_subaccount_id: source_subaccount_id.to_owned(),
            destination_subaccount_id: destination_subaccount_id.to_owned(),
            amount: amount.to_owned(),
        },
    }
    .into()])
}

pub fn create_external_transfer_msg(
    deps: &Deps<InjectiveQueryWrapper>,
    source_subaccount_id: &SubaccountId,
    destination_subaccount_id: &SubaccountId,
    amount: &Coin,
) -> StdResult<Vec<CosmosMsg<InjectiveMsgWrapper>>> {
    let sender = subaccount_id_to_injective_address(source_subaccount_id, deps)?;

    if is_default_subaccount(destination_subaccount_id) {
        let to_address = subaccount_id_to_injective_address(destination_subaccount_id, deps)?;

        let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: to_address.to_string(),
            amount: vec![amount.to_owned()],
        });

        if is_default_subaccount(source_subaccount_id) {
            return Ok(vec![bank_send_msg]);
        }

        let withdraw_msg = create_withdraw_msg(sender, source_subaccount_id.to_owned(), amount.to_owned());
        return Ok(vec![withdraw_msg, bank_send_msg]);
    }

    if is_default_subaccount(source_subaccount_id) {
        return Err(StdError::generic_err("Cannot send from default subaccount to external subaccount"));
    }

    Ok(vec![InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::ExternalTransfer {
            sender,
            source_subaccount_id: source_subaccount_id.to_owned(),
            destination_subaccount_id: destination_subaccount_id.to_owned(),
            amount: amount.to_owned(),
        },
    }
    .into()])
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

pub fn cancel_spot_order_msg(sender: Addr, market_id: MarketId, subaccount_id: SubaccountId, order_hash: String) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::CancelSpotOrder {
            sender,
            market_id,
            subaccount_id,
            order_hash,
        },
    }
    .into()
}

pub fn cancel_derivative_order_msg(
    sender: Addr,
    market_id: MarketId,
    subaccount_id: SubaccountId,
    order_hash: String,
    order_mask: i32,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::CancelDerivativeOrder {
            sender,
            market_id,
            subaccount_id,
            order_hash,
            order_mask,
        },
    }
    .into()
}

pub fn create_increase_position_margin_msg(
    sender: Addr,
    source_subaccount_id: SubaccountId,
    destination_subaccount_id: SubaccountId,
    market_id: MarketId,
    amount: FPDecimal,
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

pub fn create_privileged_execute_contract_msg(
    sender: Addr,
    funds: Vec<Coin>,
    contract_address: Addr,
    data: String,
) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::PrivilegedExecuteContract {
            sender,
            funds: coins_to_string(funds),
            contract_address,
            data,
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

pub fn create_rewards_opt_out_msg(sender: Addr) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Exchange,
        msg_data: InjectiveMsg::RewardsOptOut { sender },
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
            spot_orders_to_cancel: order_data_to_short(spot_orders_to_cancel),
            derivative_orders_to_cancel: order_data_to_short(derivative_orders_to_cancel),
            spot_orders_to_create: spot_order_to_short(spot_orders_to_create),
            derivative_orders_to_create: derivative_order_to_short(derivative_orders_to_create),
        },
    }
    .into()
}

pub fn create_relay_pyth_prices_msg(sender: Addr, price_attestations: Vec<PriceAttestation>) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Oracle,
        msg_data: InjectiveMsg::RelayPythPrices { sender, price_attestations },
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

pub fn create_burn_tokens_msg(sender: Addr, amount: Coin) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Tokenfactory,
        msg_data: InjectiveMsg::Burn { sender, amount },
    }
    .into()
}

pub fn create_new_denom_msg(sender: String, subdenom: String) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Tokenfactory,
        msg_data: InjectiveMsg::CreateDenom { sender, subdenom },
    }
    .into()
}

pub fn create_set_token_metadata_msg(denom: String, name: String, symbol: String, decimals: u8) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Tokenfactory,
        msg_data: InjectiveMsg::SetTokenMetadata {
            denom,
            name,
            symbol,
            decimals,
        },
    }
    .into()
}

pub fn create_update_contract_msg(
    sender: Addr,
    contract_address: Addr,
    gas_limit: u64,
    gas_price: u64,
    admin_address: Option<Addr>,
) -> CosmosMsg<InjectiveMsgWrapper> {
    let admin = match admin_address {
        None => "".to_string(),
        Some(addr) => addr.to_string(),
    };
    InjectiveMsgWrapper {
        route: InjectiveRoute::Wasmx,
        msg_data: InjectiveMsg::UpdateContract {
            sender,
            contract_address,
            gas_limit,
            gas_price,
            admin_address: admin,
        },
    }
    .into()
}

pub fn create_activate_contract_msg(sender: Addr, contract_address: Addr) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Wasmx,
        msg_data: InjectiveMsg::ActivateContract { sender, contract_address },
    }
    .into()
}

pub fn create_deactivate_contract_msg(sender: Addr, contract_address: Addr) -> CosmosMsg<InjectiveMsgWrapper> {
    InjectiveMsgWrapper {
        route: InjectiveRoute::Wasmx,
        msg_data: InjectiveMsg::DeactivateContract { sender, contract_address },
    }
    .into()
}
