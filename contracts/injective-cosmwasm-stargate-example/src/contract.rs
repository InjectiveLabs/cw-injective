use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::handle_query_stargate,
};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use injective_cosmwasm::{create_deposit_msg, InjectiveMsgWrapper, InjectiveQueryWrapper};

const CONTRACT_NAME: &str = "crates.io:injective:dummy-stargate-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CREATE_SPOT_ORDER_REPLY_ID: u64 = 0u64;
pub const CREATE_DERIVATIVE_ORDER_REPLY_ID: u64 = 1u64;
pub const MSG_EXEC: &str = "/cosmos.authz.v1beta1.MsgExec";

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
    match msg {
        QueryMsg::QueryStargate { path, query_request } => handle_query_stargate(&deps.querier, path, query_request),
    }
}
