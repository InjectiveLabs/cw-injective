use crate::exchange::types::{MarketId, SubaccountId};
use cosmwasm_std::Coin;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema)]
pub struct FPDecimalCoin {
    pub denom: String,
    pub amount: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SyntheticTrade {
    pub market_id: MarketId,
    pub subaccount_id: SubaccountId,
    pub is_buy: bool,
    pub quantity: FPDecimal,
    pub price: FPDecimal,
    pub margin: FPDecimal,
    pub required_funds: Option<FPDecimalCoin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SyntheticTradeAction {
    pub user_trades: Vec<SyntheticTrade>,
    pub contract_trades: Vec<SyntheticTrade>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PositionTransferAction {
    pub market_id: MarketId,
    pub source_subaccount_id: SubaccountId,
    pub destination_subaccount_id: SubaccountId,
    pub quantity: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PrivilegedAction {
    pub synthetic_trade: Option<SyntheticTradeAction>,
    pub position_transfer: Option<PositionTransferAction>,
}

pub fn coins_to_string(coins: Vec<Coin>) -> String {
    coins.into_iter().map(|coin| format!("{}", coin)).collect::<Vec<String>>().join(", ")
}
