#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, ContractInfoResponse, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdResult, WasmQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;

use crate::error::ContractError;
use crate::msg::{
    ContractExecutionParams, ContractResponse, ContractsResponse, ExecuteMsg, InstantiateMsg,
    QueryMsg,
};
use crate::state::{CONTRACT, CONTRACTS};

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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {
            contract_address,
            gas_limit,
            gas_price,
            is_executable,
        } => try_register(deps, contract_address, gas_limit, gas_price, is_executable),
        ExecuteMsg::Update {
            contract_address,
            gas_limit,
            gas_price,
        } => try_update(deps, contract_address, gas_limit, gas_price),
        ExecuteMsg::Activate { contract_address } => try_activate(deps, contract_address),
        ExecuteMsg::Deactivate { contract_address } => try_deactivate(deps, contract_address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    only_owner(&env.contract.address, &deps, info)?; // we keep a path for contract owner to update it
    match msg {
        ExecuteMsg::Register { .. } => Err(ContractError::Unauthorized {}),
        ExecuteMsg::Update {
            contract_address,
            gas_limit,
            gas_price,
        } => try_update(deps, contract_address, gas_limit, gas_price),
        ExecuteMsg::Activate { contract_address } => try_activate(deps, contract_address),
        ExecuteMsg::Deactivate { contract_address } => try_deactivate(deps, contract_address),
    }
}

pub fn only_registry(env: Env, info: MessageInfo) -> Result<(), ContractError> {
    // Check if the sender is the registry contract address (only wasmx module can do this)
    if env.contract.address != info.sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}

pub fn only_owner(
    contract_address: &Addr,
    deps: &DepsMut,
    info: MessageInfo,
) -> Result<(), ContractError> {
    // Query contract info
    let query = WasmQuery::ContractInfo {
        contract_addr: contract_address.to_string(),
    };
    let res: ContractInfoResponse = deps.querier.query(&query.into())?;

    // Check if the sender is the owner of the contract
    if res.creator != info.sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}

pub fn try_register(
    deps: DepsMut,
    contract_addr: Addr,
    gas_limit: u64,
    gas_price: u64,
    is_executable: bool,
) -> Result<Response, ContractError> {
    let contract = CONTRACT {
        gas_limit,
        gas_price,
        is_executable,
    };

    // try to store it, fail if the address is already registered
    CONTRACTS.update(deps.storage, &contract_addr, |existing| match existing {
        None => Ok(contract),
        Some(_) => Err(ContractError::AlreadyRegistered {}),
    })?;

    let res = Response::new().add_attributes(vec![
        ("action", "register"),
        ("addr", contract_addr.as_str()),
    ]);
    Ok(res)
}

pub fn try_update(
    deps: DepsMut,
    contract_addr: Addr,
    gas_limit: u64,
    gas_price: u64,
) -> Result<Response, ContractError> {
    // this fails if contract is not available
    let mut contract = CONTRACTS.load(deps.storage, &contract_addr)?;

    // update the contract
    if gas_limit != 0 {
        contract.gas_limit = gas_limit;
    }
    if gas_price != 0 {
        contract.gas_price = gas_price;
    }

    // and save
    CONTRACTS.save(deps.storage, &contract_addr, &contract)?;

    let res = Response::new()
        .add_attributes(vec![("action", "update"), ("addr", contract_addr.as_str())]);
    Ok(res)
}

pub fn try_activate(deps: DepsMut, contract_addr: Addr) -> Result<Response, ContractError> {
    // this fails if contract is not available
    let mut contract = CONTRACTS.load(deps.storage, &contract_addr)?;

    // update the contract to be executable
    contract.is_executable = true;

    // and save
    CONTRACTS.save(deps.storage, &contract_addr, &contract)?;

    let res = Response::new().add_attributes(vec![
        ("action", "activate"),
        ("addr", contract_addr.as_str()),
    ]);
    Ok(res)
}

pub fn try_deactivate(deps: DepsMut, contract_addr: Addr) -> Result<Response, ContractError> {
    // this fails if contract is not available
    let mut contract = CONTRACTS.load(deps.storage, &contract_addr)?;

    contract.is_executable = false;

    // and save
    CONTRACTS.save(deps.storage, &contract_addr, &contract)?;

    let res = Response::new().add_attributes(vec![
        ("action", "deactivate"),
        ("addr", contract_addr.as_str()),
    ]);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContract { contract_address } => {
            to_binary(&query_contract(deps, contract_address)?)
        }
        QueryMsg::GetContracts { start_after, limit } => {
            to_binary(&query_contracts(deps, start_after, limit)?)
        }
        QueryMsg::GetActiveContracts { start_after, limit } => {
            to_binary(&query_active_contracts(deps, start_after, limit)?)
        }
    }
}

pub fn query_contract(deps: Deps, contract_address: Addr) -> StdResult<ContractResponse> {
    let contract = CONTRACTS
        .may_load(deps.storage, &contract_address)?
        .unwrap();

    let contract_info = ContractExecutionParams {
        address: contract_address,
        gas_limit: contract.gas_limit,
        gas_price: contract.gas_price,
        is_executable: contract.is_executable,
    };

    Ok(ContractResponse {
        contract: contract_info,
    })
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

fn query_contracts(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ContractsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let addr = maybe_addr(deps.api, start_after)?;
    let start = addr.as_ref().map(Bound::exclusive);
    // iterate over them all
    let contracts = CONTRACTS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            item.map(|(addr, contract)| ContractExecutionParams {
                address: addr,
                gas_limit: contract.gas_limit,
                gas_price: contract.gas_price,
                is_executable: contract.is_executable,
            })
        })
        .collect::<StdResult<_>>()?;
    Ok(ContractsResponse { contracts })
}

