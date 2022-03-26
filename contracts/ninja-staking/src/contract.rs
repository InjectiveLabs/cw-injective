use crate::staking::{
    unbond,
};
use crate::state::{
    read_config, read_pool_info, store_config, store_pool_info, Config, MigrationParams, PoolInfo,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
};
use ninja_protocol::staking::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg,
};

use cw20::Cw20ReceiveMsg;

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
            ninja_token: deps.api.addr_canonicalize(&msg.ninja_token)?,
            mint_contract: deps.api.addr_canonicalize(&msg.mint_contract)?,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, info, msg),
        ExecuteMsg::UpdateConfig {
            owner,
        } => {
            let owner_addr = if let Some(owner_addr) = owner {
                Some(deps.api.addr_validate(&owner_addr)?)
            } else {
                None
            };
            update_config(
                deps,
                info,
                owner_addr,
            )
        }
        ExecuteMsg::RegisterAsset {
            asset_token,
            staking_token,
        } => {
            let api = deps.api;
            register_asset(
                deps,
                info,
                api.addr_validate(&asset_token)?,
                api.addr_validate(&staking_token)?,
            )
        }
        ExecuteMsg::DeprecateStakingToken {
            asset_token,
            new_staking_token,
        } => {
            let api = deps.api;
            deprecate_staking_token(
                deps,
                info,
                api.addr_validate(&asset_token)?,
                api.addr_validate(&new_staking_token)?,
            )
        }
        ExecuteMsg::Unbond {
            asset_token,
            amount,
        } => {
            let api = deps.api;
            unbond(deps, info.sender, api.addr_validate(&asset_token)?, amount)
        }
        ExecuteMsg::Withdraw { asset_token: _ } => {
            Ok(Response::new())
        }
    }
}

pub fn receive_cw20(
    _deps: DepsMut,
    _info: MessageInfo,
    _cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<Addr>,
) -> StdResult<Response> {
    let mut config: Config = read_config(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
        return Err(StdError::generic_err("unauthorized"));
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }

    store_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![attr("action", "update_config")]))
}

fn register_asset(
    deps: DepsMut,
    info: MessageInfo,
    asset_token: Addr,
    staking_token: Addr,
) -> StdResult<Response> {
    let config: Config = read_config(deps.storage)?;
    let asset_token_raw = deps.api.addr_canonicalize(asset_token.as_str())?;

    if config.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(StdError::generic_err("unauthorized"));
    }

    if read_pool_info(deps.storage, &asset_token_raw).is_ok() {
        return Err(StdError::generic_err("Asset was already registered"));
    }

    store_pool_info(
        deps.storage,
        &asset_token_raw,
        &PoolInfo {
            staking_token: deps.api.addr_canonicalize(staking_token.as_str())?,
            total_bond_amount: Uint128::zero(),
            total_short_amount: Uint128::zero(),
            reward_index: Decimal::zero(),
            short_reward_index: Decimal::zero(),
            pending_reward: Uint128::zero(),
            short_pending_reward: Uint128::zero(),
            premium_rate: Decimal::zero(),
            short_reward_weight: Decimal::zero(),
            premium_updated_time: 0,
            migration_params: None,
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "register_asset"),
        attr("asset_token", asset_token.as_str()),
    ]))
}

fn deprecate_staking_token(
    deps: DepsMut,
    info: MessageInfo,
    asset_token: Addr,
    new_staking_token: Addr,
) -> StdResult<Response> {
    let config: Config = read_config(deps.storage)?;
    let asset_token_raw = deps.api.addr_canonicalize(asset_token.as_str())?;

    if config.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut pool_info: PoolInfo = read_pool_info(deps.storage, &asset_token_raw)?;

    if pool_info.migration_params.is_some() {
        return Err(StdError::generic_err(
            "This asset LP token has already been migrated",
        ));
    }

    let deprecated_token_addr: Addr = deps.api.addr_humanize(&pool_info.staking_token)?;

    pool_info.total_bond_amount = Uint128::zero();
    pool_info.migration_params = Some(MigrationParams {
        index_snapshot: pool_info.reward_index,
        deprecated_staking_token: pool_info.staking_token,
    });
    pool_info.staking_token = deps.api.addr_canonicalize(new_staking_token.as_str())?;

    store_pool_info(deps.storage, &asset_token_raw, &pool_info)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "depcrecate_staking_token"),
        attr("asset_token", asset_token.to_string()),
        attr(
            "deprecated_staking_token",
            deprecated_token_addr.to_string(),
        ),
        attr("new_staking_token", new_staking_token.to_string()),
    ]))
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
        ninja_token: deps.api.addr_humanize(&state.ninja_token)?.to_string(),
        mint_contract: deps.api.addr_humanize(&state.mint_contract)?.to_string(),
    };

    Ok(resp)
}
