use crate::state::State;
use crate::utils::{div_dec, round_to_min_ticker};
use cosmwasm_std::Decimal256 as Decimal;
use injective_bindings::{
    Deposit as QueriedDeposit, DerivativeMarket as QueriedDerivativeMarket, PerpetualMarketFunding as QueriedPerpetualMarketFunding,
    PerpetualMarketInfo as QueriedPerpetualMarketInfo, Position as QueriedPosition,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub fn from_query(queried_market: QueriedDerivativeMarket) -> DerivativeMarket {
        DerivativeMarket {
            ticker: queried_market.ticker,
            oracle_base: queried_market.oracle_base,
            oracle_quote: queried_market.oracle_quote,
            oracle_type: queried_market.oracle_type,
            oracle_scale_factor: queried_market.oracle_scale_factor,
            quote_denom: queried_market.quote_denom,
            market_id: queried_market.market_id,
            initial_margin_ratio: queried_market.initial_margin_ratio,
            maintenance_margin_ratio: queried_market.maintenance_margin_ratio,
            maker_fee_rate: queried_market.maker_fee_rate,
            taker_fee_rate: queried_market.taker_fee_rate,
            isPerpetual: queried_market.isPerpetual,
            status: queried_market.status,
            min_price_tick_size: queried_market.min_price_tick_size,
            min_quantity_tick_size: queried_market.min_quantity_tick_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketInfo {
    pub market_id: String,
    pub hourly_funding_rate_cap: Decimal,
    pub hourly_interest_rate: Decimal,
    pub next_funding_timestamp: i64,
    pub funding_interval: i64,
}

impl PerpetualMarketInfo {
    pub fn from_query(queried_perpetual_market_info: QueriedPerpetualMarketInfo) -> PerpetualMarketInfo {
        PerpetualMarketInfo {
            market_id: queried_perpetual_market_info.market_id,
            hourly_funding_rate_cap: queried_perpetual_market_info.hourly_funding_rate_cap,
            hourly_interest_rate: queried_perpetual_market_info.hourly_interest_rate,
            next_funding_timestamp: queried_perpetual_market_info.next_funding_timestamp,
            funding_interval: queried_perpetual_market_info.funding_interval,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PerpetualMarketFunding {
    pub cumulative_funding: Decimal,
    pub cumulative_price: Decimal,
    pub last_timestamp: i64,
}

impl PerpetualMarketFunding {
    pub fn from_query(queried_perpetual_market_funding: QueriedPerpetualMarketFunding) -> PerpetualMarketFunding {
        PerpetualMarketFunding {
            cumulative_funding: queried_perpetual_market_funding.cumulative_funding,
            cumulative_price: queried_perpetual_market_funding.cumulative_price,
            last_timestamp: queried_perpetual_market_funding.last_timestamp,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderInfo {
    pub subaccount_id: String,
    pub fee_recipient: String,
    pub price: Decimal,
    pub quantity: Decimal,
}
impl OrderInfo {
    pub fn new(subaccount_id: String, fee_recipient: String, price: Decimal, quantity: Decimal) -> OrderInfo {
        OrderInfo {
            subaccount_id,
            fee_recipient,
            price,
            quantity,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DerivativeLimitOrder {
    pub order_info: OrderInfo,
    pub order_type: i32,
    pub margin: Decimal,
    pub fillable: Decimal,
    pub trigger_price: Option<Decimal>,
    pub order_hash: String,
}
impl DerivativeLimitOrder {
    pub fn new(
        margin: Decimal,
        fillable: Decimal,
        order_hash: String,
        trigger_price: Option<Decimal>,
        order_type: i32,
        order_info: OrderInfo,
    ) -> DerivativeLimitOrder {
        DerivativeLimitOrder {
            margin,
            fillable,
            order_hash,
            trigger_price,
            order_type,
            order_info,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub available_balance: Decimal,
    pub total_balance: Decimal,
}

impl Deposit {
    pub fn from_query(queried_deposit: QueriedDeposit) -> Deposit {
        Deposit {
            available_balance: queried_deposit.available_balance,
            total_balance: queried_deposit.total_balance,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Position {
    pub is_long: bool,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub margin: Decimal,
    pub cumulative_funding_entry: Decimal,
}
impl Position {
    pub fn from_query(queried_position: QueriedPosition) -> Position {
        Position {
            is_long: queried_position.isLong,
            quantity: queried_position.quantity,
            margin: queried_position.margin,
            entry_price: queried_position.entry_price,
            cumulative_funding_entry: queried_position.cumulative_funding_entry,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderData {
    pub market_id: String,
    pub subaccount_id: String,
    pub order_hash: String,
}

impl OrderData {
    pub fn new(order_hash: String, state: &State, market: &DerivativeMarket) -> OrderData {
        OrderData {
            market_id: market.market_id.clone(),
            subaccount_id: state.subaccount_id.clone(),
            order_hash,
        }
    }
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
    pub margin: Decimal,
    pub trigger_price: Option<String>,
}
impl DerivativeOrder {
    pub fn new(state: &State, price: Decimal, qty: Decimal, is_buy: bool, is_reduce_only: bool, market: &DerivativeMarket) -> DerivativeOrder {
        let margin = if is_reduce_only {
            Decimal::zero()
        } else {
            let margin = div_dec(price * qty, state.leverage);
            margin
        };
        DerivativeOrder {
            market_id: state.market_id.clone(),
            order_info: OrderInfo {
                subaccount_id: state.subaccount_id.clone(),
                fee_recipient: state.fee_recipient.clone(),
                price: round_to_min_ticker(price, market.min_price_tick_size),
                quantity: round_to_min_ticker(qty, market.min_quantity_tick_size),
            },
            order_type: if is_buy { 1 } else { 2 },
            margin: round_to_min_ticker(margin, market.min_quantity_tick_size),
            trigger_price: None,
        }
    }
    pub fn is_reduce_only(&self) -> bool {
        self.margin.is_zero()
    }
    pub fn get_price(&self) -> Decimal {
        self.order_info.price
    }
    pub fn get_qty(&self) -> Decimal {
        self.order_info.quantity
    }
    pub fn get_val(&self) -> Decimal {
        self.get_price() * self.get_qty()
    }
    pub fn get_margin(&self) -> Decimal {
        self.margin
    }
    pub fn non_reduce_only_is_invalid(&self) -> bool {
        self.get_margin().is_zero() || self.get_price().is_zero() || self.get_qty().is_zero()
    }
    pub fn reduce_only_is_invalid(&self) -> bool {
        self.get_price().is_zero() || self.get_qty().is_zero()
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
