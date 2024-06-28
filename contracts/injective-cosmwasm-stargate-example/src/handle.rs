use crate::{
    contract::{CREATE_DERIVATIVE_ORDER_REPLY_ID, CREATE_SPOT_ORDER_REPLY_ID},
    msg::{MSG_CREATE_DERIVATIVE_LIMIT_ORDER_ENDPOINT, MSG_CREATE_SPOT_LIMIT_ORDER_ENDPOINT},
    order_management::{create_derivative_limit_order, create_spot_limit_order, create_stargate_msg, encode_bytes_message},
    spot_market_order_msg::create_spot_market_order_message,
    state::{CacheOrderInfo, ORDER_CALL_CACHE},
    ContractError,
};
use cosmos_sdk_proto::{cosmos::authz::v1beta1::MsgExec, traits::Message, Any};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, SubMsg};
use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper, MarketId, OrderType, SubaccountId};
use injective_math::{scale::Scaled, FPDecimal};

pub const MSG_EXEC: &str = "/cosmos.authz.v1beta1.MsgExec";

pub fn handle_test_market_spot_order(
    deps: DepsMut<InjectiveQueryWrapper>,
    sender: &str,
    market_id: MarketId,
    subaccount_id: SubaccountId,
    price: String,
    quantity: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let querier = InjectiveQuerier::new(&deps.querier);
    let spot_market = querier.query_spot_market(&market_id).unwrap().market.unwrap();

    let order_msg = create_spot_market_order_message(
        FPDecimal::must_from_str(price.as_str()),
        FPDecimal::must_from_str(quantity.as_str()),
        OrderType::Sell,
        sender,
        subaccount_id.as_str(),
        "",
        &spot_market,
    )?;

    Ok(Response::new().add_message(order_msg))
}

pub fn handle_test_transient_spot_order(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: &MessageInfo,
    market_id: MarketId,
    subaccount_id: SubaccountId,
    price: String,
    quantity: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let querier = InjectiveQuerier::new(&deps.querier);
    let spot_market = querier.query_spot_market(&market_id).unwrap().market.unwrap();

    let order_msg = create_spot_limit_order(
        FPDecimal::must_from_str(price.as_str()).scaled(18i32),
        FPDecimal::must_from_str(quantity.as_str()).scaled(18i32),
        OrderType::Sell,
        info.sender.as_str(),
        subaccount_id.as_str(),
        &spot_market,
    );

    let order_bytes = encode_bytes_message(&order_msg).unwrap();

    let msg_exec = MsgExec {
        grantee: env.contract.address.to_string(),
        msgs: vec![Any {
            type_url: MSG_CREATE_SPOT_LIMIT_ORDER_ENDPOINT.to_string(),
            value: order_bytes,
        }],
    };

    let order_submessage = SubMsg::reply_on_success(
        create_stargate_msg(MSG_EXEC, msg_exec.encode_to_vec()).unwrap(),
        CREATE_SPOT_ORDER_REPLY_ID,
    );

    save_cache_info(deps, market_id, subaccount_id)?;

    Ok(Response::new().add_submessage(order_submessage))
}

pub fn handle_test_transient_derivative_order(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: &MessageInfo,
    market_id: MarketId,
    subaccount_id: SubaccountId,
    price: String,
    quantity: String,
    margin: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let querier: InjectiveQuerier = InjectiveQuerier::new(&deps.querier);
    let market = querier.query_derivative_market(&market_id).unwrap().market.unwrap();

    let order_msg = create_derivative_limit_order(
        FPDecimal::must_from_str(price.as_str()).scaled(18i32),
        FPDecimal::must_from_str(quantity.as_str()).scaled(18i32),
        FPDecimal::must_from_str(margin.as_str()).scaled(18i32),
        OrderType::Buy,
        info.sender.as_str(),
        subaccount_id.as_str(),
        &market,
    );

    let order_bytes = encode_bytes_message(&order_msg).unwrap();

    let msg_exec = MsgExec {
        grantee: env.contract.address.to_string(),
        msgs: vec![Any {
            type_url: MSG_CREATE_DERIVATIVE_LIMIT_ORDER_ENDPOINT.to_string(),
            value: order_bytes,
        }],
    };

    let order_submessage = SubMsg::reply_on_success(
        create_stargate_msg(MSG_EXEC, msg_exec.encode_to_vec()).unwrap(),
        CREATE_DERIVATIVE_ORDER_REPLY_ID,
    );

    save_cache_info(deps, market_id, subaccount_id)?;

    Ok(Response::new().add_submessage(order_submessage))
}

fn save_cache_info(deps: DepsMut<InjectiveQueryWrapper>, market_id: MarketId, subaccount_id: SubaccountId) -> Result<(), ContractError> {
    let cache_order_info = CacheOrderInfo {
        subaccount: subaccount_id,
        market_id,
    };

    let mut order_cache = match ORDER_CALL_CACHE.may_load(deps.storage)? {
        Some(order_cache) => order_cache,
        None => vec![],
    };

    order_cache.push(cache_order_info);

    ORDER_CALL_CACHE.save(deps.storage, &order_cache)?;
    Ok(())
}
