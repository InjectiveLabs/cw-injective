use crate::{traits::general::MarketId, types::injective::exchange::v1beta1::Position};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Empty, StdError, StdResult};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

pub trait PositionExtension {
    fn get_is_long(&self) -> bool;
    fn get_quantity(&self) -> FPDecimal;
    fn get_entry_price(&self) -> FPDecimal;
    fn get_margin(&self) -> FPDecimal;
    fn get_cumulative_funding_entry(&self) -> FPDecimal;
    fn get_position_value_without_funding(&self, valuation_price: FPDecimal) -> FPDecimal;
    fn get_position_value_with_funding(&self, valuation_price: FPDecimal, cumulative_funding: FPDecimal) -> FPDecimal;
    fn apply_funding(&mut self, cumulative_funding: FPDecimal);
}

impl PositionExtension for Position {
    fn get_is_long(&self) -> bool {
        self.isLong
    }

    fn get_quantity(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.quantity)
    }

    fn get_entry_price(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.entry_price)
    }

    fn get_margin(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.margin)
    }

    fn get_cumulative_funding_entry(&self) -> FPDecimal {
        FPDecimal::must_from_str(&self.cumulative_funding_entry)
    }

    fn get_position_value_without_funding(&self, valuation_price: FPDecimal) -> FPDecimal {
        let pnl = if self.get_is_long() {
            self.get_quantity() * (valuation_price - self.get_entry_price())
        } else {
            self.get_quantity() * (self.get_entry_price() - valuation_price)
        };

        self.get_margin() + pnl
    }

    fn get_position_value_with_funding(&self, valuation_price: FPDecimal, cumulative_funding: FPDecimal) -> FPDecimal {
        if self.isLong {
            let pnl = self.get_quantity() * (valuation_price - self.get_entry_price());
            let unrealized_funding = self.get_quantity() * (self.get_cumulative_funding_entry() - cumulative_funding);

            return self.get_margin() + pnl + unrealized_funding;
        }

        let pnl = self.get_quantity() * (self.get_entry_price() - valuation_price);
        let unrealized_funding = self.get_quantity() * (cumulative_funding - self.get_cumulative_funding_entry());

        self.get_margin() + pnl + unrealized_funding
    }

    fn apply_funding(&mut self, cumulative_funding: FPDecimal) {
        let unrealized_funding = self.get_quantity()
            * if self.isLong {
                self.get_cumulative_funding_entry() - cumulative_funding
            } else {
                cumulative_funding - self.get_cumulative_funding_entry()
            };

        self.margin += &unrealized_funding.to_string();
        self.cumulative_funding_entry = cumulative_funding.to_string();
    }
}
