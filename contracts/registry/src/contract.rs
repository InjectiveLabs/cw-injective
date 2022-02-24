#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, BankMsg, coins};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ContractsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, CONTRACT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {contract_address, gas_limit} => try_register(deps, info, contract_address, gas_limit),        
    }
}


pub fn try_register(deps: DepsMut, info: MessageInfo, contract_addr: Addr, gas_limit: u64) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    
    // if state.contracts.contains(&contract_addr) {
    //     return Err(ContractError::Unauthorized {
    //         msg: "contract already registered".to_string(),
    //     });
    // }
    let contract = CONTRACT {
        address: contract_addr,
        gas_limit: gas_limit,
    };

    state.contracts.push(contract);
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "register"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContracts {} => to_binary(&query_contracts(deps)?),
    }
}

fn query_contracts(deps: Deps) -> StdResult<ContractsResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(ContractsResponse { contracts: state.contracts })
}
