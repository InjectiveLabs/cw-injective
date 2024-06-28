use cosmwasm_std::{CosmosMsg, StdResult};
use injective_cosmwasm::{InjectiveMsgWrapper, OrderType, SpotMarket};
use injective_math::{display::ToProto, round_to_min_tick, round_to_nearest_tick, FPDecimal};
use injective_std::types::injective::exchange::v1beta1 as Exchange;
use prost::Message;

pub fn create_spot_market_order_message(
    price: FPDecimal,
    quantity: FPDecimal,
    order_type: OrderType,
    sender: &str,
    subaccount_id: &str,
    fee_recipient: &str,
    market: &SpotMarket,
) -> StdResult<CosmosMsg<InjectiveMsgWrapper>> {
    let msg = create_spot_market_order(price, quantity, order_type, sender, subaccount_id, fee_recipient, market);

    let mut order_bytes = vec![];
    Exchange::MsgCreateSpotMarketOrder::encode(&msg, &mut order_bytes).unwrap();

    Ok(CosmosMsg::Stargate {
        type_url: Exchange::MsgCreateSpotMarketOrder::TYPE_URL.to_string(),
        value: order_bytes.into(),
    })
}

fn create_spot_market_order(
    price: FPDecimal,
    quantity: FPDecimal,
    order_type: OrderType,
    sender: &str,
    subaccount_id: &str,
    fee_recipient: &str,
    market: &SpotMarket,
) -> Exchange::MsgCreateSpotMarketOrder {
    let rounded_quantity = round_to_min_tick(quantity, market.min_quantity_tick_size);
    let rounded_price = round_to_nearest_tick(price, market.min_price_tick_size);

    Exchange::MsgCreateSpotMarketOrder {
        sender: sender.to_string(),
        order: Some(Exchange::SpotOrder {
            market_id: market.market_id.as_str().into(),
            order_info: Some(Exchange::OrderInfo {
                subaccount_id: subaccount_id.to_string(),
                fee_recipient: fee_recipient.to_string(),
                price: rounded_price.to_proto_string(),
                quantity: rounded_quantity.to_proto_string(),
                cid: "".to_string(),
            }),
            order_type: order_type as i32,
            trigger_price: "".to_string(),
        }),
    }
}
