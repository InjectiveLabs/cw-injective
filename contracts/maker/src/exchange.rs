use crate::state::State;
use crate::utils::round_to_precision;
use cosmwasm_std::{Decimal256 as Decimal, StdError, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeMarket {
    pub ticker: String,
    pub oracle_base: String,
    pub oracle_quote: String,
    pub oracle_type: i32,
    pub oracle_scale_factor: u32,
    pub quote_denom: String,
    pub market_id: String,
    pub initial_margin_ratio: String,
    pub maintenance_margin_ratio: String,
    pub maker_fee_rate: String,
    pub taker_fee_rate: String,
    pub isPerpetual: bool,
    pub status: i32,
    pub min_price_tick_size: String,
    pub min_quantity_tick_size: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedDerivativeMarket {
    pub ticker: String,
    pub oracle_base: String,
    pub oracle_quote: String,
    pub oracle_type: i32,
    pub oracle_scale_factor: u32,
    pub quote_denom: String,
    pub market_id: String,
    pub initial_margin_ratio: Decimal,
    pub maintenance_margin_ratio: Decimal,
    pub maker_fee_rate: Decimal,
    pub taker_fee_rate: Decimal,
    pub isPerpetual: bool,
    pub status: i32,
    pub min_price_tick_size: Decimal,
    pub min_quantity_tick_size: Decimal,
}

impl DerivativeMarket {
    pub fn wrap(&self) -> Result<WrappedDerivativeMarket, StdError> {
        Ok(WrappedDerivativeMarket {
            ticker: self.ticker.to_string(),
            oracle_base: self.oracle_base.to_string(),
            oracle_quote: self.oracle_quote.to_string(),
            oracle_type: self.oracle_type,
            oracle_scale_factor: self.oracle_scale_factor,
            quote_denom: self.quote_denom.to_string(),
            market_id: self.market_id.to_string(),
            initial_margin_ratio: Decimal::from_str(&self.initial_margin_ratio).unwrap(),
            maintenance_margin_ratio: Decimal::from_str(&self.maintenance_margin_ratio).unwrap(),
            maker_fee_rate: Decimal::from_str(&self.maker_fee_rate).unwrap(),
            taker_fee_rate: Decimal::from_str(&self.taker_fee_rate).unwrap(),
            isPerpetual: self.isPerpetual,
            status: self.status,
            min_price_tick_size: Decimal::from_str(&self.min_price_tick_size).unwrap(),
            min_quantity_tick_size: Decimal::from_str(&self.min_quantity_tick_size).unwrap(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketInfo {
    pub market_id: String,
    pub hourly_funding_rate_cap: String,
    pub hourly_interest_rate: String,
    pub next_funding_timestamp: i64,
    pub funding_interval: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedPerpetualMarketInfo {
    pub market_id: String,
    pub hourly_funding_rate_cap: Decimal,
    pub hourly_interest_rate: Decimal,
    pub next_funding_timestamp: i64,
    pub funding_interval: i64,
}

impl PerpetualMarketInfo {
    pub fn wrap(&self) -> Result<WrappedPerpetualMarketInfo, StdError> {
        Ok(WrappedPerpetualMarketInfo {
            market_id: self.market_id.to_string(),
            hourly_funding_rate_cap: Decimal::from_str(&self.hourly_funding_rate_cap).unwrap(),
            hourly_interest_rate: Decimal::from_str(&self.hourly_interest_rate).unwrap(),
            next_funding_timestamp: self.next_funding_timestamp,
            funding_interval: self.funding_interval,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketFunding {
    pub cumulative_funding: String,
    pub cumulative_price: String,
    pub last_timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedPerpetualMarketFunding {
    pub cumulative_funding: Decimal,
    pub cumulative_price: Decimal,
    pub last_timestamp: i64,
}

impl PerpetualMarketFunding {
    pub fn wrap(&self) -> Result<WrappedPerpetualMarketFunding, StdError> {
        Ok(WrappedPerpetualMarketFunding {
            cumulative_funding: Decimal::from_str(&self.cumulative_funding).unwrap(),
            cumulative_price: Decimal::from_str(&self.cumulative_price).unwrap(),
            last_timestamp: self.last_timestamp,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderInfo {
    pub subaccount_id: String,
    pub fee_recipient: String,
    pub price: String,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedOrderInfo {
    pub subaccount_id: String,
    pub fee_recipient: String,
    pub price: Decimal,
    pub quantity: Decimal,
}

impl OrderInfo {
    pub fn wrap(&self) -> Result<WrappedOrderInfo, StdError> {
        Ok(WrappedOrderInfo {
            subaccount_id: self.subaccount_id.to_string(),
            fee_recipient: self.fee_recipient.to_string(),
            price: Decimal::from_str(&self.price).unwrap(),
            quantity: Decimal::from_str(&self.quantity).unwrap(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: String,
    pub fillable: String,
    pub trigger_price: Option<String>,
    pub order_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedDerivativeLimitOrder {
    pub order_info: WrappedOrderInfo,
    pub order_type: i32,
    pub margin: Decimal,
    pub fillable: Decimal,
    pub trigger_price: Option<Decimal>,
    pub order_hash: Vec<u8>,
}

impl DerivativeLimitOrder {
    pub fn wrap(&self) -> Result<WrappedDerivativeLimitOrder, StdError> {
        Ok(WrappedDerivativeLimitOrder {
            order_info: self.order_info.wrap()?,
            order_type: self.order_type,
            margin: Decimal::from_str(&self.margin).unwrap(),
            fillable: Decimal::from_str(&self.fillable).unwrap(),
            trigger_price: None,
            order_hash: self.order_hash.clone(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub available_balance: String,
    pub total_balance: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedDeposit {
    pub available_balance: Decimal,
    pub total_balance: Decimal,
}

impl Deposit {
    pub fn wrap(&self) -> Result<WrappedDeposit, StdError> {
        Ok(WrappedDeposit {
            available_balance: Decimal::from_str(&self.available_balance).unwrap(),
            total_balance: Decimal::from_str(&self.total_balance).unwrap(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
    pub is_long: bool,
    pub quantity: String,
    pub entry_price: String,
    pub margin: String,
    pub cumulative_funding_entry: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WrappedPosition {
    pub is_long: bool,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub margin: Decimal,
    pub cumulative_funding_entry: Decimal,
}

impl Position {
    pub fn wrap(&self) -> Result<WrappedPosition, StdError> {
        Ok(WrappedPosition {
            is_long: self.is_long,
            quantity: Decimal::from_str(&self.quantity).unwrap(),
            entry_price: Decimal::from_str(&self.entry_price).unwrap(),
            margin: Decimal::from_str(&self.margin).unwrap(),
            cumulative_funding_entry: Decimal::from_str(&self.cumulative_funding_entry).unwrap(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderData {
    pub market_id: String,
    pub subaccount_id: String,
    pub order_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub trigger_price: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeOrder {
    pub market_id: String,
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: String,
    pub trigger_price: Option<String>,
}
impl DerivativeOrder {
    pub fn new(state: &State, price: Decimal, quantity: Decimal, is_buy: bool, margin: Decimal) -> DerivativeOrder {
        DerivativeOrder {
            market_id: state.market_id.clone(),
            order_info: OrderInfo {
                subaccount_id: state.sub_account.clone(),
                fee_recipient: state.fee_recipient.clone(),
                price: round_to_precision(price, Uint256::from_str("1").unwrap()).to_string(),
                quantity: round_to_precision(quantity, state.base_precision_shift).to_string(),
            },
            order_type: if is_buy { 1 } else { 2 },
            margin: round_to_precision(margin, Uint256::from_str("1").unwrap()).to_string(),
            trigger_price: None,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        Decimal::from_str(&self.margin).unwrap().is_zero()
    }
    pub fn get_price(&self) -> Decimal {
        Decimal::from_str(&self.order_info.price).unwrap()
    }
    pub fn get_qty(&self) -> Decimal {
        Decimal::from_str(&self.order_info.quantity).unwrap()
    }
    pub fn get_val(&self) -> Decimal {
        self.get_price() * self.get_qty()
    }
    pub fn get_margin(&self) -> Decimal {
        Decimal::from_str(&self.margin).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MsgBatchUpdateOrders {
    pub sender: String,
    pub subaccount_id: String,
    pub spot_market_ids_to_cancel_all: Vec<String>,
    pub derivative_market_ids_to_cancel_all: Vec<String>,
    pub spot_orders_to_cancel: Vec<OrderData>,
    pub derivative_orders_to_cancel: Vec<OrderData>,
    pub spot_orders_to_create: Vec<SpotOrder>,
    pub derivative_orders_to_create: Vec<DerivativeOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MsgCreateDerivativeMarketOrder {
    pub sender: String,
    pub order: DerivativeOrder,
}

// TODO: fill in the rest
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExchangeMsg {
    MsgCreateDerivativeMarketOrder,
    BatchUpdateOrders(MsgBatchUpdateOrders),
    // MsgBatchUpdateOrders {
    //     sender: String,
    //     subaccount_id: String,
    //     spot_market_ids_to_cancel_all: Vec<String>,
    //     derivative_market_ids_to_cancel_all: Vec<String>,
    //     spot_orders_to_cancel: Vec<OrderData>,
    //     derivative_orders_to_cancel: Vec<OrderData>,
    //     spot_orders_to_create: Vec<SpotOrder>,
    //     derivative_orders_to_create: Vec<DerivativeOrder>,
    // },
}
