use crate::state::{config_read, State};
use cosmwasm_std::{Coin, Decimal256 as Decimal, Deps, Fraction, StdError, Uint256};
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
    pub price_distribution_rate: String,
    pub slices_per_spread_bp: String,
    pub ratio_active_capital: String,
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
        position: Position,
        total_notional_balance: String,
        standard_deviation: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WrappedPosition {
    pub is_long: bool,
    pub quantity: Decimal,
    pub avg_price: Decimal,
    pub margin: Decimal,
    pub cum_funding_entry: Decimal,
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
        decimal_shift: Decimal,
        price: Decimal,
        quantity: Decimal,
        is_buy: bool,
        is_reduce_only: bool,
        leverage: Decimal,
    ) -> WrappedOrderResponse {
        // TODO: find out if there's a limit on decimal precision for quantity and leverage. If so,
        // we need to round them to the number of significant digits.
        WrappedOrderResponse {
            market_id: state.market_id.clone(),
            subaccount_id: state.sub_account.clone(),
            fee_recipient: state.fee_recipient.clone(),
            price: format!("{:.0}", (price * decimal_shift).to_string()),
            quantity: quantity.to_string(),
            leverage: leverage.to_string(),
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

pub fn wrap(unwrapped_num: &String, deps: Deps<InjectiveQueryWrapper>) -> Decimal {
    let state = config_read(deps.storage).load().unwrap();
    Decimal::from_str(unwrapped_num).unwrap() / Uint256::from_str(&state.decimal_shift).unwrap()
}

fn wrap_from_state(unwrapped_num: &String, state: &State) -> Decimal {
    let shift = Uint256::from_str(&state.decimal_shift).unwrap();
    Decimal::from_str(unwrapped_num).unwrap()
        * Decimal::from_ratio(Uint256::from_str("1").unwrap(), shift)
}

pub fn div_int(num: Decimal, denom: Uint256) -> Decimal {
    num * Decimal::from_ratio(Uint256::from_str("1").unwrap(), denom)
}

pub fn div_dec(num: Decimal, denom: Decimal) -> Decimal {
    num * denom.inv().unwrap()
}
