use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Reply, Response,
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
use crate::msg::{ExecuteMsg, FeeRecipient, InstantiateMsg, QueryMsg};
use crate::queries::{estimate_single_swap_execution, estimate_swap_result};
use crate::state::{
    read_swap_route, remove_swap_route, store_swap_route, CONFIG, STEP_STATE, SWAP_OPERATION_STATE,
};
use crate::types::{Config, CurrentSwapOperation, CurrentSwapStep, FPCoin, SwapRoute};

// use injective_protobuf::proto::tx;

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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    save_config(deps, env, msg.admin, msg.fee_recipient)?;
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
            denom_1,
            denom_2,
            route,
        } => set_route(deps, &info.sender, denom_1, denom_2, route),
        ExecuteMsg::DeleteRoute { denom_1, denom_2 } => {
            delete_route(deps, &info.sender, denom_1, denom_2)
        }
        ExecuteMsg::Swap {
            target_denom,
            min_quantity,
        } => start_swap_flow(deps, env, info, target_denom, min_quantity),
        ExecuteMsg::UpdateConfig {
            admin,
            fee_recipient,
        } => update_config(deps, env, info.sender, admin, fee_recipient),
        ExecuteMsg::WithdrawSupportFunds {
            coins,
            target_address,
        } => withdraw_support_funds(deps, info.sender, coins, target_address),
    }
}

pub fn set_route(
    deps: DepsMut<InjectiveQueryWrapper>,
    sender: &Addr,
    denom_1: String,
    denom_2: String,
    route: Vec<MarketId>,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    verify_sender_is_admin(deps.as_ref(), sender)?;
    if denom_1 == denom_2 {
        return Err(ContractError::CustomError {
            val: "Cannot set a route for the same denom!".to_string(),
        });
    }
    let route = SwapRoute {
        steps: route,
        denom_1,
        denom_2,
    };
    store_swap_route(deps.storage, &route)?;
    Ok(Response::new().add_attribute("method", "set_route"))
}

pub fn delete_route(
    deps: DepsMut<InjectiveQueryWrapper>,
    sender: &Addr,
    denom_1: String,
    denom_2: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    verify_sender_is_admin(deps.as_ref(), sender)?;
    remove_swap_route(deps.storage, &denom_1, &denom_2);
    Ok(Response::new().add_attribute("method", "delete_route"))
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
            val: "Only 1 denom can be passed in funds".to_string(),
        });
    }
    if min_target_quantity.is_negative() || min_target_quantity.is_zero() {
        return Err(ContractError::CustomError {
            val: "Min target quantity must be positive!".to_string(),
        });
    }
    let coin_provided = info.funds[0].clone();
    let from_denom = coin_provided.denom;
    let target_denom = target_denom;
    let route = read_swap_route(deps.storage, &from_denom, &target_denom)?;
    deps.api.debug(&format!(
        "Read a route {:?} {:?}: {:?}",
        from_denom, target_denom, route
    ));
    let steps = route.steps_from(&from_denom);

    let current_balance: FPCoin = info.funds.first().unwrap().clone().into();
    let swap_operation = CurrentSwapOperation {
        sender_address: info.sender,
        swap_steps: steps,
        min_target_quantity,
    };
    SWAP_OPERATION_STATE.save(deps.storage, &swap_operation)?;
    execute_swap_step(deps, env, swap_operation, 0, current_balance).map_err(ContractError::Std)
}

