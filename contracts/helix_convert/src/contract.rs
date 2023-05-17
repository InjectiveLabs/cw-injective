use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult, SubMsg,
};
use cw2::set_contract_version;
use protobuf::Message;

use injective_cosmwasm::{
    create_spot_market_order_msg, get_default_subaccount_id_for_checked_address,
    InjectiveMsgWrapper, InjectiveQueryWrapper, MarketId, OrderType, SpotOrder,
};
use injective_math::FPDecimal;
use injective_protobuf::proto::tx;

use crate::error::ContractError;
use crate::helpers::dec_scale_factor;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::queries::estimate_single_swap_execution;
use crate::state::{read_swap_route, store_swap_route, STEP_STATE, SWAP_OPERATION_STATE};
use crate::types::{CurrentSwapOperation, CurrentSwapStep, FPCoin, SwapRoute};

// use injective_protobuf::proto::tx;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atomic-order-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ATOMIC_ORDER_REPLY_ID: u64 = 1u64;
pub const DEPOSIT_REPLY_ID: u64 = 2u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::SetRoute {
            denon_1,
            denom_2,
            route,
        } => set_route(deps, denon_1, denom_2, route),
        ExecuteMsg::Swap {
            target_denom,
            min_quantity,
        } => start_swap_flow(deps, env, info, target_denom, min_quantity),
    }
}

pub fn set_route(
    deps: DepsMut<InjectiveQueryWrapper>,
    denom_1: String,
    denom_2: String,
    route: Vec<MarketId>,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // TODO: add checking ownership
    let route = SwapRoute {
        steps: route,
        denom_1,
        denom_2,
    };
    store_swap_route(deps.storage, &route)?;
    Ok(Response::new().add_attribute("method", "set_route"))
}

pub fn start_swap_flow(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    target_denom: String,
    min_target_quantity: FPDecimal,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::CustomError {
            val: "Wrong amount of funds deposited!".to_string(),
        });
    }
    let coin_provided = info.funds[0].clone();
    let from_denom = coin_provided.denom;
    let route = read_swap_route(deps.storage, &from_denom, &target_denom)?;
    let steps = route.steps_from(&from_denom);

    let current_balance: FPCoin = info.funds.first().unwrap().clone().into();
    let swap_operation = CurrentSwapOperation {
        sender_address: info.sender,
        swap_steps: steps,
        min_target_quantity,
    };
    SWAP_OPERATION_STATE.save(deps.storage, &swap_operation)?;
    execute_swap_step(deps, env, swap_operation, 0, current_balance)
        .map_err(ContractError::Std)
}

fn execute_swap_step(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    swap_operation: CurrentSwapOperation,
    step_idx: usize,
    current_balance: FPCoin,
) -> StdResult<Response<InjectiveMsgWrapper>> {
    let market_id = swap_operation.swap_steps[step_idx].clone();
    let contract = env.contract.address;
    let subaccount_id = get_default_subaccount_id_for_checked_address(&contract);

    let estimation =
        estimate_single_swap_execution(&deps.as_ref(), &market_id, current_balance.clone())?;
    // TODO: add handling of supporting funds

    let order = SpotOrder::new(
        estimation.worst_price,
        if estimation.is_buy_order {
            estimation.result_quantity
        } else {
            current_balance.amount
        },
        if estimation.is_buy_order {
            OrderType::BuyAtomic
        } else {
            OrderType::SellAtomic
        },
        &market_id,
        subaccount_id,
        Some(contract.to_owned()),
    );

    let order_message = SubMsg::reply_on_success(
        create_spot_market_order_msg(contract, order),
        ATOMIC_ORDER_REPLY_ID,
    );

    let current_step = CurrentSwapStep {
        step_idx,
        current_balance,
        step_target_denom: estimation.result_denom,
    };
    STEP_STATE.save(deps.storage, &current_step)?;

    let response: Response<InjectiveMsgWrapper> = Response::new().add_submessage(order_message);
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
    // let dec_scale_factor: FPDecimal = FPDecimal::from(1000000000000000000_i128);
    // let order_response: MsgCreateSpotMarketOrderResponse = from_binary(
    //     &msg.result
    //         .into_result()
    //         .map_err(ContractError::SubMsgFailure)?
    //         .data
    //         .ok_or_else(|| ContractError::ReplyParseFailure {
    //             id: ATOMIC_ORDER_REPLY_ID,
    //             err: "Missing reply data".to_owned(),
    //         })?, // .as_slice(),
    // )
    // .map_err(|err| ContractError::ReplyParseFailure {
    //     id: ATOMIC_ORDER_REPLY_ID,
    //     err: err.to_string(),
    // })?;
    // // unwrap results into trade_data
    // let trade_data = order_response.results;
    // let quantity = trade_data.quantity / dec_scale_factor;
    // // let price = trade_data.price / dec_scale_factor;
    // // let fee = trade_data.fee / dec_scale_factor;
    let dec_scale_factor = dec_scale_factor();
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
    println!("Quantity: {}", quantity);

    let current_step = STEP_STATE
        .load(deps.storage)
        .map_err(ContractError::Std)?;
    let new_balance = FPCoin {
        amount: quantity,
        denom: current_step.step_target_denom,
    };

    let swap = SWAP_OPERATION_STATE.load(deps.storage)?;
    if current_step.step_idx < swap.swap_steps.len() - 1 {
        execute_swap_step(deps, env, swap, current_step.step_idx + 1, new_balance)
            .map_err(ContractError::Std)
    } else {
        // last step, finalise and send back funds to a caller
        if new_balance.amount < swap.min_target_quantity {
            return Err(ContractError::MinExpectedSwapAmountNotReached(
                swap.min_target_quantity,
            ));
        }
        let send_message = BankMsg::Send {
            to_address: swap.sender_address.to_string(),
            amount: vec![new_balance.clone().into()],
        };
        SWAP_OPERATION_STATE.remove(deps.storage);
        STEP_STATE.remove(deps.storage);
        let response = Response::new()
            .add_message(send_message)
            .add_attribute("swap_final_amount", new_balance.amount.to_string())
            .add_attribute("swap_final_denom", new_balance.denom);

        Ok(response)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
