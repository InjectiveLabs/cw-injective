use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    BankMsg, Coin, DepsMut, Env, MessageInfo, Reply, Response, StdError, SubMsg, Uint128,
};
use cw2::set_contract_version;

use injective_cosmwasm::{
    create_deposit_msg, create_spot_market_order_msg, create_withdraw_msg,
    get_default_subaccount_id_for_checked_address, InjectiveMsgWrapper, InjectiveQuerier,
    InjectiveQueryWrapper, SpotOrder,
};
use injective_math::FPDecimal;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::proto_parser::{parse_protobuf_bytes, parse_protobuf_string, ResultToStdErrExt};
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
    if let Some(market) = querier.query_spot_market(msg.market_id.clone())?.market {
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
            val: format!(
                "Deposit: {} below min_deposit: {}",
                message_deposit, min_deposit
            ),
        });
    }
    let order = SpotOrder::new(
        price,
        quantity,
        true,
        false,
        true,
        &config.market_id,
        &subaccount_id,
        &contract,
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
) -> Result<Response<InjectiveMsgWrapper>, StdError> {
    match msg.id {
        ATOMIC_ORDER_REPLY_ID => handle_atomic_order_reply(deps, env, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_atomic_order_reply(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    msg: Reply,
) -> Result<Response<InjectiveMsgWrapper>, StdError> {
    let dec_scale_factor: FPDecimal = FPDecimal::from(1000000000000000000_i128);
    let mut data = msg.result.unwrap().data.unwrap().to_vec();
    let _ = parse_protobuf_string(&mut data, 1); // order hash - we need to read to advance reader

    let trade_result = parse_protobuf_bytes(&mut data, 2).with_stderr()?;
    let mut trade_data = trade_result.unwrap().to_vec();
    let field1 = parse_protobuf_string(&mut trade_data, 1).with_stderr()?;
    let field2 = parse_protobuf_string(&mut trade_data, 2).with_stderr()?;
    let field3 = parse_protobuf_string(&mut trade_data, 3).with_stderr()?;
    let quantity = FPDecimal::from_str(&field1)? / dec_scale_factor;
    let price = FPDecimal::from_str(&field2)? / dec_scale_factor;
    let fee = FPDecimal::from_str(&field3)? / dec_scale_factor;

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
