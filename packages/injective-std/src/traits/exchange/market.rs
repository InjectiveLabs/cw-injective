use crate::{
    traits::{general::MarketId, oracle::OracleType},
    types::injective::exchange::v1beta1::{Deposit, DerivativeMarket, ExchangeQuerier, MidPriceAndTob, PerpetualMarketFunding, SpotMarket},
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Empty, StdError, StdResult};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use std::fmt;

pub trait MarketExtension {
    fn get_ticker(&self) -> &str;
    fn get_quote_denom(&self) -> &str;
    fn get_maker_fee_rate(&self) -> FPDecimal;
    fn get_taker_fee_rate(&self) -> FPDecimal;
    fn get_market_id(&self) -> MarketId;
    fn get_status(&self) -> i32;
    fn get_min_price_tick_size(&self) -> FPDecimal;
    fn get_min_quantity_tick_size(&self) -> FPDecimal;
}

pub trait DerivativeMarketExtension {
    fn get_initial_margin_ratio(&self) -> FPDecimal;
    fn get_maintenance_margin_ratio(&self) -> FPDecimal;
    fn get_oracle_type(&self) -> OracleType;
}

pub trait SpotMarketExtension {
    fn get_base_denom(&self) -> &str;
}

pub trait PerpetualMarketFundingExtension {
    fn get_cumulative_funding(&self) -> FPDecimal;
    fn get_cumulative_price(&self) -> FPDecimal;
    fn get_last_timestamp(&self) -> i64;
}

pub trait PerpetualMarketInfoExtension {
    fn get_market_id(&self) -> MarketId;
    fn get_hourly_funding_rate_cap(&self) -> FPDecimal;
    fn get_hourly_interest_rate(&self) -> FPDecimal;
    fn get_next_funding_timestamp(&self) -> i64;
    fn get_funding_interval(&self) -> i64;
}

pub trait MidPriceAndTobExtension {
    fn get_mid_price(&self) -> FPDecimal;
    fn get_best_buy_price(&self) -> FPDecimal;
    fn get_best_sell_price(&self) -> FPDecimal;
}

pub trait DepositExtension {
    fn get_available_balance(&self) -> FPDecimal;
    fn get_total_balance(&self) -> FPDecimal;
}

impl MarketExtension for DerivativeMarket {
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

    fn get_min_quantity_tick_size(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.min_quantity_tick_size)
    }
}

impl DerivativeMarketExtension for DerivativeMarket {
    fn get_initial_margin_ratio(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.initial_margin_ratio)
    }

    fn get_maintenance_margin_ratio(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.maintenance_margin_ratio)
    }

    fn get_oracle_type(&self) -> OracleType {
        OracleType::from_i32(self.oracle_type)
    }
}

impl SpotMarketExtension for SpotMarket {
    fn get_base_denom(&self) -> &str {
        &self.base_denom
    }
}

impl MarketExtension for SpotMarket {
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

    fn get_min_quantity_tick_size(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.min_quantity_tick_size)
    }
}

impl PerpetualMarketFundingExtension for PerpetualMarketFunding {
    fn get_cumulative_funding(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.cumulative_funding)
    }

    fn get_cumulative_price(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.cumulative_price)
    }

    fn get_last_timestamp(&self) -> i64 {
        self.last_timestamp
    }
}

impl MidPriceAndTobExtension for MidPriceAndTob {
    fn get_mid_price(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.mid_price)
    }

    fn get_best_buy_price(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.best_buy_price)
    }

    fn get_best_sell_price(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.best_sell_price)
    }
}

impl DepositExtension for Deposit {
    fn get_available_balance(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.available_balance)
    }

    fn get_total_balance(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.total_balance)
    }
}
