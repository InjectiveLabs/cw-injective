use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use injective_cosmwasm::{MarketId, SubaccountId};

pub const MSG_CREATE_SPOT_LIMIT_ORDER_ENDPOINT: &str = "/injective.exchange.v1beta1.MsgCreateSpotLimitOrder";
pub const MSG_CREATE_DERIVATIVE_LIMIT_ORDER_ENDPOINT: &str = "/injective.exchange.v1beta1.MsgCreateDerivativeLimitOrder";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TestTraderTransientSpotOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        price: String,
        quantity: String,
    },
    TestTraderTransientDerivativeOrders {
        market_id: MarketId,
        subaccount_id: SubaccountId,
        price: String,
        quantity: String,
        margin: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryStargate { path: String, query_request: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryStargateResponse {
    pub value: String,
}
