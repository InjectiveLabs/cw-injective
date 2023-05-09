use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQueryWrapper, MarketId, SubaccountId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Deps, DepsMut, StdError, StdResult};
use cw_storage_plus::{Item, Map};
use injective_math::FPDecimal;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    /// The 'fee_discount' field specifies the percentage of the trading fee that the contract will cover.
    /// The suggested default value for this field is 0.4 (the contract will return all trading fees it receives back from being designated as fee recipient)
    /// Any value above 0 requires the contract to have sufficient funds to provide for orders.
    pub fee_discount: FPDecimal,

    pub fee_recipient: Addr,
}

pub const ROUTES: Item<SwapRoute> = Item::new("routes");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SwapRoute {
    pub steps: Vec<MarketId>,
    pub denom_1: String,
    pub denom_2: String,
}

impl SwapRoute {

    pub fn steps_from(&self, denom: &str) -> Vec<MarketId> {
        if &self.denom_1 == denom {
            self.steps.clone()
        } else {
            let mut mut_steps = self.steps.clone();
            mut_steps.reverse();
            mut_steps
        }
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SwapStep {
    pub market_id : MarketId,
    pub quote_denom: String, // quote for this step of swap, eg for swpa eth/inj using eth/usdt and inj/usdt markets, quotes will be eth in 1st step and usdt in 2nd
}

pub const SWAP_ROUTES: Map<(&str, &str), SwapRoute> = Map::new("swap_routes");

pub fn store_swap_route(deps: &mut DepsMut<InjectiveQueryWrapper>, route: SwapRoute) -> StdResult<()> {
    let key = route_key(&route.denom_1, &route.denom_2);
    SWAP_ROUTES.save(deps.storage, key, &route)
}

pub fn read_swap_route(deps: &Deps<InjectiveQueryWrapper>, denom1: &str, denom2: &str) -> StdResult<SwapRoute> {
    let key = route_key(denom1, denom2);
    SWAP_ROUTES.load(deps.storage, key).map_err(StdError::from)
}

fn route_key<'a>(denom1: &'a str, denom2: &'a str) -> (&'a str, &'a str) {
    if denom1 < denom2 {
        (denom1, denom2)
    } else {
        (denom2, denom1)
    }
}

pub const SWAP_OPERATION_STATE: Item<SwapCacheState> = Item::new("cache");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SwapCacheState {
    pub sender_address: String,
    pub deposited_amount: Coin,
    pub route: SwapRoute
}
