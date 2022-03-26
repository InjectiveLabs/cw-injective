use crate::errors::ContractError;
use crate::state::{read_config, store_config, Config};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use ninja_protocol::collector::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg,
};
use injective_cosmwasm::InjectiveMsgWrapper;

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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            distribution_contract,
            ninja_token,
        } => update_config(
            deps,
            info,
            owner,
            distribution_contract,
            ninja_token,
        ),
        ExecuteMsg::Distribute {} => distribute(deps, env),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_config(
    deps: DepsMut,
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

// Anyone can execute send function to receive staking token rewards
pub fn distribute(deps: DepsMut, _env: Env) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let _config: Config = read_config(deps.storage)?;
    // TODO:
    Ok(Response::default())
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
