use crate::{
    state::{config_read, State},
    utils::wrap_from_state,
};
use cosmwasm_std::{Coin, Decimal256 as Decimal, Deps, StdError};
use injective_bindings::InjectiveQueryWrapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub manager: String,
    pub market_id: String,
    pub sub_account: String,
    pub fee_recipient: String,
    pub risk_aversion: String,
    pub order_density: String,
    pub active_capital_perct: String,
    pub max_notional_position: String,
    pub min_pnl: String,
    pub manual_offset_perct: String,
    pub tail_dist_to_head_bp: String,
    pub head_chg_tol_bp: String,
    pub max_dd: String,
    pub leverage: String,
    pub decimal_shift: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Subscribe { subaccount_id: String, amount: Coin },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    GetAction {
        is_deriv: bool,
        open_orders: Vec<OpenOrder>,
        position: Option<Position>, // Will be None if is deriv == false
        inv_base_val: String,       // Will be 0.0 if deriv == true
        inv_val: String, // This includes any notional balance that may be tied up in a position
        std_dev: String,
        mid_price: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
    pub is_long: bool,
    pub quantity: String,
    pub avg_price: String,
    pub margin: String,
    pub cum_funding_entry: String,
}
impl Position {
    pub fn wrap(&self, deps: Deps<InjectiveQueryWrapper>) -> Result<WrappedPosition, StdError> {
        let state = config_read(deps.storage).load()?;
        Ok(WrappedPosition {
            is_long: self.is_long,
            quantity: Decimal::from_str(&self.quantity).unwrap(),
            avg_price: wrap_from_state(&self.avg_price, &state),
            margin: wrap_from_state(&self.margin, &state),
            cum_funding_entry: wrap_from_state(&self.cum_funding_entry, &state),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OpenOrder {
    pub order_hash: String,
    pub is_buy: bool,
    pub qty: String,
    pub price: String,
}
impl OpenOrder {
    pub fn wrap(&self, deps: Deps<InjectiveQueryWrapper>) -> Result<WrappedOpenOrder, StdError> {
        let state = config_read(deps.storage).load()?;
        Ok(WrappedOpenOrder {
            order_hash: self.order_hash.clone(),
            is_buy: self.is_buy,
            qty: Decimal::from_str(&self.qty).unwrap(),
            price: wrap_from_state(&self.price, &state),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WrappedPosition {
    pub is_long: bool,
    pub quantity: Decimal,
    pub avg_price: Decimal,
    pub margin: Decimal,
    pub cum_funding_entry: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WrappedOpenOrder {
    pub order_hash: String,
    pub is_buy: bool,
    pub qty: Decimal,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedOrderResponse {
    pub market_id: String,
    pub subaccount_id: String,
    pub fee_recipient: String,
    pub price: String,
    pub quantity: String,
    pub leverage: String,
    pub is_buy: bool,
    pub is_reduce_only: bool,
}
impl WrappedOrderResponse {
    pub fn new(
        state: &State,
        price: Decimal,
        quantity: Decimal,
        is_buy: bool,
        is_reduce_only: bool,
    ) -> WrappedOrderResponse {
        // TODO: find out if there's a limit on decimal precision for quantity and leverage. If so,
        // we need to round them to the number of significant digits.
        WrappedOrderResponse {
            market_id: state.market_id.clone(),
            subaccount_id: state.sub_account.clone(),
            fee_recipient: state.fee_recipient.clone(),
            price: format!("{:.0}", (price * state.decimal_shift).to_string()),
            quantity: quantity.to_string(),
            leverage: state.leverage.to_string(),
            is_buy,
            is_reduce_only,
        }
    }
}
impl fmt::Display for WrappedOrderResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = if self.is_buy { "BUY" } else { "SELL" };
        write!(
            f,
            "{} ${} {} {} {}",
            side, self.price, self.quantity, self.is_reduce_only, self.leverage
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedGetActionResponse {
    pub hashes_to_cancel: Vec<String>,
    pub orders_to_open: Vec<WrappedOrderResponse>,
}
impl fmt::Display for WrappedGetActionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut log = String::from("");
        for order in self.orders_to_open.iter() {
            log = format!("{}\n{}", log, order);
        }
        write!(f, "{}", log)
    }
}
