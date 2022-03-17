#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ContractsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, CONTRACT, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        contracts: Vec::new(),
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {
            contract_address,
            gas_limit,
            gas_price,
            is_executable,
        } => try_register(
            deps,
            env,
            info,
            contract_address,
            gas_limit,
            gas_price,
            is_executable,
        ),
    }
}

pub fn only_owner(sender: &Addr, owner: &Addr) {
    assert_eq!(sender, owner);
}

pub fn try_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract_addr: Addr,
    gas_limit: u64,
    gas_price: String,
    is_executable: bool,
) -> Result<Response, ContractError> {
    // Ensure that only wasmx module calls this method
    only_owner(&env.contract.address, &info.sender);

    let mut state = STATE.load(deps.storage)?;
    let contract = CONTRACT {
        address: contract_addr,
        gas_limit: gas_limit,
        gas_price: gas_price,
        is_executable: is_executable,
    };

    state.contracts.push(contract);
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "register"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContracts {} => to_binary(&query_contracts(deps)?),
        QueryMsg::GetActiveContracts {} => to_binary(&query_active_contracts(deps)?),
    }
}

fn query_contracts(deps: Deps) -> StdResult<ContractsResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(ContractsResponse {
        contracts: state.contracts,
    })
}

fn query_active_contracts(deps: Deps) -> StdResult<ContractsResponse> {
    let state = STATE.load(deps.storage)?;
    let active_contracts: Vec<CONTRACT>;
    active_contracts = state
        .contracts
        .into_iter()
        .filter(|contract| contract.is_executable)
        .collect();
    Ok(ContractsResponse {
        contracts: active_contracts,
    })
}
