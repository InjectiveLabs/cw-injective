#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQueryWrapper};

use crate::error::ContractError;
use crate::mock_pyth_attestation::execute_trigger_pyth_update;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const COUNTER: Item<u32> = Item::new("counter");
pub const ACTIVE: Item<bool> = Item::new("registered");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    COUNTER.save(deps.storage, &0u32)?;
    ACTIVE.save(deps.storage, &false)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError>  {
    match msg {
        ExecuteMsg::Ping { .. } => {
            let mut response = Response::new();
            response.data = Some(to_binary("pong")?);
            Ok(response)
        }
        ExecuteMsg::Error { .. } => Err(ContractError::Std(StdError::generic_err("oh no!"))),
        ExecuteMsg::TriggerPythUpdate { price } => execute_trigger_pyth_update(deps, env, price),
    }
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BeginBlocker {} => {
            let runs = COUNTER.load(deps.storage)? + 1;
            COUNTER.save(deps.storage, &runs)?;
            ACTIVE.save(deps.storage, &true)?;
            Ok(Response::new())
        }
        SudoMsg::Deregister {} | SudoMsg::Deactivate {} => {
            ACTIVE.save(deps.storage, &false)?;
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ping { .. } => to_binary("pong"),
        QueryMsg::Error { .. } => Err(StdError::generic_err("oh no!")),
        QueryMsg::Runs {} => {
            let runs_count = COUNTER.load(deps.storage)?;
            to_binary(&format!("{runs_count}"))
        }
        QueryMsg::Active {} => {
            let is_active = ACTIVE.load(deps.storage)?;
            to_binary(&format!("{is_active}"))
        }
    }
}
