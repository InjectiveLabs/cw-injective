use crate::{
    state::{config_read, State},
    utils::{div_dec, round_to_precision, wrap_from_state},
};
use cosmwasm_std::{Decimal256 as Decimal, Deps, DepsMut, Env, StdError, Uint128, Uint256};
use injective_bindings::{InjectiveMsgWrapper, InjectiveQueryWrapper};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use std::io::Bytes;
use crate::exchange::{DerivativeLimitOrder, Position, Deposit, DerivativeMarket, PerpetualMarketInfo, PerpetualMarketFunding, ExchangeMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub manager: String,
    // Contract creator's address
    pub market_id: String,
    // Market Id
    pub sub_account: String,
    // The contract's delegated subaccount
    pub is_deriv: bool,
    // Whether the contract will be operating on a derivative market
    pub leverage: String,
    // Leverage that a contract will use on its orders
    pub order_density: String,
    // Number of orders to place between the head and the tail
    pub reservation_param: String,
    // A constant between 0..1 that will be used to control the sensitivity of the reservation_price
    pub spread_param: String,
    // A constant between 0..1 that will be used to control the sensitivity of the spread around the mid_price
    pub active_capital: String,
    // A constant between 0..1 that will be used to determine how much of our capital we want resting on the book
    pub head_chg_tol_bp: String,
    // A threshold for which we actually want to take action in BP (if new head is more than x dist away from old head)
    pub tail_dist_from_mid_bp: String,
    // The distance in BP from the mid_price that we want to place our tails
    pub min_tail_dist_bp: String,
    // The minimum distance in BP from the head that we want our tail (risk management param)
    pub max_market_data_delay: String,
    // The maximum time we are willing to tolerate since the last market data update for which the contract will behave expectedly
    pub decimal_shift: String,
    // 10^(number of decimals of the quote currency)
    pub base_precision_shift: String,
    // 10^(decimal precision of base quantity respective of the market)
    pub cw20_code_id: String,
    // CW20 Wasm contract code id
    pub lp_name: String,
    // LP Token Name
    pub lp_symbol: String,
    // LP Token Symbol
    pub lp_decimals: String,           // LP Token Decimals
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateMarketState { mid_price: String, volatility: String },
    // The chain will not be responsible for calling this
    MintToUser { subaccount_id_sender: String, amount: Uint128 },
    BurnFromUser { subaccount_id_sender: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    GetAction {
        market: DerivativeMarket,
        perpetual_market_info: Option<PerpetualMarketInfo>,
        perpetual_market_funding: Option<PerpetualMarketFunding>,
        // Trader's open orders that are currently on the book at the time of the call
        open_orders: Vec<DerivativeLimitOrder>,
        deposit: Deposit,
        position: Option<Position>,
        oracle_price: String,
    },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedOrderResponse {
    pub market_id: String,
    pub subaccount_id: String,
    pub price: String,
    pub quantity: String,
    pub leverage: String,
    pub is_buy: bool,
    pub is_reduce_only: bool,
    pub decimal_shift: String,
}

impl WrappedOrderResponse {
    pub fn new(state: &State, price: Decimal, quantity: Decimal, is_buy: bool, is_reduce_only: bool) -> WrappedOrderResponse {
        WrappedOrderResponse {
            market_id: state.market_id.clone(),
            subaccount_id: state.sub_account.clone(),
            price: round_to_precision(
                price * Decimal::from_str(&state.decimal_shift.to_string()).unwrap(),
                Uint256::from_str("1").unwrap(),
            )
                .to_string(),
            quantity: round_to_precision(quantity, state.base_precision_shift).to_string(),
            leverage: state.leverage.to_string(),
            is_buy,
            is_reduce_only,
            decimal_shift: state.decimal_shift.to_string(),
        }
    }
    pub fn get_price(&self) -> Decimal {
        div_dec(
            Decimal::from_str(&self.price).unwrap(),
            Decimal::from_str(&self.decimal_shift.to_string()).unwrap(),
        )
    }
    pub fn get_val(&self) -> Decimal {
        div_dec(
            Decimal::from_str(&self.quantity).unwrap() * Decimal::from_str(&self.price).unwrap(),
            Decimal::from_str(&self.decimal_shift.to_string()).unwrap(),
        )
    }
    pub fn get_qty(&self) -> Decimal {
        Decimal::from_str(&self.quantity).unwrap()
    }
}

impl fmt::Display for WrappedOrderResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = if self.is_buy { "BUY" } else { "SELL" };
        write!(
            f,
            "{} ${} {} {} {} val: {}",
            side,
            self.get_price(),
            self.quantity,
            self.is_reduce_only,
            self.leverage,
            self.get_val()
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedGetActionResponse {
    pub msgs: Vec<ExchangeMsg>,
}

impl fmt::Display for WrappedGetActionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut log = String::from("");
        for order in self.buy_orders_to_open.iter() {
            log = format!("{}\n{}", log, order);
        }
        for order in self.sell_orders_to_open.iter() {
            log = format!("{}\n{}", log, order);
        }
        write!(f, "{}", log)
    }
}
