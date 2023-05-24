use cosmwasm_std::{StdError, StdResult, Storage};
use cw_storage_plus::{Item, Map};

use crate::types::{Config, CurrentSwapOperation, CurrentSwapStep, SwapRoute};



pub const SWAP_ROUTES: Map<(String, String), SwapRoute> = Map::new("swap_routes");
pub const SWAP_OPERATION_STATE: Item<CurrentSwapOperation> = Item::new("current_swap_cache");
pub const STEP_STATE: Item<CurrentSwapStep> = Item::new("current_step_cache");
pub const CONFIG: Item<Config> = Item::new("config");

pub fn store_swap_route(storage: &mut dyn Storage, route: &SwapRoute) -> StdResult<()> {
    let key = route_key(&route.denom_1, &route.denom_2);
    SWAP_ROUTES.save(storage, key, route)
}

pub fn read_swap_route(storage: &dyn Storage, denom1: &str, denom2: &str) -> StdResult<SwapRoute> {
    let key = route_key(denom1, denom2);
    SWAP_ROUTES.load(storage, key).map_err(|_| {
        StdError::generic_err(format!(
            "No swap route not found from {} to {}",
            denom1, denom2
        ))
    })
}

pub fn remove_swap_route(storage: &mut dyn Storage, denom_1: &str, denom_2: &str) {
    let key = route_key(denom_1, denom_2);
    SWAP_ROUTES.remove(storage, key)
}

fn route_key<'a>(denom1: &'a str, denom2: &'a str) -> (String, String) {
    if denom1 < denom2 {
        (denom1.to_string(), denom2.to_string())
    } else {
        (denom2.to_string(), denom1.to_string())
    }
}
