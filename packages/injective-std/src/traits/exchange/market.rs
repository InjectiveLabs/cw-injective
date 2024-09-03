use crate::{
    traits::general::MarketId,
    types::injective::exchange::v1beta1::{DerivativeMarket, ExchangeQuerier, SpotMarket},
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Empty, StdError, StdResult};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use std::fmt;

impl GenericMarket for DerivativeMarket {
    fn get_ticker(&self) -> &str {
        &self.ticker
    }

    fn get_quote_denom(&self) -> &str {
        &self.quote_denom
    }

    fn get_maker_fee_rate(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.maker_fee_rate)
    }

    fn get_taker_fee_rate(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.taker_fee_rate)
    }

    fn get_market_id(&self) -> MarketId {
        MarketId::new(self.market_id.clone()).unwrap()
    }

    fn get_status(&self) -> i32 {
        self.status
    }

    fn get_min_price_tick_size(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.min_price_tick_size)
    }

    fn min_quantity_tick_size(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.min_quantity_tick_size)
    }
}

impl GenericMarket for SpotMarket {
    fn get_ticker(&self) -> &str {
        &self.ticker
    }

    fn get_quote_denom(&self) -> &str {
        &self.quote_denom
    }

    fn get_maker_fee_rate(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.maker_fee_rate)
    }

    fn get_taker_fee_rate(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.taker_fee_rate)
    }

    fn get_market_id(&self) -> MarketId {
        MarketId::new(self.market_id.clone()).unwrap()
    }

    fn get_status(&self) -> i32 {
        self.status
    }

    fn get_min_price_tick_size(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.min_price_tick_size)
    }

    fn min_quantity_tick_size(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.min_quantity_tick_size)
    }
}

pub trait GenericMarket {
    fn get_ticker(&self) -> &str;
    fn get_quote_denom(&self) -> &str;
    fn get_maker_fee_rate(&self) -> FPDecimal;
    fn get_taker_fee_rate(&self) -> FPDecimal;
    fn get_market_id(&self) -> MarketId;
    fn get_status(&self) -> i32;
    fn get_min_price_tick_size(&self) -> FPDecimal;
    fn min_quantity_tick_size(&self) -> FPDecimal;
}

pub enum MarketType {
    Spot,
    Derivative,
}
