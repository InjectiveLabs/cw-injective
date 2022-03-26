use cosmwasm_std::{
     Addr, DepsMut, Response, StdResult, Uint128,
};

// pub fn bond(
//     _deps: DepsMut,
//     staker_addr: Addr,
//     asset_token: Addr,
//     amount: Uint128,
// ) -> StdResult<Response> {
//     // let staker_addr_raw: CanonicalAddr = deps.api.addr_canonicalize(staker_addr.as_str())?;
//     // let asset_token_raw: CanonicalAddr = deps.api.addr_canonicalize(asset_token.as_str())?;
//     // _increase_bond_amount(
//     //     deps.storage,
//     //     &staker_addr_raw,
//     //     &asset_token_raw,
//     //     amount,
//     //     false,
//     // )?;
//
//     Ok(Response::new().add_attributes(vec![
//         attr("action", "bond"),
//         attr("staker_addr", staker_addr.as_str()),
//         attr("asset_token", asset_token.as_str()),
//         attr("amount", amount.to_string()),
//     ]))
// }

pub fn unbond(
    _deps: DepsMut,
    _staker_addr: Addr,
    _asset_token: Addr,
    _amount: Uint128,
) -> StdResult<Response> {
    Ok(Response::new())

    // let staker_addr_raw: CanonicalAddr = deps.api.addr_canonicalize(staker_addr.as_str())?;
    // let asset_token_raw: CanonicalAddr = deps.api.addr_canonicalize(asset_token.as_str())?;
    // let staking_token: CanonicalAddr = _decrease_bond_amount(
    //     deps.storage,
    //     &staker_addr_raw,
    //     &asset_token_raw,
    //     amount,
    //     false,
    // )?;
    // let staking_token_addr: Addr = deps.api.addr_humanize(&staking_token)?;

    // Ok(Response::new
    //     //     .add_message(CosmosMsg::Wasm(WasmMsg::E()xecute {
    //         contract_addr: deps.api.addr_humanize(&staking_token)?.to_string(),
    //         msg: to_binary(&Cw20ExecuteMsg::Transfer {
    //             recipient: staker_addr.to_string(),
    //             amount,
    //         })?,
    //         funds: vec![],
    //     }))
    //     .add_attributes(vec![
    //         attr("action", "unbond"),
    //         attr("staker_addr", staker_addr.as_str()),
    //         attr("asset_token", asset_token.as_str()),
    //         attr("amount", amount.to_string()),
    //         // attr("staking_token", staking_token_addr.as_str()),
    //     ]))
}
