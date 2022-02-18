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
    // The contract's delegated subaccount
    pub is_deriv: bool,
    // Whether the contract will be operating on a derivative market
    pub leverage: String,
    // Leverage that a contract will use on its orders
    pub order_density: String,
    // Number of orders to place between the head and the tail
    pub reservation_param: String,
    // A constant between 0..1 that will be used to control the sensitivity of the reservation_price
    pub spread_param: String,
    // A constant between 0..1 that will be used to control the sensitivity of the spread around the mid_price
    pub active_capital: String,
    // A constant between 0..1 that will be used to determine how much of our capital we want resting on the book
    pub head_chg_tol_bp: String,
    // A threshold for which we actually want to take action in BP (if new head is more than x dist away from old head)
    pub tail_dist_from_mid_bp: String,
    // The distance in BP from the mid_price that we want to place our tails
    pub min_tail_dist_bp: String,
    // The minimum distance in BP from the head that we want our tail (risk management param)
    pub max_market_data_delay: String,
    // The maximum time we are willing to tolerate since the last market data update for which the contract will behave expectedly
    pub decimal_shift: String,
    // 10^(number of decimals of the quote currency)
    pub base_precision_shift: String,
    // 10^(decimal precision of base quantity respective of the market)
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
    GetActionStateChanging {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    // GetAction {
    //     market: DerivativeMarket,
    //     perpetual_market_info: Option<PerpetualMarketInfo>,
    //     perpetual_market_funding: Option<PerpetualMarketFunding>,
    //     open_orders: Vec<DerivativeLimitOrder>,
    //     deposit: Deposit,
    //     position: Option<Position>,
    //     oracle_price: String,
    //     volatility: String,
    //     mid_price: String,
    // },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedGetActionResponse {
    pub msgs: Vec<ExchangeMsg>,
}
