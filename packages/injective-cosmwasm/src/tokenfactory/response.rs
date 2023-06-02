use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TokenFactoryDenomSupplyResponse {
    pub total_supply: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TokenFactoryCreateDenomFeeResponse {
    pub fee: Vec<Coin>,
}
