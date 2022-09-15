use std::cmp::min;
use std::str::FromStr;

use cosmwasm_std::{Binary, Deps, DepsMut, Env, from_binary, MessageInfo, Reply, Response, StdError, StdResult, SubMsg, to_binary, WasmMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::set_contract_version;
use cw_storage_plus::Item;
use cw_utils::parse_reply_execute_data;

use injective_cosmwasm::{address_to_subaccount_id, create_batch_update_orders_msg, create_deposit_msg, create_external_transfer_msg, create_spot_market_order_msg, default_subaccount_id, DerivativeOrder, InjectiveMsg, InjectiveMsgWrapper, OrderData, OrderInfo, SpotMarketOrder, SpotOrder, MsgCreateSpotMarketOrderResponse};
use injective_math::FPDecimal;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::proto_parser::parse_protobuf_string;
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atomic-order-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TEST_CONTRACT_ADDR: &str = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";
pub const ATOMIC_ORDER_REPLY_ID: u64 = 1u64;
pub const DEPOSIT_REPLY_ID: u64 = 2u64;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let state = State {
        market_id: msg.market_id,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::SwapSpot { quantity, price } => {
            try_swap(deps, _env, info, quantity, price)
        }
        // ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_swap(deps: DepsMut, env: Env, info: MessageInfo, quantity: FPDecimal, price: FPDecimal) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    deps.api.debug(format!("SC ----> Funds info: {}", info.funds[0]).as_str());
    let state = STATE.load(deps.storage)?;
    let contract = env.contract.address;
    let subaccount_id = default_subaccount_id(&contract);
    let min_deposit = price * quantity;
    let message_deposit = FPDecimal::from(info.funds[0].amount.u128());
    if message_deposit < min_deposit { // TODO scaling? check denom?
        return Err(ContractError::CustomError { val: format!("Deposit: {} below min_deposit: {}", message_deposit, min_deposit) });
    }
    let order = SpotOrder::new(price, quantity, true, false, true, &state.market_id, &subaccount_id, contract.as_str());

    let coins = info.funds[0].clone();
    let deposit_message = SubMsg::reply_on_error(create_deposit_msg(contract.to_string(), subaccount_id, coins), DEPOSIT_REPLY_ID);
    let order_message = SubMsg::reply_on_success(create_spot_market_order_msg(contract.into_string(), order), ATOMIC_ORDER_REPLY_ID);
    let mut_response = Response::new().add_submessage(deposit_message).add_submessage(order_message);
    return Ok(mut_response);
}

// pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         state.count += 1;
//         Ok(state)
//     })?;
//
//     Ok(Response::new().add_attribute("method", "try_increment"))
// }

// pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         if info.sender != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(Response::new().add_attribute("method", "reset"))
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    deps.api.debug(format!("SC ----> Received reply (data): {}", msg.clone().result.unwrap().data.unwrap()).as_str());
    let events = msg.clone().result.unwrap().events;
    deps.api.debug(format!("SC ----> Received reply (events count): {}", events.len()).as_str());
    match msg.id {
        ATOMIC_ORDER_REPLY_ID => handle_atomic_order_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_atomic_order_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // let res = parse_reply_execute_data(msg).unwrap();
    // let mut data = res.data.unwrap().to_vec();
    // let rep = msg.result.into_result().map_err(StdError::generic_err);
    // let mut data = res.unwrap().to_vec();

    let mut data = msg.clone().result.into_result().unwrap().data.unwrap().to_vec();
    let field1 = parse_protobuf_string(&mut data,1);

    let field2 = parse_protobuf_string(&mut data,2);
    let field3 = parse_protobuf_string(&mut data,3);
    let field4 = parse_protobuf_string(&mut data,4);

    // let des = from_binary::<MsgCreateSpotMarketOrderResponse>(&data);
    // MsgCreateSpotMarketOrderResponse::
    // parse_protobuf_string(data.0.as_mut(), 1);

    Ok(Response::new())

    // TODO: parse response
    // TODO: find a way to obtain original message data (deposit, sender addr )
    // TODO: return response with transfer messages to transfer back newly obtained coins + leftover deposit
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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, from_binary, SubMsgResponse, SubMsgResult};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use injective_cosmwasm::{Deposit, InjectiveRoute};
    use injective_cosmwasm::InjectiveMsg::CreateSpotMarketOrder;

    use crate::helpers::{get_message_data, i32_to_dec, inj_mock_env};

    use super::*;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { market_id: "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6".to_string() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // reply(deps, )
        // // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn test_swap() {
        let mut deps = mock_dependencies();

        let contract_addr = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";
        let market_id = "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6".to_string();
        let env = inj_mock_env();
        let msg = InstantiateMsg { market_id: market_id.clone() };
        let inst_info = mock_info("creator", &coins(1, "usdt"));
        let _res = instantiate(deps.as_mut(), env.clone(), inst_info, msg).unwrap();

        let swap_sender = "anyone";
        let info = mock_info(swap_sender, &coins(1000, "usdt"));
        let msg = ExecuteMsg::SwapSpot { quantity: i32_to_dec(2), price: i32_to_dec(490) };
        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let expected_atomic_order_message = CreateSpotMarketOrder {
            sender: env.contract.address.into_string(),
            order: SpotOrder {
                market_id,
                order_info: OrderInfo {
                    subaccount_id: "0xade4a5f5803a439835c636395a8d648dee57b2fc000000000000000000000000".to_string(),
                    fee_recipient: contract_addr.to_string(),
                    price: i32_to_dec(490),
                    quantity: i32_to_dec(2),
                },
                order_type: 9,
                trigger_price: None,
            },
        };

        match &get_message_data(&res.messages, 0).msg_data {
            InjectiveMsg::Deposit { sender, subaccount_id, amount } => {
                assert_eq!(sender, contract_addr, "sender not correct")
            }
            _ => {}
        }
        let order_message = get_message_data(&res.messages, 1);
        assert_eq!(InjectiveRoute::Exchange, order_message.route, "route was incorrect");
        assert_eq!(expected_atomic_order_message, order_message.msg_data, "spot create order had incorrect content");

        let binary_response = Binary::from_base64("CkIweGRkNzI5MmY2ODcwMzIwOTc2YTUxYTUwODBiMGQ2NDU5M2NhZjE3OWViM2YxOTNjZWVlZGFiNGVhNWUxNDljZWISQwoTODAwMDAwMDAwMDAwMDAwMDAwMBIWMTAwMDAwMDAwMDAwMDAwMDAwMDAwMBoUMzYwMDAwMDAwMDAwMDAwMDAwMDA=").unwrap();
        let reply_msg = Reply { id: ATOMIC_ORDER_REPLY_ID, result: SubMsgResult::Ok(SubMsgResponse { events: vec![], data: Some(binary_response) } ) };

        let transfers_response = reply(deps.as_mut(), inj_mock_env(), reply_msg);

    }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();
    //
    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    //
    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }
    //
    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
    //
    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