fn execute_swap_step(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    swap_operation: CurrentSwapOperation,
    step_idx: u16,
    current_balance: FPCoin,
) -> StdResult<Response<InjectiveMsgWrapper>> {
    let market_id = swap_operation.swap_steps[usize::from(step_idx)].clone();
    let contract = &env.contract.address;
    let subaccount_id = get_default_subaccount_id_for_checked_address(contract);

    let estimation =
        estimate_single_swap_execution(&deps.as_ref(), &env, &market_id, current_balance.clone())?;
    // TODO: add handling of supporting funds

    let fee_recipient = &CONFIG.load(deps.storage)?.fee_recipient;

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
        Some(fee_recipient.to_owned()),
    );

    let order_message = SubMsg::reply_on_success(
        create_spot_market_order_msg(contract.to_owned(), order),
        ATOMIC_ORDER_REPLY_ID,
    );

    let current_step = CurrentSwapStep {
        step_idx,
        current_balance,
        step_target_denom: estimation.result_denom,
        is_buy: estimation.is_buy_order,
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
    deps.api
        .debug(&format!("Order response: {:?}", order_response));

    // unwrap results into trade_data
    let trade_data = match order_response.results.into_option() {
        Some(trade_data) => Ok(trade_data),
        None => Err(ContractError::CustomError {
            val: "No trade data in order response".to_string(),
        }),
    }?;
    let quantity = FPDecimal::from_str(&trade_data.quantity)? / dec_scale_factor;
    let avg_price = FPDecimal::from_str(&trade_data.price)? / dec_scale_factor;
    let fee = FPDecimal::from_str(&trade_data.fee)? / dec_scale_factor;
    deps.api.debug(&format!(
        "Quantity: {quantity}, price {avg_price}, fee {fee}"
    ));

    let current_step = STEP_STATE.load(deps.storage).map_err(ContractError::Std)?;
    let new_quantity = if current_step.is_buy {
        quantity
    } else {
        quantity * avg_price - fee
    };

    let new_balance = FPCoin {
        amount: new_quantity,
        denom: current_step.step_target_denom,
    };

    deps.api.debug(&format!("New balance: {:?}", new_balance));

    let swap = SWAP_OPERATION_STATE.load(deps.storage)?;
    if current_step.step_idx < (swap.swap_steps.len() - 1) as u16 {
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
        deps.api.debug(&format!("Send message: {:?}", send_message));
        SWAP_OPERATION_STATE.remove(deps.storage);
        STEP_STATE.remove(deps.storage);
        let response = Response::new()
            .add_message(send_message)
            .add_attribute("swap_final_amount", new_balance.amount.to_string())
            .add_attribute("swap_final_denom", new_balance.denom);

        Ok(response)
    }
}

fn save_config(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    admin: Addr,
    fee_recipient: FeeRecipient,
) -> StdResult<()> {
    let fee_recipient = match fee_recipient {
        FeeRecipient::Address(addr) => addr,
        FeeRecipient::SwapContract => env.contract.address,
    };
    let config = Config {
        fee_recipient,
        admin,
    };
    CONFIG.save(deps.storage, &config)
}

fn verify_sender_is_admin(
    deps: Deps<InjectiveQueryWrapper>,
    sender: &Addr,
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.admin != sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}

fn update_config(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    admin: Option<Addr>,
    fee_recipient: Option<FeeRecipient>,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    verify_sender_is_admin(deps.as_ref(), &sender)?;
    let mut config = CONFIG.load(deps.storage)?;
    if let Some(admin) = admin {
        config.admin = admin;
    }
    if let Some(fee_recipient) = fee_recipient {
        config.fee_recipient = match fee_recipient {
            FeeRecipient::Address(addr) => addr,
            FeeRecipient::SwapContract => env.contract.address,
        };
    }
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("method", "update_config"))
}

fn withdraw_support_funds(
    deps: DepsMut<InjectiveQueryWrapper>,
    sender: Addr,
    coins: Vec<Coin>,
    target_address: Addr,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    verify_sender_is_admin(deps.as_ref(), &sender)?;
    let send_message = BankMsg::Send {
        to_address: target_address.to_string(),
        amount: coins,
    };
    let response = Response::new()
        .add_message(send_message)
        .add_attribute("method", "withdraw_support_funds")
        .add_attribute("target_address", target_address.to_string());
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InjectiveQueryWrapper>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRoute { denom_1, denom_2 } => Ok(to_binary(&read_swap_route(
            deps.storage,
            &denom_1,
            &denom_2,
        )?)?),
        QueryMsg::GetExecutionQuantity {
            from_quantity,
            from_denom,
            to_denom,
        } => {
            let target_quantity =
                estimate_swap_result(deps, env, from_denom, from_quantity, to_denom)?;
            Ok(to_binary(&target_quantity)?)
        }
    }
}
