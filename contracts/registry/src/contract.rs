#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps,Order, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::{set_contract_version, get_contract_version, ContractVersion};

use crate::error::ContractError;
use crate::msg::{ContractInfo, ContractsResponse,ContractResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONTRACTS, CONTRACT};

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {contract_address,gas_limit,gas_price,is_executable,} => try_register(deps,env,info,contract_address,gas_limit,gas_price,is_executable),
        ExecuteMsg::Update {contract_address, gas_limit, gas_price} => try_update(deps,env,info,contract_address, gas_limit, gas_price),
        ExecuteMsg::Activate {contract_address} => try_activate(deps,env,info,contract_address),
        ExecuteMsg::DeActivate {contract_address} => try_deactivate(deps,env,info,contract_address),
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

    let contract = CONTRACT {        
        gas_limit: gas_limit,
        gas_price: gas_price,
        is_executable: is_executable,
    };

    // try to store it, fail if the address is already registered
    CONTRACTS.update(deps.storage, &contract_addr, |existing| match existing {
        None => Ok(contract),
        Some(_) => Err(ContractError::AlreadyRegistered {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "register"), ("addr", contract_addr.as_str())]);
    Ok(res)
}

pub fn try_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract_addr: Addr, 
    gas_limit: u64,
    gas_price: String,
) -> Result<Response, ContractError> {
    
    // this fails if contract is not available
    let mut contract = CONTRACTS.load(deps.storage, &contract_addr)?;

    // update the contract to be executable 
    if gas_limit != 0 {
        contract.gas_limit = gas_limit;
    }
    
    if gas_price != "" {
        contract.gas_price = gas_price;
    }

    // and save
    CONTRACTS.save(deps.storage, &contract_addr, &contract)?;

    let res = Response::new().add_attributes(vec![("action", "activate_contract"), ("addr", contract_addr.as_str())]);
    Ok(res)
}

pub fn try_activate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract_addr: Addr, 
) -> Result<Response, ContractError> {
    
    // this fails if contract is not available
    let mut contract = CONTRACTS.load(deps.storage, &contract_addr)?;

    // update the contract to be executable 
    contract.is_executable = true;

    // and save
    CONTRACTS.save(deps.storage, &contract_addr, &contract)?;

    let res = Response::new().add_attributes(vec![("action", "activate_contract"), ("addr", contract_addr.as_str())]);
    Ok(res)
}

pub fn try_deactivate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    contract_addr: Addr, 
) -> Result<Response, ContractError> {
    
    // this fails if contract is not available
    let mut contract = CONTRACTS.load(deps.storage, &contract_addr)?;

    contract.is_executable = false;

    // and save
    CONTRACTS.save(deps.storage, &contract_addr, &contract)?;

    let res = Response::new().add_attributes(vec![("action", "deactivate_contract"), ("addr", contract_addr.as_str())]);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContract {contract_address} => to_binary(&query_contract(deps, contract_address)?),
        QueryMsg::GetContracts {} => to_binary(&query_contracts(deps)?),
        QueryMsg::GetActiveContracts {} => to_binary(&query_active_contracts(deps)?),
    }
}

pub fn query_contract(deps: Deps, contract_address: Addr) -> StdResult<ContractResponse> {    
    let contract = CONTRACTS
        .may_load(deps.storage, &contract_address)?
        .unwrap();

        let contract_info = ContractInfo {
            address: contract_address,
            gas_limit: contract.gas_limit,
            gas_price: contract.gas_price,
            is_executable: contract.is_executable,
        };

        Ok(ContractResponse {
            contract: contract_info,
        })
}

fn query_contracts(deps: Deps) -> StdResult<ContractsResponse> {    
     // iterate over them all
    let contracts = CONTRACTS    
    .range(deps.storage, None, None, Order::Ascending)    
    .map(|item| {
        item.map(|(addr, contract)| ContractInfo {
            address: addr,
            gas_limit: contract.gas_limit,
            gas_price: contract.gas_price,
            is_executable: contract.is_executable,
        })
    })
    .collect::<StdResult<_>>()?;
Ok(ContractsResponse { contracts })
}

