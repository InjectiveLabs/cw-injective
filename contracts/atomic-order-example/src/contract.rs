use std::cmp::min;
use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use cw_utils::parse_reply_execute_data;

use injective_cosmwasm::{
    address_to_subaccount_id, create_batch_update_orders_msg, create_deposit_msg,
    create_external_transfer_msg, create_spot_market_order_msg, default_subaccount_id,
    DerivativeOrder, InjectiveMsg, InjectiveMsgWrapper, MsgCreateSpotMarketOrderResponse,
    OrderData, OrderInfo, SpotMarketOrder, SpotOrder,
};
use injective_math::FPDecimal;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::proto_parser::{parse_protobuf_bytes, parse_protobuf_string};
use crate::state::{ContractConfigState, SwapCacheState, CACHE, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atomic-order-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ATOMIC_ORDER_REPLY_ID: u64 = 1u64;
pub const DEPOSIT_REPLY_ID: u64 = 2u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // TODO: add validation using market query
    let state = ContractConfigState {
        market_id: msg.market_id,
        base_denom: msg.base_denom,
        quote_denom: msg.quote_denom,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::SwapSpot { quantity, price } => try_swap(deps, _env, info, quantity, price),
    }
}

pub fn try_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    quantity: FPDecimal,
    price: FPDecimal,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    deps.api
        .debug(format!("SC ----> Funds info: {}", info.funds[0]).as_str());
    let config = STATE.load(deps.storage)?;
    let contract = env.contract.address;
    let subaccount_id = default_subaccount_id(&contract);
    let min_deposit = price * quantity;
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
        contract.as_str(),
    );

    let coins = &info.funds[0];
    let deposit_message = SubMsg::new(create_deposit_msg(
        contract.to_string(),
        subaccount_id,
        coins.clone(),
    ));
    let order_message = SubMsg::reply_on_success(
        create_spot_market_order_msg(contract.into_string(), order),
        ATOMIC_ORDER_REPLY_ID,
    );
    let response = Response::new()
        .add_submessage(deposit_message)
        .add_submessage(order_message);

    let cache = SwapCacheState { sender_address: info.sender.to_string(), deposited_amount: coins.clone() };
    CACHE.save(deps.storage, &cache)?;

    return Ok(response);
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response<InjectiveMsgWrapper>, StdError> {
    // deps.api.debug(format!("SC ----> Received reply (data): {}", msg.clone().result.unwrap().data.unwrap()).as_str(), );
    // let events = msg.clone().result.unwrap().events;
    // deps.api.debug(format!("SC ----> Received reply (events count): {}", events.len()).as_str());
    match msg.id {
        ATOMIC_ORDER_REPLY_ID => handle_atomic_order_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_atomic_order_reply(
    deps: DepsMut,
    msg: Reply,
) -> Result<Response<InjectiveMsgWrapper>, StdError> {
    let dec_scale_factor: FPDecimal = FPDecimal::from(1000000000000000000 as i128);
    let mut data = msg.result.unwrap().data.unwrap().to_vec();
    let _ = parse_protobuf_string(&mut data, 1); // order hash - we need to read to advance pointer

    let trade_result = parse_protobuf_bytes(&mut data, 2).unwrap();
    let mut trade_data = trade_result.unwrap().to_vec();
    let field1 = parse_protobuf_string(&mut trade_data, 1).unwrap();
    let field2 = parse_protobuf_string(&mut trade_data, 2).unwrap();
    let field3 = parse_protobuf_string(&mut trade_data, 3).unwrap();
    let quantity = FPDecimal::from_str(&field1)? / dec_scale_factor;
    let price = FPDecimal::from_str(&field2)? / dec_scale_factor;
    let fee = FPDecimal::from_str(&field3)? / dec_scale_factor;

    let config = STATE.load(deps.storage)?;
    let cache = CACHE.load(deps.storage)?;

    let recipient_subaccount_id = default_subaccount_id(&Addr::unchecked(&cache.sender_address));

    let msg1 = create_external_transfer_msg(
        config.owner.clone().into_string(),
        default_subaccount_id(&config.owner),
        recipient_subaccount_id.clone(),
        Coin::new(u128::from(quantity), config.base_denom.clone()),
    );

    let paid = quantity * price + fee;
    let to_send = cache.deposited_amount.amount - Uint128::from(u128::from(paid));
    let msg2 = create_external_transfer_msg(
        config.owner.clone().into_string(),
        default_subaccount_id(&config.owner),
        recipient_subaccount_id,
        Coin::new(u128::from(to_send), config.quote_denom),
    );

    let response = Response::new().add_message(msg1).add_message(msg2);
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    return Err(StdError::not_found("No queries defined"));
    // match msg {
    //     // QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    // }
}
//
// fn query_count(deps: Deps) -> StdResult<GetCountResponse> {
//     let state = STATE.load(deps.storage)?;
//     Ok(GetCountResponse { count: state.count })
// }
