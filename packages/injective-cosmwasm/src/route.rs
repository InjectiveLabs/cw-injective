use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// InjectiveRoute is enum type to represent injective query route path
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InjectiveRoute {
    // Auction,
    Exchange,
    // Insurance,
    TokenFactory,
    Oracle,
    // Peggy,
}
