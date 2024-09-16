use cosmwasm_std::Addr;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema, Copy)]
#[repr(i32)]
#[derive(Default)]
pub enum OracleType {
    #[default]
    Unspecified = 0,
    Band = 1,
    PriceFeed = 2,
    Coinbase = 3,
    Chainlink = 4,
    Razor = 5,
    Dia = 6,
    API3 = 7,
    Uma = 8,
    Pyth = 9,
    BandIBC = 10,
    Provider = 11,
}

impl OracleType {
    pub fn from_i32(value: i32) -> OracleType {
        match value {
            0 => OracleType::Unspecified,
            1 => OracleType::Band,
            2 => OracleType::PriceFeed,
            3 => OracleType::Coinbase,
            4 => OracleType::Chainlink,
            5 => OracleType::Razor,
            6 => OracleType::Dia,
            7 => OracleType::API3,
            8 => OracleType::Uma,
            9 => OracleType::Pyth,
            10 => OracleType::Band,
            11 => OracleType::Provider,
            _ => unimplemented!("Oracle type not supported!"),
        }
    }
}
