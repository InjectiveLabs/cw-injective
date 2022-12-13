use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};

// Contract struct defines begin blocker contract execution params.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Contract {
    pub gas_limit: u64,
    pub gas_price: u64,
    pub is_executable: bool,
}

pub(crate) const INACTIVE_CONTRACT: u8 = 0;

pub struct ContractIndexes<'a> {
    pub active_by_gasprice_addr: MultiIndex<'a, u64, Contract, Addr>,
    // pub active: MultiIndex<'a, u8, Contract, Addr>,
}

impl<'a> IndexList<Contract> for ContractIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Contract>> + '_> {
        let v: Vec<&dyn Index<Contract>> = vec![&self.active_by_gasprice_addr];
        Box::new(v.into_iter())
    }
}

pub fn contracts<'a>() -> IndexedMap<'a, &'a Addr, Contract, ContractIndexes<'a>> {
    let indexes = ContractIndexes {
        active_by_gasprice_addr: MultiIndex::new(
            |_, c: &Contract| {
                if c.is_executable {
                    c.gas_price
                } else {
                    u64::from(INACTIVE_CONTRACT)
                }
            },
            "contracts",
            "contracts__active",
        ),
    };
    IndexedMap::new("contracts", indexes)
}
