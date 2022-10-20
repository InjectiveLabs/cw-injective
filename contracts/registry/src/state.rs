use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};

// Contract struct defines begin blocker contract execution params.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CONTRACT {
    pub gas_limit: u64,
    pub gas_price: u64,
    pub is_executable: bool,
}

// pub const CONTRACTS: Map<&Addr, CONTRACT> = Map::new("contracts");

pub struct ContractIndexes<'a> {
    pub active: MultiIndex<'a, u8, CONTRACT, Addr>,
}

impl<'a> IndexList<CONTRACT> for ContractIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CONTRACT>> + '_> {
        let v: Vec<&dyn Index<CONTRACT>> = vec![&self.active];
        Box::new(v.into_iter())
    }
}

pub fn contracts<'a>() -> IndexedMap<'a, &'a Addr, CONTRACT, ContractIndexes<'a>> {
    let indexes = ContractIndexes {
        active: MultiIndex::new(
            |_, c: &CONTRACT| {
                if c.is_executable {
                    1u8
                } else {
                    0u8
                }
            },
            "contracts",
            "contracts__active",
        ),
    };
    IndexedMap::new("contracts", indexes)
}
