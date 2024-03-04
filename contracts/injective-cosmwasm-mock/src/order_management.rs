use crate::types;
use cosmwasm_std::{to_json_binary, Coin, CosmosMsg, StdResult, SubMsg, Uint128, WasmMsg};
use injective_cosmwasm::{InjectiveMsgWrapper, OrderType, SpotMarket};
use injective_math::FPDecimal;


pub fn create_stargate_msg(type_url: &str, value: Vec<u8>) -> StdResult<CosmosMsg<InjectiveMsgWrapper>> {
    Ok(CosmosMsg::Stargate {
        type_url: type_url.to_string(),
        value: value.into(),
    })
}

pub fn create_spot_market_order(
    price: FPDecimal,
    quantity: FPDecimal,
    order_type: OrderType,
    sender: &str,
    subaccount_id: &str,
    market: &SpotMarket,
) -> types::MsgCreateSpotMarketOrder {
    types::MsgCreateSpotMarketOrder {
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