fn query_active_contracts(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ContractsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let addr = maybe_addr(deps.api, start_after)?;
    let start = addr.as_ref().map(Bound::exclusive);
    // iterate over all and return only executable contracts
    let contracts = CONTRACTS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .filter(|item| {
            if let Ok((_, contract)) = item {
                contract.is_executable
            } else {
                false
            }
        })
        .map(|item| {
            item.map(|(addr, contract)| ContractExecutionParams {
                address: addr,
                gas_limit: contract.gas_limit,
                gas_price: contract.gas_price,
                is_executable: contract.is_executable,
            })
        })
        .collect::<StdResult<_>>()?;
    Ok(ContractsResponse { contracts })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use cw2::{get_contract_version, ContractVersion};

    use super::*;

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let loaded = get_contract_version(&deps.storage).unwrap();
        let expected = ContractVersion {
            contract: CONTRACT_NAME.to_string(),
            version: CONTRACT_VERSION.to_string(),
        };
        assert_eq!(expected, loaded);
    }

    #[test]
    fn register() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Only Registry contract can register other contracts
        let registry_addr = mock_env().contract.address;
        let _info = mock_info(registry_addr.as_ref(), &coins(2, "token"));
        let market_maker1: Addr = Addr::unchecked("market_maker1".to_string());
        let msg = ExecuteMsg::Register {
            contract_address: market_maker1.clone(),
            gas_limit: 100,
            gas_price: 10000000,
            is_executable: true,
        };

        let _res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

        // Query by contract address
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContract {
                contract_address: market_maker1.clone(),
            },
        )
        .unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();
        assert_eq!(market_maker1, registered_contract.contract.address);

        // Query all registered contracts
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContracts {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let registered_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, registered_contracts.contracts.len());
    }

    #[ignore]
    #[test]
    fn activation() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Only Registry contract can register other contracts
        let registry_addr = mock_env().contract.address;
        let _info = mock_info(registry_addr.as_ref(), &coins(2, "token"));
        let market_maker: Addr = Addr::unchecked("market_maker1".to_string());
        let msg = ExecuteMsg::Register {
            contract_address: market_maker.clone(),
            gas_limit: 100,
            gas_price: 10000000,
            is_executable: true,
        };

        let _res = sudo(deps.as_mut(), mock_env(), msg).unwrap();

        // Query by contract address
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContract {
                contract_address: market_maker.clone(),
            },
        )
        .unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();
        assert_eq!(market_maker, registered_contract.contract.address);
        assert!(registered_contract.contract.is_executable);

        // Query all registered contracts
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContracts {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let registered_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, registered_contracts.contracts.len());

        // Query all active contracts
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetActiveContracts {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let active_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, active_contracts.contracts.len());

        // Deactivate contract
        let msg = ExecuteMsg::Deactivate {
            contract_address: market_maker.clone(),
        };
        let info = mock_info(market_maker.as_ref(), &coins(2, "token"));
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContract {
                contract_address: market_maker.clone(),
            },
        )
        .unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();
        assert_eq!(market_maker, registered_contract.contract.address);
        assert!(!registered_contract.contract.is_executable);

        // Query all active contracts
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetActiveContracts {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let active_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(0, active_contracts.contracts.len());

        // Activate contract
        let msg = ExecuteMsg::Activate {
            contract_address: market_maker.clone(),
        };
        let info = mock_info(market_maker.as_ref(), &coins(2, "token"));
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContract {
                contract_address: market_maker.clone(),
            },
        )
        .unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();
        assert_eq!(market_maker, registered_contract.contract.address);
        assert!(registered_contract.contract.is_executable);

        // Query all active contracts
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetActiveContracts {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let active_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, active_contracts.contracts.len());
    }

    #[ignore]
    #[test]
    fn update() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Only Registry contract can register other contracts
        let registry_addr = mock_env().contract.address;
        let info = mock_info(registry_addr.as_ref(), &coins(2, "token"));
        let market_maker: Addr = Addr::unchecked("market_maker1".to_string());
        let msg = ExecuteMsg::Register {
            contract_address: market_maker.clone(),
            gas_limit: 100,
            gas_price: 10000000,
            is_executable: true,
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContract {
                contract_address: market_maker.clone(),
            },
        )
        .unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();
        assert_eq!(market_maker, registered_contract.contract.address);
        assert!(registered_contract.contract.is_executable);

        // Mock querier to use
        // Update contract
        let msg = ExecuteMsg::Update {
            contract_address: market_maker.clone(),
            gas_limit: 200,
            gas_price: 15000000,
        };
        let info = mock_info(market_maker.as_ref(), &coins(2, "token"));
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query contract info & validate
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetContract {
                contract_address: market_maker.clone(),
            },
        )
        .unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();
        assert_eq!(market_maker, registered_contract.contract.address);
        assert_eq!(200, registered_contract.contract.gas_limit);
        assert_eq!(15000000, registered_contract.contract.gas_price);
    }
}
