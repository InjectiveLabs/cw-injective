use cosmwasm_std::{Coin, Int64};
use injective_cosmwasm::MarketId;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExchangeParams {
    pub spot_market_instant_listing_fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ParamResponse<T> {
    pub params: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AuthParams {
    pub max_memo_characters: String,
    pub sig_verify_cost_ed25519: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MySpotMarketResponse {
    pub market: Option<MySpotMarket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MySpotMarket {
    pub ticker: String,
    pub base_denom: String,
    pub quote_denom: String,
    pub maker_fee_rate: FPDecimal,
    pub taker_fee_rate: FPDecimal,
    pub relayer_fee_share_rate: FPDecimal,
    pub market_id: MarketId,
    pub status: String,
    pub min_price_tick_size: FPDecimal,
    pub min_quantity_tick_size: FPDecimal,
    pub min_notional: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyVolatilityResponse {
    pub volatility: Option<FPDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyPerpetualMarketInfo {
    pub market_id: MarketId,
    #[serde(default)]
    pub hourly_funding_rate_cap: FPDecimal,
    #[serde(default)]
    pub hourly_interest_rate: FPDecimal,
    #[serde(default)]
    pub next_funding_timestamp: Int64,
    pub funding_interval: Int64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyPerpetualMarketInfoResponse {
    pub info: Option<MyPerpetualMarketInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyDerivativeMarketResponse {
    pub market: Option<MyFullDerivativeMarket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyFullDerivativeMarket {
    pub market: Option<MyDerivativeMarket>,
    pub mark_price: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyDerivativeMarket {
    pub ticker: String,
    pub oracle_base: String,
    pub oracle_quote: String,
    #[serde(default)]
    pub oracle_type: String,
    #[serde(default)]
    pub oracle_scale_factor: u32,
    pub quote_denom: String,
    pub market_id: MarketId,
    pub initial_margin_ratio: FPDecimal,
    pub maintenance_margin_ratio: FPDecimal,
    pub maker_fee_rate: FPDecimal,
    pub taker_fee_rate: FPDecimal,
    #[serde(rename = "isPerpetual", default)]
    pub is_perpetual: bool,
    #[serde(default)]
    pub min_price_tick_size: FPDecimal,
    pub min_quantity_tick_size: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyPerpetualMarketFundingResponse {
    pub state: Option<MyPerpetualMarketFunding>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyPerpetualMarketFunding {
    #[serde(default)]
    pub cumulative_funding: FPDecimal,
    #[serde(default)]
    pub cumulative_price: FPDecimal,
    #[serde(default)]
    pub last_timestamp: Int64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyOraclePriceResponse {
    pub price_pair_state: Option<MyPricePairState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyPricePairState {
    #[serde(default)]
    pub pair_price: FPDecimal,
    #[serde(default)]
    pub base_price: FPDecimal,
    #[serde(default)]
    pub quote_price: FPDecimal,
    #[serde(default)]
    pub base_cumulative_price: FPDecimal,
    #[serde(default)]
    pub quote_cumulative_price: FPDecimal,
    #[serde(default)]
    pub base_timestamp: Int64,
    #[serde(default)]
    pub quote_timestamp: Int64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MyPythPriceResponse {
    pub price_state: Option<PythPriceState>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PythPriceState {
    #[serde(default)]
    pub price_id: String,
    #[serde(default)]
    pub ema_price: FPDecimal,
    #[serde(default)]
    pub ema_conf: FPDecimal,
    #[serde(default)]
    pub conf: FPDecimal,
    #[serde(default)]
    pub publish_time: Int64,
    pub price_state: PriceState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PriceState {
    #[serde(default)]
    pub price: FPDecimal,
    #[serde(default)]
    pub cumulative_price: FPDecimal,
    #[serde(default)]
    pub timestamp: Int64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CosmosAuthQueryAccountsResponse {
    pub account: AuthAccount,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AuthAccount {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub base_account: BaseAccount,
    pub code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BaseAccount {
    pub address: String,
    pub pub_key: Option<String>,
    pub account_number: String,
    pub sequence: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BankParams {
    pub send_enabled: Vec<SendEnabled>,
    pub default_send_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SendEnabled {
    pub denom: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryBalanceResponse {
    pub balance: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QuerySupplyOffResponse {
    pub amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryDenomMetadataResponse {
    pub metadatas: Vec<Metadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub uri_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u32,
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    // Define fields based on your pagination structure, if any
}
