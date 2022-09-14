use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    from_slice, to_binary, BankQuery, Binary, ContractResult, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError, SystemResult, WasmQuery,
};

use injective_math::FPDecimal;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::oracle::OracleHistoryOptions;
use crate::volatility::TradeHistoryOptions;
use crate::{
    Deposit, DerivativeMarket, DerivativeMarketMidPriceAndTOBResponse, DerivativeMarketResponse, FullDerivativeMarket, InjectiveQuery,
    InjectiveQueryWrapper, MarketVolatilityResponse, OracleInfo, OracleVolatilityResponse, PerpetualMarketFundingResponse,
    PerpetualMarketInfoResponse, SpotMarket, SpotMarketMidPriceAndTOBResponse, SpotMarketResponse, SubaccountDepositResponse,
    SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse, TraderDerivativeOrdersResponse, TraderSpotOrdersResponse,
};

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new();

    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
        custom_query_type: PhantomData::default(),
    }
}

fn default_subaccount_deposit_response_handler() -> QuerierResult {
    let response = SubaccountDepositResponse {
        deposits: Deposit {
            available_balance: FPDecimal::from(100u128),
            total_balance: FPDecimal::from(100u128),
        },
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_spot_market_response_handler(market_id: String) -> QuerierResult {
    let response = SpotMarketResponse {
        market: Some(SpotMarket {
            ticker: "INJ/USDT".to_string(),
            base_denom: "INJ".to_string(),
            quote_denom: "USDT".to_string(),
            maker_fee_rate: FPDecimal::from_str("0.001").unwrap(),
            taker_fee_rate: FPDecimal::from_str("0.002").unwrap(),
            relayer_fee_share_rate: FPDecimal::from_str("0.4").unwrap(),
            market_id,
            status: 0,
            min_price_tick_size: FPDecimal::from_str("1000").unwrap(),
            min_quantity_tick_size: FPDecimal::from_str("0.001").unwrap(),
        }),
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_trader_spot_orders_response_handler() -> QuerierResult {
    let response = TraderSpotOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_trader_spot_orders_to_cancel_up_to_amount_response_handler() -> QuerierResult {
    let response = TraderSpotOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_trader_derivative_orders_to_cancel_up_to_amount_response_handler() -> QuerierResult {
    let response = TraderDerivativeOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_derivative_market_response_handler(market_id: String) -> QuerierResult {
    let response = DerivativeMarketResponse {
        market: FullDerivativeMarket {
            market: Some(DerivativeMarket {
                ticker: "ticker".to_string(),
                oracle_base: "oracle_base".to_string(),
                oracle_quote: "oracle_quote".to_string(),
                oracle_type: 1,
                oracle_scale_factor: 1,
                quote_denom: "inj".to_string(),
                market_id,
                initial_margin_ratio: FPDecimal::from_str("0.1").unwrap(),
                maintenance_margin_ratio: FPDecimal::from_str("0.05").unwrap(),
                maker_fee_rate: FPDecimal::from_str("0.001").unwrap(),
                taker_fee_rate: FPDecimal::from_str("0.002").unwrap(),
                isPerpetual: true,
                status: 0,
                min_price_tick_size: FPDecimal::from_str("1000").unwrap(),
                min_quantity_tick_size: FPDecimal::from_str("0.001").unwrap(),
            }),
            info: None,
            mark_price: FPDecimal::one(),
        },
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_subaccount_positions_response_handler() -> QuerierResult {
    todo!()
}

fn default_subaccount_position_in_market_response_handler() -> QuerierResult {
    let response = SubaccountPositionInMarketResponse { state: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_subaccount_effective_position_in_market_response_handler() -> QuerierResult {
    let response = SubaccountEffectivePositionInMarketResponse { state: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_trader_derivative_orders_response_handler() -> QuerierResult {
    let response = TraderDerivativeOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_trader_transient_spot_orders_response_handler() -> QuerierResult {
    let response = TraderSpotOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_trader_transient_derivative_orders_response_handler() -> QuerierResult {
    let response = TraderDerivativeOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_perpetual_market_info_response_handler() -> QuerierResult {
    let response = PerpetualMarketInfoResponse { info: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_perpetual_market_funding_response_handler() -> QuerierResult {
    let response = PerpetualMarketFundingResponse { state: None };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_market_volatility_response_handler() -> QuerierResult {
    let response = MarketVolatilityResponse {
        volatility: Some(FPDecimal::one()),
        history_metadata: None,
        raw_history: None,
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_spot_market_mid_price_and_tob_response_handler() -> QuerierResult {
    let response = SpotMarketMidPriceAndTOBResponse {
        mid_price: Some(FPDecimal::from_str("200000").unwrap()),
        best_bid: None,
        best_ask: None,
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_derivative_market_mid_price_and_tob_response_handler() -> QuerierResult {
    let response = DerivativeMarketMidPriceAndTOBResponse {
        mid_price: Some(FPDecimal::from_str("200000").unwrap()),
        best_bid: None,
        best_ask: None,
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_oracle_volatility_response_handler() -> QuerierResult {
    let response = OracleVolatilityResponse {
        volatility: Some(FPDecimal::one()),
        history_metadata: None,
        raw_history: None,
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

type TraderSpotOrdersToCancelUpToAmountResponseHandler = Option<
    fn(
        market_id: String,
        subaccount_id: String,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> QuerierResult,
>;
type TraderDerivativeOrdersToCancelUpToAmountResponseHandler =
    Option<fn(market_id: String, subaccount_id: String, quote_amount: FPDecimal, strategy: i32, reference_price: Option<FPDecimal>) -> QuerierResult>;
type OracleVolatilityResponseHandler =
    Option<fn(base_info: Option<OracleInfo>, quote_info: Option<OracleInfo>, oracle_history_options: Option<OracleHistoryOptions>) -> QuerierResult>;

pub struct WasmMockQuerier {
    pub smart_query_handler: Option<fn(contract_addr: &str, msg: &Binary) -> QuerierResult>,
    pub bank_query_handler: Option<fn(query: &BankQuery) -> QuerierResult>,
    pub subaccount_deposit_response_handler: Option<fn(subaccount_id: String, denom: String) -> QuerierResult>,
    pub spot_market_response_handler: Option<fn(market_id: String) -> QuerierResult>,
    pub trader_spot_orders_response_handler: Option<fn(market_id: String, subaccount_id: String) -> QuerierResult>,
    pub trader_spot_orders_to_cancel_up_to_amount_response_handler: TraderSpotOrdersToCancelUpToAmountResponseHandler,
    pub trader_derivative_orders_to_cancel_up_to_amount_response_handler: TraderDerivativeOrdersToCancelUpToAmountResponseHandler,
    pub derivative_market_response_handler: Option<fn(market_id: String) -> QuerierResult>,
    pub subaccount_positions_response_handler: Option<fn(subaccount_id: String) -> QuerierResult>,
    pub subaccount_position_in_market_response_handler: Option<fn(market_id: String, subaccount_id: String) -> QuerierResult>,
    pub subaccount_effective_position_in_market_response_handler: Option<fn(market_id: String, subaccount_id: String) -> QuerierResult>,
    pub trader_derivative_orders_response_handler: Option<fn(market_id: String, subaccount_id: String) -> QuerierResult>,
    pub trader_transient_spot_orders_response_handler: Option<fn(market_id: String, subaccount_id: String) -> QuerierResult>,
    pub trader_transient_derivative_orders_response_handler: Option<fn(market_id: String, subaccount_id: String) -> QuerierResult>,
    pub perpetual_market_info_response_handler: Option<fn(market_id: String) -> QuerierResult>,
    pub perpetual_market_funding_response_handler: Option<fn(market_id: String) -> QuerierResult>,
    pub market_volatility_response_handler: Option<fn(market_id: String, trade_history_options: TradeHistoryOptions) -> QuerierResult>,
    pub spot_market_mid_price_and_tob_response_handler: Option<fn(market_id: String) -> QuerierResult>,
    pub derivative_market_mid_price_and_tob_response_handler: Option<fn(market_id: String) -> QuerierResult>,
    pub oracle_volatility_response_handler: OracleVolatilityResponseHandler,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<InjectiveQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };

        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<InjectiveQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => match self.smart_query_handler {
                Some(handler) => handler(contract_addr, msg),
                None => panic!("Unknown smart query"),
            },
            QueryRequest::Custom(query) => match query.query_data.clone() {
                InjectiveQuery::SubaccountDeposit { subaccount_id, denom } => match self.subaccount_deposit_response_handler {
                    Some(handler) => handler(subaccount_id, denom),
                    None => default_subaccount_deposit_response_handler(),
                },
                InjectiveQuery::SpotMarket { market_id } => match self.spot_market_response_handler {
                    Some(handler) => handler(market_id),
                    None => default_spot_market_response_handler(market_id),
                },
                InjectiveQuery::TraderSpotOrders { market_id, subaccount_id } => match self.trader_spot_orders_response_handler {
                    Some(handler) => handler(market_id, subaccount_id),
                    None => default_trader_spot_orders_response_handler(),
                },
                InjectiveQuery::TraderSpotOrdersToCancelUpToAmount {
                    market_id,
                    subaccount_id,
                    base_amount,
                    quote_amount,
                    strategy,
                    reference_price,
                } => match self.trader_spot_orders_to_cancel_up_to_amount_response_handler {
                    Some(handler) => handler(market_id, subaccount_id, base_amount, quote_amount, strategy, reference_price),
                    None => default_trader_spot_orders_to_cancel_up_to_amount_response_handler(),
                },
                InjectiveQuery::TraderDerivativeOrdersToCancelUpToAmount {
                    market_id,
                    subaccount_id,
                    quote_amount,
                    strategy,
                    reference_price,
                } => match self.trader_derivative_orders_to_cancel_up_to_amount_response_handler {
                    Some(handler) => handler(market_id, subaccount_id, quote_amount, strategy, reference_price),
                    None => default_trader_derivative_orders_to_cancel_up_to_amount_response_handler(),
                },
                InjectiveQuery::DerivativeMarket { market_id } => match self.derivative_market_response_handler {
                    Some(handler) => handler(market_id),
                    None => default_derivative_market_response_handler(market_id),
                },
                InjectiveQuery::SubaccountPositions { subaccount_id } => match self.subaccount_positions_response_handler {
                    Some(handler) => handler(subaccount_id),
                    None => default_subaccount_positions_response_handler(),
                },
                InjectiveQuery::SubaccountPositionInMarket { market_id, subaccount_id } => {
                    match self.subaccount_position_in_market_response_handler {
                        Some(handler) => handler(market_id, subaccount_id),
                        None => default_subaccount_position_in_market_response_handler(),
                    }
                }
                InjectiveQuery::SubaccountEffectivePositionInMarket { market_id, subaccount_id } => {
                    match self.subaccount_effective_position_in_market_response_handler {
                        Some(handler) => handler(market_id, subaccount_id),
                        None => default_subaccount_effective_position_in_market_response_handler(),
                    }
                }
                InjectiveQuery::TraderDerivativeOrders { market_id, subaccount_id } => match self.trader_derivative_orders_response_handler {
                    Some(handler) => handler(market_id, subaccount_id),
                    None => default_trader_derivative_orders_response_handler(),
                },
                InjectiveQuery::TraderTransientSpotOrders { market_id, subaccount_id } => match self.trader_transient_spot_orders_response_handler {
                    Some(handler) => handler(market_id, subaccount_id),
                    None => default_trader_transient_spot_orders_response_handler(),
                },
                InjectiveQuery::TraderTransientDerivativeOrders { market_id, subaccount_id } => {
                    match self.trader_transient_derivative_orders_response_handler {
                        Some(handler) => handler(market_id, subaccount_id),
                        None => default_trader_transient_derivative_orders_response_handler(),
                    }
                }
                InjectiveQuery::PerpetualMarketInfo { market_id } => match self.perpetual_market_info_response_handler {
                    Some(handler) => handler(market_id),
                    None => default_perpetual_market_info_response_handler(),
                },
                InjectiveQuery::PerpetualMarketFunding { market_id } => match self.perpetual_market_funding_response_handler {
                    Some(handler) => handler(market_id),
                    None => default_perpetual_market_funding_response_handler(),
                },
                InjectiveQuery::MarketVolatility {
                    market_id,
                    trade_history_options,
                } => match self.market_volatility_response_handler {
                    Some(handler) => handler(market_id, trade_history_options),
                    None => default_market_volatility_response_handler(),
                },
                InjectiveQuery::SpotMarketMidPriceAndTob { market_id } => match self.spot_market_mid_price_and_tob_response_handler {
                    Some(handler) => handler(market_id),
                    None => default_spot_market_mid_price_and_tob_response_handler(),
                },
                InjectiveQuery::DerivativeMarketMidPriceAndTob { market_id } => match self.derivative_market_mid_price_and_tob_response_handler {
                    Some(handler) => handler(market_id),
                    None => default_derivative_market_mid_price_and_tob_response_handler(),
                },
                InjectiveQuery::OracleVolatility {
                    base_info,
                    quote_info,
                    oracle_history_options,
                } => match self.oracle_volatility_response_handler {
                    Some(handler) => handler(base_info, quote_info, oracle_history_options),
                    None => default_oracle_volatility_response_handler(),
                },
            },
            QueryRequest::Bank(query) => match self.bank_query_handler {
                Some(handler) => handler(query),
                None => panic!("Unknown bank query"),
            },
            _ => panic!("Unknown query"),
        }
    }
}

impl Default for WasmMockQuerier {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmMockQuerier {
    pub fn new() -> Self {
        WasmMockQuerier {
            smart_query_handler: None,
            bank_query_handler: None,
            subaccount_deposit_response_handler: None,
            spot_market_response_handler: None,
            trader_spot_orders_response_handler: None,
            trader_spot_orders_to_cancel_up_to_amount_response_handler: None,
            trader_derivative_orders_to_cancel_up_to_amount_response_handler: None,
            derivative_market_response_handler: None,
            subaccount_positions_response_handler: None,
            subaccount_position_in_market_response_handler: None,
            subaccount_effective_position_in_market_response_handler: None,
            trader_derivative_orders_response_handler: None,
            trader_transient_spot_orders_response_handler: None,
            trader_transient_derivative_orders_response_handler: None,
            perpetual_market_info_response_handler: None,
            perpetual_market_funding_response_handler: None,
            market_volatility_response_handler: None,
            spot_market_mid_price_and_tob_response_handler: None,
            derivative_market_mid_price_and_tob_response_handler: None,
            oracle_volatility_response_handler: None,
        }
    }
}
