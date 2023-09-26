use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::MarketId;

#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Debug, PartialEq, Eq, JsonSchema, Copy)]
#[repr(i32)]
pub enum MarketStatus {
    #[default]
    Unspecified = 0,
    Active = 1,
    Paused = 2,
    Demolished = 3,
    Expired = 4,
}

pub trait GenericMarket {
    fn get_ticker(&self) -> &str;
    fn get_quote_denom(&self) -> &str;
    fn get_maker_fee_rate(&self) -> FPDecimal;
    fn get_taker_fee_rate(&self) -> FPDecimal;
    fn get_market_id(&self) -> &MarketId;
    fn get_status(&self) -> MarketStatus;
    fn get_min_price_tick_size(&self) -> FPDecimal;
    fn min_quantity_tick_size(&self) -> FPDecimal;
}
