use std::str::FromStr;

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult, SubMsg, to_binary, WasmMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::set_contract_version;
use cw_storage_plus::Item;

use injective_cosmwasm::{address_to_subaccount_id, create_batch_update_orders_msg, create_deposit_msg, create_external_transfer_msg, create_spot_market_order_msg, default_subaccount_id, DerivativeOrder, InjectiveMsg, InjectiveMsgWrapper, OrderData, OrderInfo, SpotMarketOrder, SpotOrder};
use injective_math::FPDecimal;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atomic-order-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ATOMIC_ORDER_REPLY_ID: u64 = 1u64;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
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
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Swap { quantity, price } => {
            try_swap(deps, _env, info, FPDecimal::from_str(quantity.as_str())?, FPDecimal::from_str(price.as_str())?)
        }
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_swap(deps: DepsMut, env: Env, info: MessageInfo, quantity: FPDecimal, price: FPDecimal) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let contract = env.contract.address;
    let subaccount_id = default_subaccount_id(&contract);
    let min_deposit = price * quantity;
    if min_deposit < FPDecimal::from(info.funds[0].amount) { // TODO scaling? check denom?
        return Err(ContractError::CustomError { val: "Not enough deposit".to_string() });
    }
    let order = SpotOrder::new(price, quantity, true, false, true, &state.market_id, &subaccount_id, contract.as_str());

    let submessage = SubMsg::reply_on_success(create_spot_market_order_msg(contract.into_string(), order), ATOMIC_ORDER_REPLY_ID);
    let mut_response = Response::new().add_submessage(submessage);
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
    match msg.id {
        ATOMIC_ORDER_REPLY_ID => handle_atomic_order_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_atomic_order_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let binary_response = msg.result.into_result()?.data?;
    // TODO: parse response
    // TODO: find a way to obtain original message data (deposit, sender addr )
    // TODO: return response with transfer messages to transfer back newly obtained coins + leftover deposit
}



// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
//     }
// }
//
// fn query_count(deps: Deps) -> StdResult<GetCountResponse> {
//     let state = STATE.load(deps.storage)?;
//     Ok(GetCountResponse { count: state.count })
// }

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, from_binary};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { market_id: "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6".to_string() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies();
    //
    //     let msg = InstantiateMsg { market_id: "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6".to_string()};
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    //
    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    //
    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

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
