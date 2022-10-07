use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

// Contract struct defines begin blocker contract execution params.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CONTRACT {
    pub gas_limit: u64,
    pub gas_price: u128,
    pub is_executable: bool,
}

pub const CONTRACTS: Map<&Addr, CONTRACT> = Map::new("contracts");
