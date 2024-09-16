use crate::{
    error::ContractError,
    handle::{handle_test_market_spot_order, handle_test_transient_derivative_order, handle_test_transient_spot_order},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{handle_query_bank_params, handle_query_spot_market, handle_query_stargate_raw},
    reply::{handle_create_derivative_order_reply_stargate, handle_create_order_reply_stargate},
};

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;
use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQueryWrapper};

const CONTRACT_NAME: &str = "crates.io:injective:dummy-stargate-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CREATE_SPOT_ORDER_REPLY_ID: u64 = 0u64;
pub const CREATE_DERIVATIVE_ORDER_REPLY_ID: u64 = 1u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
        ExecuteMsg::TestTraderTransientSpotOrders {
            market_id,
            subaccount_id,
            price,
            quantity,
        } => handle_test_transient_spot_order(deps, env, &info, market_id, subaccount_id, price, quantity),
        ExecuteMsg::TestMarketOrderStargate {
            market_id,
            subaccount_id,
            price,
            quantity,
        } => handle_test_market_spot_order(deps, env.contract.address.as_ref(), market_id, subaccount_id, price, quantity),
        ExecuteMsg::TestTraderTransientDerivativeOrders {
            market_id,
            subaccount_id,
            price,
            quantity,
            margin,
        } => handle_test_transient_derivative_order(deps, env, &info, market_id, subaccount_id, price, quantity, margin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryStargateRaw { path, query_request } => handle_query_stargate_raw(&deps.querier, path, query_request),
        QueryMsg::QueryBankParams {} => handle_query_bank_params(deps),
        QueryMsg::QuerySpotMarket { market_id } => handle_query_spot_market(deps, &market_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<InjectiveQueryWrapper>, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        CREATE_SPOT_ORDER_REPLY_ID => handle_create_order_reply_stargate(deps, &msg),
        CREATE_DERIVATIVE_ORDER_REPLY_ID => handle_create_derivative_order_reply_stargate(deps, &msg),
        _ => Err(ContractError::UnrecognizedReply(msg.id)),
    }
}
