use crate::errors::ContractError;
use crate::state::{read_config, store_config, Config, SUBACCOUNT_ID_TO_SUBCONTRACT};
use cosmwasm_std::CosmosMsg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128, WasmMsg,
};
use ethereum_types::H160;
use injective_cosmwasm::{
    InjectiveMsg, InjectiveMsgWrapper, InjectiveQueryWrapper, InjectiveRoute,
};
use ninja_protocol::master::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use subtle_encoding::bech32;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    store_config(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(&msg.owner)?,
            distribution_contract: deps.api.addr_canonicalize(&msg.distribution_contract)?,
            ninja_token: deps.api.addr_canonicalize(&msg.ninja_token)?,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::MintToUser {
            subaccount_id_sender,
            pool_subaccount_id,
            amount,
        } => mint_to_user(
            deps,
            env,
            info.sender,
            subaccount_id_sender,
            pool_subaccount_id,
            amount,
        ),
        ExecuteMsg::BurnFromUser {
            subaccount_id_sender,
            pool_subaccount_id,
            amount,
        } => burn_from_user(
            deps,
            env,
            info.sender,
            subaccount_id_sender,
            pool_subaccount_id,
            amount,
        ),
        ExecuteMsg::UpdateConfig {
            owner,
            distribution_contract,
            ninja_token,
        } => update_config(deps, info, owner, distribution_contract, ninja_token),
        ExecuteMsg::ExecuteOrders { injective_messages } => {
            execute_orders(deps, env, info.sender, injective_messages)
        }
    }
}

pub fn decode_bech32(addr: &String) -> String {
    let zeros_to_append = "000000000000000000000000";
    let decoded_bytes = bech32::decode(addr.as_str()).unwrap().1;
    let decoded_h160 = H160::from_slice(&decoded_bytes);
    let decoded_string = format!("{:?}{}", decoded_h160, zeros_to_append);
    decoded_string
}

pub fn execute_orders(
    _deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    _sender: Addr,
    msgs: Vec<InjectiveMsg>,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // TODO only_owner(&any, &sender);

    let mut new_messages: Vec<CosmosMsg<InjectiveMsgWrapper>> = Vec::new();

    msgs.into_iter().for_each(|msg| match msg {
        InjectiveMsg::BatchUpdateOrders {
            sender,
            subaccount_id,
            spot_market_ids_to_cancel_all,
            derivative_market_ids_to_cancel_all,
            spot_orders_to_cancel,
            derivative_orders_to_cancel,
            spot_orders_to_create,
            derivative_orders_to_create,
        } => new_messages.push(
            InjectiveMsgWrapper {
                route: InjectiveRoute::Exchange,
                msg_data: InjectiveMsg::BatchUpdateOrders {
                    sender: sender.to_string(),
                    subaccount_id: subaccount_id.to_string(),
                    spot_market_ids_to_cancel_all,
                    derivative_market_ids_to_cancel_all,
                    spot_orders_to_cancel,
                    derivative_orders_to_cancel,
                    spot_orders_to_create,
                    derivative_orders_to_create,
                },
            }
            .into(),
        ),
    });

    Ok(Response::new().add_messages(new_messages)) // TODO .add_message(message))
}

pub fn mint_to_user(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    subaccount_id_sender: String,
    pool_subaccount_id: String,
    amount: Uint128,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // Ensure that only exchange module calls this method
    only_owner(&env.contract.address, &sender);

    let contract_address = get_contract_from_subaccount_id(deps, pool_subaccount_id);
    let subcontract_subaccount_id = decode_bech32(&contract_address);

    let mint = ExecuteMsg::MintToUser {
        subaccount_id_sender,
        pool_subaccount_id: subcontract_subaccount_id,
        amount,
    };
    let message = WasmMsg::Execute {
        contract_addr: contract_address.into(),
        msg: to_binary(&mint)?,
        funds: vec![],
    };

    Ok(Response::new().add_message(message))
}

pub fn burn_from_user(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    subaccount_id_sender: String,
    pool_subaccount_id: String,
    amount: Uint128,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // Ensure that only exchange module calls this method
    only_owner(&env.contract.address, &sender);

    let contract_address = get_contract_from_subaccount_id(deps, pool_subaccount_id);
    let subcontract_subaccount_id = decode_bech32(&contract_address);

    let burn = ExecuteMsg::BurnFromUser {
        subaccount_id_sender,
        pool_subaccount_id: subcontract_subaccount_id,
        amount,
    };
    let message = WasmMsg::Execute {
        contract_addr: contract_address.into(),
        msg: to_binary(&burn)?,
        funds: vec![],
    };

    Ok(Response::new().add_message(message))
}

fn get_contract_from_subaccount_id(
    deps: DepsMut<InjectiveQueryWrapper>,
    subaccount_id: String,
) -> String {
    SUBACCOUNT_ID_TO_SUBCONTRACT
        .load(deps.storage, &subaccount_id)
        .unwrap()
}

#[allow(clippy::too_many_arguments)]
pub fn update_config(
    deps: DepsMut<InjectiveQueryWrapper>,
    info: MessageInfo,
    owner: Option<String>,
    distribution_contract: Option<String>,
    ninja_token: Option<String>,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    if config.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(&owner)?;
    }

    if let Some(distribution_contract) = distribution_contract {
        config.distribution_contract = deps.api.addr_canonicalize(&distribution_contract)?;
    }

    if let Some(ninja_token) = ninja_token {
        config.ninja_token = deps.api.addr_canonicalize(&ninja_token)?;
    }

    store_config(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_config")]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = read_config(deps.storage)?;
    let resp = ConfigResponse {
        owner: deps.api.addr_humanize(&state.owner)?.to_string(),
        distribution_contract: deps
            .api
            .addr_humanize(&state.distribution_contract)?
            .to_string(),
        ninja_token: deps.api.addr_humanize(&state.ninja_token)?.to_string(),
    };

    Ok(resp)
}

pub fn only_owner(sender: &Addr, owner: &Addr) {
    assert_eq!(sender, owner);
}
