use crate::{encode_helper::encode_proto_message, query::handle_query_stargate, state::ORDER_CALL_CACHE, ContractError};
use cosmwasm_std::{DepsMut, Event, Reply, Response};
use injective_cosmwasm::InjectiveQueryWrapper;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct QueryTraderSpotOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    #[serde(alias = "subaccountID")]
    pub subaccount_id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct QueryTraderDerivativeOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    #[serde(alias = "subaccountID")]
    pub subaccount_id: ::prost::alloc::string::String,
}

pub fn handle_create_order_reply_stargate(deps: DepsMut<InjectiveQueryWrapper>, _msg: &Reply) -> Result<Response, ContractError> {
    let mut response_str = "Something went wrong".to_string();
    if let Some(mut cache) = ORDER_CALL_CACHE.may_load(deps.storage)? {
        if !cache.is_empty() {
            let order_info = &cache[0];
            let encode_query_message = encode_proto_message(QueryTraderSpotOrdersRequest {
                market_id: order_info.market_id.clone().into(),
                subaccount_id: order_info.subaccount.clone().into(),
            });
            let stargate_response = handle_query_stargate(
                &deps.querier,
                "/injective.exchange.v1beta1.Query/TraderSpotTransientOrders".to_string(),
                encode_query_message,
            );
            response_str = match stargate_response {
                Ok(binary) => String::from_utf8(binary.0).unwrap_or_else(|e| format!("Failed to decode binary to string: {:?}", e)),
                Err(e) => format!("Error: {:?}", e),
            };
            cache.clear();
            ORDER_CALL_CACHE.save(deps.storage, &cache)?;
        }
    };

    Ok(Response::new().add_event(Event::new("transient_order").add_attributes([("query_str", response_str)])))
}

pub fn handle_create_derivative_order_reply_stargate(deps: DepsMut<InjectiveQueryWrapper>, _msg: &Reply) -> Result<Response, ContractError> {
    let mut response_str = "Something went wrong".to_string();

    if let Some(mut cache) = ORDER_CALL_CACHE.may_load(deps.storage)? {
        if !cache.is_empty() {
            let order_info = &cache[0];
            let encode_query_message = encode_proto_message(QueryTraderDerivativeOrdersRequest {
                market_id: order_info.market_id.clone().into(),
                subaccount_id: order_info.subaccount.clone().into(),
            });
            let stargate_response = handle_query_stargate(
                &deps.querier,
                "/injective.exchange.v1beta1.Query/TraderDerivativeTransientOrders".to_string(),
                encode_query_message,
            );
            response_str = match stargate_response {
                Ok(binary) => String::from_utf8(binary.0).unwrap_or_else(|e| format!("Failed to decode binary to string: {:?}", e)),
                Err(e) => format!("Error: {:?}", e),
            };
            cache.clear();
            ORDER_CALL_CACHE.save(deps.storage, &cache)?;
        }
    };

    Ok(Response::new().add_event(Event::new("transient_derivative_order").add_attributes([("query_str", response_str)])))
}
