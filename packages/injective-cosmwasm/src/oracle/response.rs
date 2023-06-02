use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::oracle::types::{PricePairState, PythPriceState};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OraclePriceResponse {
    pub price_pair_state: Option<PricePairState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PythPriceResponse {
    pub price_state: Option<PythPriceState>,
}
