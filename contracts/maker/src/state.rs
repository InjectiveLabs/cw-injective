use cw20::Cw20Contract;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal256 as Decimal, Storage, Uint256};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub market_id: String,     // Market Id
    pub subaccount_id: String, // The contract's delegated subaccount
    pub fee_recipient: String,
    pub leverage: Decimal,                             // Leverage that a contract will use on its orders
    pub order_density: Uint256,                        // Number of orders to place between the head and the tail
    pub reservation_price_sensitivity_ratio: Decimal, // A constant between 0..1 that will be used to control the sensitivity of the reservation_price from mid_price
    pub reservation_spread_sensitivity_ratio: Decimal, // A constant between 0..1 that will be used to control the sensitivity of the spread around the reservation_price
    pub max_active_capital_utilization_ratio: Decimal, // A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book
    pub head_change_tolerance_ratio: Decimal, // A constant between 0..1 that serves as a threshold for which we actually want to take action in the new block
    pub mid_price_tail_deviation_ratio: Decimal, // A constant between 0..1 that is used to determine how far we want to place our tails from the mid_price
    pub min_head_to_tail_deviation_ratio: Decimal, // A constant between 0..1 that ensures our tail is at least some distance from the head (risk management param)
    pub min_proximity_to_liquidation: Decimal, // A constant between 0..1 that represents the minimum proximity to liquidation we are willing to tolerate
    pub post_reduction_perc_of_max_position: Decimal, // A constant between 0..1. Our new position will be this percent under the max position value after market order reduction
    pub lp_token_address: Option<Cw20Contract>,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
