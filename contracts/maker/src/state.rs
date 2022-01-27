use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal256 as Decimal, Storage, Uint256};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub market_id: String,
    pub is_deriv: bool,
    pub manager: String,
    pub sub_account: String,
    pub fee_recipient: String,
    pub risk_aversion: Decimal,
    pub order_density: Uint256,
    pub active_capital_perct: Decimal,
    pub manual_offset_perct: Decimal,
    pub min_tail_dist_perct: Decimal,
    pub tail_dist_from_mid_perct: Decimal,
    pub head_chg_tol_perct: Decimal,
    pub leverage: Decimal,
    pub decimal_shift: Uint256,
    pub base_precision_shift: Uint256,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
