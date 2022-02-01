use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal256 as Decimal, Storage, Uint256};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub manager: Addr,
    pub market_id: String,
    pub fee_recipient: String,
    pub sub_account: String,
    pub is_deriv: bool,
    pub leverage: Decimal,
    pub order_density: Uint256,
    pub mid_price: Decimal,
    pub volitility: Decimal,
    pub last_update_utc: i64,
    pub min_market_data_delay_sec: i64,
    pub reservation_param: Decimal,
    pub spread_param: Decimal,
    pub active_capital_perct: Decimal,
    pub head_chg_tol_perct: Decimal,
    pub tail_dist_from_mid_perct: Decimal,
    pub min_tail_dist_perct: Decimal,
    pub decimal_shift: Uint256,
    pub base_precision_shift: Uint256,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
