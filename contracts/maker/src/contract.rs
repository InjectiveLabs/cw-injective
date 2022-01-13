use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Addr,
};
use injective_bindings::{
    SubaccountDepositResponse, InjectiveQueryWrapper, InjectiveQuerier,
    InjectiveMsgWrapper,
    create_subaccount_transfer_msg,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        market_id: msg.market_id.to_string(),
        manager: info.sender.into(),
    };

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Subscribe { subaccount_id, amount } => subscribe(deps, env, info.sender, subaccount_id, amount),
    }
}

pub fn subscribe(
    _deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    subaccount_id: String,
    amount: Coin,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let contract = env.contract.address;

    let querier = InjectiveQuerier::new(&_deps.querier);
    let res: SubaccountDepositResponse = querier.query_subaccount_deposit(subaccount_id.clone(), amount.denom.clone().into())?;

    // just log the available balance for now
    _deps.api.debug(res.deposits.available_balance.to_string().as_str());

    let msg = create_subaccount_transfer_msg(
        sender,
        subaccount_id.into(),
        contract.into(),
        amount,
    );

    let res = Response::new().add_message(msg);
    Ok(res)
}


#[entry_point]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
    }
}
