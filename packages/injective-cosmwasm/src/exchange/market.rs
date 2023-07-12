use schemars::JsonSchema;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Debug, PartialEq, Eq, JsonSchema, Copy)]
#[repr(i32)]
pub enum MarketStatus {
    #[default]
    Unspecified = 0,
    Active = 1,
    Paused = 2,
    Demolished = 3,
    Expired = 4,
}
