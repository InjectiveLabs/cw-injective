use injective_cosmwasm::{MarketId, SubaccountId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

pub const STATE: Item<ContractConfigState> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractConfigState {
    pub market_id: MarketId,
    pub owner: Addr,
    pub contract_subaccount_id: SubaccountId,
    pub base_denom: String,
    pub quote_denom: String,
}

pub const SWAP_OPERATION_STATE: Item<SwapCacheState> = Item::new("cache");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SwapCacheState {
    pub sender_address: String,
    pub deposited_amount: Coin,
}
