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
    pub market_id: String,
    pub subaccount_id: String,
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
    pub market_id: String,
    pub source_subaccount_id: String,
    pub destination_subaccount_id: String,
    pub quantity: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PrivilegedAction {
    pub synthetic_trade: Option<SyntheticTradeAction>,
    pub position_transfer: Option<PositionTransferAction>,
}
