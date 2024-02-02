use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use injective_cosmwasm::{create_deposit_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::TestDepositMsg { subaccount_id, amount } => {
            Ok(Response::new().add_message(create_deposit_msg(env.contract.address, subaccount_id, amount)))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let querier: InjectiveQuerier = InjectiveQuerier::new(&deps.querier);

    match msg {
        QueryMsg::TestSpotMarketQuery { market_id } => to_json_binary(&querier.query_spot_market(&market_id)?),
        QueryMsg::TestExchangeParamsQuery {} => to_json_binary(&querier.query_exchange_params()?),
        QueryMsg::TestSubAccountDepositQuery { subaccount_id, denom } => to_json_binary(&querier.query_subaccount_deposit(&subaccount_id, &denom)?),
    }
}
