use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read};

pub static KEY_CONFIG: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub distribution_contract: CanonicalAddr, // collected rewards receiver
    pub ninja_token: CanonicalAddr,
}

pub const SUBCONTRACT_TO_SUBACCOUNT_ID: Map<&str, String> = Map::new("subcontract_to_subaccount_id");
pub const SUBACCOUNT_ID_TO_SUBCONTRACT: Map<&str, String> = Map::new("subaccount_id_to_subcontract");

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}
