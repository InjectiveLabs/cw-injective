use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Grant {
    authorization: String,
    expiration: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GrantAuthorization {
    pub granter: String,
    pub grantee: String,
    pub authorization: String,
    pub expiration: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PageResponse {
    pub next_key: Option<Vec<u8>>,
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GrantsResponse {
    pub grants: Option<Vec<Grant>>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GranteeGrantsResponse {
    pub grants: Option<Vec<GrantAuthorization>>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GranterGrantsResponse {
    pub grants: Option<Vec<GrantAuthorization>>,
    pub pagination: Option<PageResponse>,
}
