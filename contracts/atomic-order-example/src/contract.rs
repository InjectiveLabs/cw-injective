#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, Uint128};
use cw2::set_contract_version;
use protobuf::Message;
use std::str::FromStr;

use injective_cosmwasm::{
    create_deposit_msg, create_spot_market_order_msg, create_withdraw_msg,
    get_default_subaccount_id_for_checked_address, InjectiveMsgWrapper, InjectiveQuerier,
    InjectiveQueryWrapper, OrderType, SpotOrder,
};
use injective_math::FPDecimal;
use injective_protobuf::proto::tx;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{ContractConfigState, SwapCacheState, STATE, SWAP_OPERATION_STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atomic-order-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ATOMIC_ORDER_REPLY_ID: u64 = 1u64;
pub const DEPOSIT_REPLY_ID: u64 = 2u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let querier = InjectiveQuerier::new(&deps.querier);
    if let Some(market) = querier.query_spot_market(&msg.market_id)?.market {
        let state = ContractConfigState {
            market_id: msg.market_id,
            base_denom: market.base_denom,
            quote_denom: market.quote_denom,
            owner: info.sender.clone(),
            contract_subaccount_id: get_default_subaccount_id_for_checked_address(
                &env.contract.address,
            ),
        };
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        STATE.save(deps.storage, &state)?;

        Ok(Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("owner", info.sender))
    } else {
        Err(ContractError::CustomError {
            val: format!("Market with id: {} not found", msg.market_id.as_str()),
        })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::SwapSpot { quantity, price } => try_swap(deps, env, info, quantity, price),
    }
}

pub fn try_swap(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    quantity: FPDecimal,
    price: FPDecimal,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let config = STATE.load(deps.storage)?;
    let contract = env.contract.address;
    let subaccount_id = config.contract_subaccount_id;
    let min_deposit = price * quantity;
    if info.funds.is_empty() {
        return Err(ContractError::CustomError {
            val: "No funds deposited!".to_string(),
        });
    }
    let message_deposit = FPDecimal::from(info.funds[0].amount.u128());
    if message_deposit < min_deposit {
        return Err(ContractError::CustomError {
            val: format!("Deposit: {message_deposit} below min_deposit: {min_deposit}"),
        });
    }
    let order = SpotOrder::new(
        price,
        quantity,
        OrderType::BuyAtomic,
        &config.market_id,
        subaccount_id.to_owned(),
        Some(contract.to_owned()),
    );

    let coins = &info.funds[0];
    let deposit_message = SubMsg::new(create_deposit_msg(
        contract.clone(),
        subaccount_id,
        coins.clone(),
    ));
    let order_message = SubMsg::reply_on_success(
        create_spot_market_order_msg(contract, order),
        ATOMIC_ORDER_REPLY_ID,
    );
    let response = Response::new()
        .add_submessage(deposit_message)
        .add_submessage(order_message);

    let cache = SwapCacheState {
        sender_address: info.sender.to_string(),
        deposited_amount: coins.clone(),
    };
    SWAP_OPERATION_STATE.save(deps.storage, &cache)?;

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    msg: Reply,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg.id {
        ATOMIC_ORDER_REPLY_ID => handle_atomic_order_reply(deps, env, msg),
        _ => Err(ContractError::UnrecognisedReply(msg.id)),
    }
}

fn handle_atomic_order_reply(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    msg: Reply,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let dec_scale_factor: FPDecimal = FPDecimal::from(1000000000000000000_i128);
    let id = msg.id;
    let order_response: tx::MsgCreateSpotMarketOrderResponse = Message::parse_from_bytes(
        msg.result
            .into_result()
            .map_err(ContractError::SubMsgFailure)?
            .data
            .ok_or_else(|| ContractError::ReplyParseFailure {
                id,
                err: "Missing reply data".to_owned(),
            })?
            .as_slice(),
    )
    .map_err(|err| ContractError::ReplyParseFailure {
        id,
        err: err.to_string(),
    })?;
    // unwrap results into trade_data
    let trade_data = match order_response.results.into_option() {
        Some(trade_data) => Ok(trade_data),
        None => Err(ContractError::CustomError {
            val: "No trade data in order response".to_string(),
        }),
    }?;
    let quantity = FPDecimal::from_str(&trade_data.quantity)? / dec_scale_factor;
    let price = FPDecimal::from_str(&trade_data.price)? / dec_scale_factor;
    let fee = FPDecimal::from_str(&trade_data.fee)? / dec_scale_factor;

    let config = STATE.load(deps.storage)?;
    let contract_address = env.contract.address;
    let subaccount_id = config.contract_subaccount_id;

    let cache = SWAP_OPERATION_STATE.load(deps.storage)?;

    let purchased_coins = Coin::new(u128::from(quantity), config.base_denom.clone());
    let paid = quantity * price + fee;
    let leftover = cache.deposited_amount.amount - Uint128::from(u128::from(paid));
    let leftover_coins = Coin::new(u128::from(leftover), config.quote_denom);
    // we need to withdraw coins from subaccount to main account so we can transfer them back to a user
    let withdraw_purchased_message = create_withdraw_msg(
        contract_address.clone(),
        subaccount_id.clone(),
        purchased_coins.clone(),
    );
    let withdraw_leftover_message =
        create_withdraw_msg(contract_address, subaccount_id, leftover_coins.clone());

    let send_message = BankMsg::Send {
        to_address: cache.sender_address,
        amount: vec![purchased_coins, leftover_coins],
    };

    let response = Response::new()
        .add_message(withdraw_purchased_message)
        .add_message(withdraw_leftover_message)
        .add_message(send_message);
    Ok(response)
}