fn query_active_contracts(deps: Deps) -> StdResult<ContractsResponse> {    
      // iterate over all and return only executable contracts
      let contracts = CONTRACTS    
      .range(deps.storage, None, None, Order::Ascending)    
      .filter(|item| {
        if let Ok((_, contract)) = item {
            contract.is_executable
        } else {
            false
        }
    })
      .map(|item| {
          item.map(|(addr, contract)| ContractInfo {
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
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

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
        let info = mock_info(&registry_addr.to_string(), &coins(2, "token"));
        let market_maker1:Addr = Addr::unchecked("market_maker1".to_string());                
        let msg = ExecuteMsg::Register {
            contract_address: market_maker1.clone(),
            gas_limit: 100,
            gas_price: "10000000".to_string(),
            is_executable: true,
        };
        
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContract {contract_address: market_maker1.clone()}).unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();                
        assert_eq!(market_maker1, registered_contract.contract.address);

        // Query all registered contracts
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContracts {}).unwrap();
        let registered_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, registered_contracts.contracts.len());        
    }

    #[test]
    fn activation() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));                
                
        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
                
        // Only Registry contract can register other contracts        
        let registry_addr = mock_env().contract.address;
        let info = mock_info(&registry_addr.to_string(), &coins(2, "token"));
        let market_maker:Addr = Addr::unchecked("market_maker1".to_string());                
        let msg = ExecuteMsg::Register {
            contract_address: market_maker.clone(),
            gas_limit: 100,
            gas_price: "10000000".to_string(),
            is_executable: true,
        };
        
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContract {contract_address: market_maker.clone()}).unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();                
        assert_eq!(market_maker, registered_contract.contract.address);
        assert_eq!(true, registered_contract.contract.is_executable);

        // Query all registered contracts
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContracts {}).unwrap();
        let registered_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, registered_contracts.contracts.len());


        // Query all active contracts
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetActiveContracts{}).unwrap();
        let active_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, active_contracts.contracts.len());
        
        // Deactivate contract
        let msg = ExecuteMsg::DeActivate {
            contract_address: market_maker.clone(),            
        };
        let info = mock_info(&market_maker.to_string(), &coins(2, "token"));                
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContract {contract_address: market_maker.clone()}).unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();                
        assert_eq!(market_maker, registered_contract.contract.address);
        assert_eq!(false, registered_contract.contract.is_executable);

        // Query all active contracts
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetActiveContracts{}).unwrap();
        let active_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(0, active_contracts.contracts.len());

        // Activate contract
        let msg = ExecuteMsg::Activate {
            contract_address: market_maker.clone(),            
        };
        let info = mock_info(&market_maker.to_string(), &coins(2, "token"));                
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContract {contract_address: market_maker.clone()}).unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();                
        assert_eq!(market_maker, registered_contract.contract.address);
        assert_eq!(true, registered_contract.contract.is_executable);

        // Query all active contracts
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetActiveContracts{}).unwrap();
        let active_contracts: ContractsResponse = from_binary(&res).unwrap();
        assert_eq!(1, active_contracts.contracts.len());
    }


    #[test]
    fn update() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));                
                
        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
                
        // Only Registry contract can register other contracts        
        let registry_addr = mock_env().contract.address;
        let info = mock_info(&registry_addr.to_string(), &coins(2, "token"));
        let market_maker:Addr = Addr::unchecked("market_maker1".to_string());                
        let msg = ExecuteMsg::Register {
            contract_address: market_maker.clone(),
            gas_limit: 100,
            gas_price: "10000000".to_string(),
            is_executable: true,
        };
        
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query by contract address
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContract {contract_address: market_maker.clone()}).unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();                
        assert_eq!(market_maker, registered_contract.contract.address);
        assert_eq!(true, registered_contract.contract.is_executable);
        
        // Update contract
        let msg = ExecuteMsg::Update {
            contract_address: market_maker.clone(),          
            gas_limit: 200,
            gas_price: "15000000".to_string(),
        };
        let info = mock_info(&market_maker.to_string(), &coins(2, "token"));                
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Query contractinfo & validate
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContract {contract_address: market_maker.clone()}).unwrap();
        let registered_contract: ContractResponse = from_binary(&res).unwrap();                
        assert_eq!(market_maker, registered_contract.contract.address);
        assert_eq!(200, registered_contract.contract.gas_limit);
        assert_eq!("15000000".to_string(), registered_contract.contract.gas_price);
    }
}
