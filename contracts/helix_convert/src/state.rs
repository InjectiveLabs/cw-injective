use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQueryWrapper, MarketId, SubaccountId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Deps, DepsMut, StdError, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use injective_math::FPDecimal;
use crate::types::{Config, CurrentSwapOperation, CurrentSwapStep, SwapRoute};



pub const SWAP_ROUTES: Map<(&str, &str), SwapRoute> = Map::new("swap_routes");
pub const SWAP_OPERATION_STATE: Item<CurrentSwapOperation> = Item::new("current_swap_cache");
pub const STEP_STATE: Item<CurrentSwapStep> = Item::new("current_step_cache");
pub const CONFIG: Item<Config> = Item::new("config");


pub fn store_swap_route(storage: &mut dyn Storage, route: &SwapRoute) -> StdResult<()> {
    let key = route_key(&route.denom_1, &route.denom_2);
    SWAP_ROUTES.save(storage, key, &route)
}

pub fn read_swap_route(storage: &dyn Storage, denom1: &str, denom2: &str) -> StdResult<SwapRoute> {
    let key = route_key(denom1, denom2);
    SWAP_ROUTES.load(storage, key).map_err(StdError::from)
}

fn route_key<'a>(denom1: &'a str, denom2: &'a str) -> (&'a str, &'a str) {
    if denom1 < denom2 {
        (denom1, denom2)
    } else {
        (denom2, denom1)
    }
}
