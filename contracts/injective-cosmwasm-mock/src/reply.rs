use crate::state::ORDER_CALL_CACHE;
use crate::ContractError;
use cosmwasm_std::{DepsMut, Event, Reply, Response};
use injective_cosmwasm::{InjectiveQuerier, InjectiveQueryWrapper};

pub fn handle_create_order_reply(deps: DepsMut<InjectiveQueryWrapper>, _msg: &Reply) -> Result<Response, ContractError> {
    let mut response_str = "Something went wrong".to_string();

    let querier: InjectiveQuerier = InjectiveQuerier::new(&deps.querier);

    if let Some(mut cache) = ORDER_CALL_CACHE.may_load(deps.storage)? {
        if !cache.is_empty() {
            let order_info = &cache[0];
            let response = querier.query_trader_transient_spot_orders(&order_info.market_id, &order_info.subaccount);
            response_str = format!("{:?}", &response);
            cache.clear();
            ORDER_CALL_CACHE.save(deps.storage, &cache)?;
        }
    };

    Ok(Response::new().add_event(Event::new("transient_order").add_attributes([("query_str", response_str)])))
}

pub fn handle_create_derivative_order_reply(deps: DepsMut<InjectiveQueryWrapper>, _msg: &Reply) -> Result<Response, ContractError> {
    let mut response_str = "Something went wrong".to_string();
    let querier: InjectiveQuerier = InjectiveQuerier::new(&deps.querier);

    if let Some(mut cache) = ORDER_CALL_CACHE.may_load(deps.storage)? {
        if !cache.is_empty() {
            let order_info = &cache[0];
            let response = querier.query_trader_transient_derivative_orders(&order_info.market_id, &order_info.subaccount);
            response_str = format!("{:?}", &response);
            cache.clear();
            ORDER_CALL_CACHE.save(deps.storage, &cache)?;
        }
    };

    Ok(Response::new().add_event(Event::new("transient_derivative_order").add_attributes([("query_str", response_str)])))
}
