use schemars::JsonSchema;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[repr(u8)]
pub enum CancellationStrategy {
    UnspecifiedOrder = 0,
    FromWorstToBest = 1,
    FromBestToWorst = 2,
}
