use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub market_id: String,
    pub manager: String,
    pub sub_account: String,
    pub fee_recipient: String,
    pub risk_aversion: String,
    pub price_distribution_rate: String,
    pub slices_per_spread_bp: String,
    pub ratio_active_capital: String,
    pub leverage: String,
    pub decimal_shift: String,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
