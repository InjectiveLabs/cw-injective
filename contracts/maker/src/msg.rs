use crate::exchange::ExchangeMsg;
use cosmwasm_std::Uint128;
use cw20::Logo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // Market Id
    pub market_id: String,
    pub fee_recipient: String,
    // Leverage that a contract will use on its orders
    pub leverage: String,
    // Number of orders to place between the head and the tail
    pub order_density: String,
    // A constant between 0..1 that will be used to control the sensitivity of the reservation_price from mid_price
    pub reservation_price_sensitivity_ratio: String,
    // A constant between 0..1 that will be used to control the sensitivity of the spread around the reservation_price
    pub reservation_spread_sensitivity_ratio: String,
    // A constant between 0..1 that will be used to determine what percentage of how much of our total deposited balance we want margined on the book\
    pub max_active_capital_utilization_ratio: String,
    // A constant between 0..1 that serves as a threshold for which we actually want to take action in the new block
    pub head_change_tolerance_ratio: String,
    // A constant between 0..1 that is used to determine how far we want to place our tails from the midprice
    pub mid_price_tail_deviation_ratio: String,
    // A constant between 0..1 that ensures our tail is at least some distance from the head (risk management param)
    pub min_head_to_tail_deviation_ratio: String,
    // CW20 Wasm contract code id
    pub cw20_code_id: String,
    // LP Token Name
    pub lp_name: String,
    // LP Token Symbol
    pub lp_symbol: String,
    // LP Token Decimals
    pub lp_decimals: String,
    // Label for the CW20 Token
    pub cw20_label: String,
    // Custom marketing info for the CW20 Token
    pub cw20_marketing_info: Option<InstantiateMarketingInfo>,
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
    GetMarketId {},
    GetTotalLpSupply {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedGetActionResponse {
    pub msgs: Vec<ExchangeMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketIdResponse {
    pub market_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalSupplyResponse {
    pub total_supply: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}
