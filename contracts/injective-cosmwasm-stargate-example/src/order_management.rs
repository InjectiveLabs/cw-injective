use crate::types;
use cosmwasm_std::{CosmosMsg, StdResult};
use injective_cosmwasm::{FullDerivativeMarket, InjectiveMsgWrapper, OrderType, SpotMarket};
use injective_math::FPDecimal;
use prost::Message;

pub fn create_stargate_msg(type_url: &str, value: Vec<u8>) -> StdResult<CosmosMsg<InjectiveMsgWrapper>> {
    Ok(CosmosMsg::Stargate {
        type_url: type_url.to_string(),
        value: value.into(),
    })
}

pub fn create_spot_limit_order(
    price: FPDecimal,
    quantity: FPDecimal,
    order_type: OrderType,
    sender: &str,
    subaccount_id: &str,
    market: &SpotMarket,
) -> types::MsgCreateSpotLimitOrder {
    types::MsgCreateSpotLimitOrder {
        sender: sender.to_string(),
        order: Some(types::SpotOrder {
            market_id: market.market_id.as_str().into(),
            order_info: Some(types::OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient: sender.to_string(),
                price: price.to_string(),
                quantity: quantity.to_string(),
            }),
            order_type: order_type as i32,
            trigger_price: "".to_string(),
        }),
    }
}

pub fn create_derivative_limit_order(
    price: FPDecimal,
    quantity: FPDecimal,
    margin: FPDecimal,
    order_type: OrderType,
    sender: &str,
    subaccount_id: &str,
    market: &FullDerivativeMarket,
) -> types::MsgCreateDerivativeLimitOrder {
    let market_id = market.market.as_ref().unwrap().market_id.as_str().to_string();

    types::MsgCreateDerivativeLimitOrder {
        sender: sender.to_string(),
        order: Some(types::DerivativeOrder {
            market_id,
            order_info: Some(types::OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient: sender.to_string(),
                price: price.to_string(),
                quantity: quantity.to_string(),
            }),
            order_type: order_type as i32,
            margin: margin.to_string(),
            trigger_price: "".to_string(),
        }),
    }
}

pub(crate) fn encode_bytes_message<T: Message>(order_msg: &T) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buffer = Vec::new();
    order_msg.encode(&mut buffer)?; // Encode the message using prost
    Ok(buffer)
}
