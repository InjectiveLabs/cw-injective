use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal256 as Decimal, Storage, Uint256};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub manager: Addr,                 // Contract creator's address
    pub market_id: String,             // Market Id
    pub sub_account: String,           // The contract's delegated subaccount
    pub is_deriv: bool,                // Whether the contract will be operating on a derivative market
    pub leverage: Decimal,             // Leverage that a contract will use on its orders
    pub order_density: Uint256,        // Number of orders to place between the head and the tail
    pub mid_price: Decimal,            // Some measure of mid_price determined by and external caller
    pub volitility: Decimal,           // Some measure of volitility determined by and external caller
    pub last_update_utc: i64,          // The last utc time stamp in seconds where we updated volitility and mid_price
    pub max_market_data_delay: i64, // The maximum time in sec we are willing to tolerate since the last market data update for which the contract will behave expectedly
    pub reservation_param: Decimal, // A constant between 0..1 that will be used to control the sensitivity of the reservation_price
    pub spread_param: Decimal,      // A constant between 0..1 that will be used to control the sensitivity of the spread around the mid_price
    pub active_capital: Decimal,    // A constant between 0..1 that will be used to determine how much of our capital we want resting on the book
    pub head_chg_tol: Decimal, // A threshold for which we actually want to take action between 0..1 (if new head is more than x dist away from old head)
    pub tail_dist_from_mid: Decimal, // The distance between 0..1 from the mid_price that we want to place our tails
    pub min_tail_dist: Decimal, // The minimum between 0..1 format from the head that we want our tail (risk management param)
    pub decimal_shift: Uint256, // 10^(number of decimals of the quote currency)
    pub base_precision_shift: Uint256, // 10^(decimal precision of base quantity respective of the market)
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
