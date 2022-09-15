use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;


pub const STATE: Item<ContractConfigState> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractConfigState {
    pub market_id: String,
    pub owner: Addr,
    pub base_denom: String,
    pub quote_denom : String,
}

pub const CACHE : Item<SwapCacheState> = Item::new("cache");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapCacheState {
    pub sender_address: String,
    pub deposited_amount: Coin
}
