use crate::MarketId;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Deposit is data format for the subaccount deposit
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Deposit {
    #[serde(default)]
    pub available_balance: FPDecimal,
    #[serde(default)]
    pub total_balance: FPDecimal,
}

/// Volume values divided by type (maker or taker volume)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct VolumeByType {
    pub maker_volume: FPDecimal,
    pub taker_volume: FPDecimal,
}

/// Total volume on a given market
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MarketVolume {
    pub market_id: MarketId,
    pub volume: VolumeByType,
}
