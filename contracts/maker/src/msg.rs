use crate::exchange::ExchangeMsg;
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub market_id: String,
    // Market Id
    pub subaccount_id: String,
    pub fee_recipient: String,
    // Whether the contract will be operating on a derivative market
    pub leverage: String,
    // Leverage that a contract will use on its orders
    pub order_density: String,
    // Number of orders to place between the head and the tail
    pub reservation_price_sensitivity_ratio: String,
    // A constant between 0..1 that will be used to control the sensitivity of the reservation_price from mid_price
    pub reservation_spread_sensitivity_ratio: String,
    // A constant between 0..1 that will be used to control the sensitivity of the spread around the reservation_price
    pub max_active_capital_utilization_ratio: String,
    // A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book
    pub head_change_tolerance_ratio: String,
    // A constant between 0..1 that serves as a threshold for which we actually want to take action in the new block
    pub mid_price_tail_deviation_ratio: String,
    // A constant between 0..1 that is used to determine how far we want to place our tails from the midprice
    pub min_head_to_tail_deviation_ratio: String,
    // A constant between 0..1 that ensures our tail is at least some distance from the head (risk management param)
    pub cw20_code_id: String,
    // CW20 Wasm contract code id
    pub lp_name: String,
    // LP Token Name
    pub lp_symbol: String,
    // LP Token Symbol
    pub lp_decimals: String, // LP Token Decimals
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintToUser { subaccount_id_sender: String, amount: Uint128 },
    BurnFromUser { subaccount_id_sender: String, amount: Uint128 },
    BeginBlocker {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    GetTotalLpSupply {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedGetActionResponse {
    pub msgs: Vec<ExchangeMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalSupplyResponse {
    pub total_supply: Uint128,
}
