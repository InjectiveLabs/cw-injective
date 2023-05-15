use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_cosmwasm::{
    InjectiveQuerier, InjectiveQueryWrapper, MarketId, OrderSide, OrderType, PriceLevel,
};
use injective_math::FPDecimal;

pub struct ExecutionPrice {
    pub worst_price: FPDecimal,
    pub average_price: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct FPCoin {
    pub amount: FPDecimal,
    pub denom: String,
}

impl From<FPCoin> for Coin {
    fn from(value: FPCoin) -> Self {
        Coin::new(value.amount.into(), value.denom)
    }
}

impl From<Coin> for FPCoin {
    fn from(value: Coin) -> Self {
        FPCoin {
            amount: value.amount.into(),
            denom: value.denom,
        }
    }
}

pub struct StepExecutionEstimate {
    pub worst_price: FPDecimal,
    pub result_denom: String,
    pub result_quantity: FPDecimal,
    pub is_buy_order: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CurrentSwapOperation {
    // whole swap operation
    pub sender_address: Addr,
    pub swap_steps: Vec<MarketId>,
    pub min_target_quantity: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CurrentSwapStep {
    // current step
    pub step_idx: usize,
    pub current_balance: FPCoin,
    pub step_target_denom: String,
}
