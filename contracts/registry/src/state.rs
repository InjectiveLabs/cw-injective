use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Contract struct defines begin blocker contract execution params.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CONTRACT {
    pub address: Addr,
    pub gas_limit: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contracts: Vec<CONTRACT>,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
