use std::marker::PhantomData;
use std::str::FromStr;

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    from_slice, to_binary, AllBalanceResponse, BalanceResponse, BankQuery, Binary, Coin, ContractResult, OwnedDeps, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};

use injective_math::FPDecimal;

use crate::oracle::{OracleHistoryOptions, OracleType};
use crate::query::{TokenFactoryCreateDenomFeeResponse, TokenFactoryDenomSupplyResponse};
use crate::volatility::TradeHistoryOptions;
use crate::{
    Deposit, DerivativeMarket, DerivativeMarketMidPriceAndTOBResponse, DerivativeMarketResponse, FullDerivativeMarket, InjectiveQuery,
    InjectiveQueryWrapper, MarketVolatilityResponse, OracleInfo, OracleVolatilityResponse, PerpetualMarketFundingResponse,
    PerpetualMarketInfoResponse, SpotMarket, SpotMarketMidPriceAndTOBResponse, SpotMarketResponse, SubaccountDepositResponse,
    SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse, TraderDerivativeOrdersResponse, TraderSpotOrdersResponse,
};
use crate::{MarketId, SubaccountId};

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InjectiveQueryWrapper> {
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

fn default_spot_market_response_handler(market_id: MarketId) -> QuerierResult {
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

fn default_derivative_market_response_handler(market_id: MarketId) -> QuerierResult {
    let response = DerivativeMarketResponse {
        market: FullDerivativeMarket {
            market: Some(DerivativeMarket {
                ticker: "ticker".to_string(),
                oracle_base: "oracle_base".to_string(),
                oracle_quote: "oracle_quote".to_string(),
                oracle_type: OracleType::Band,
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

fn default_token_factory_denom_total_supply_handler() -> QuerierResult {
    let response = TokenFactoryDenomSupplyResponse {
        total_supply: Uint128::from(1000u128),
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_token_factory_denom_creation_fee_handler() -> QuerierResult {
    let response = TokenFactoryCreateDenomFeeResponse {
        fee: vec![Coin::new(10, "inj")],
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_balance_query_handler(denom: impl Into<String>) -> QuerierResult {
    let response = BalanceResponse {
        amount: Coin::new(1000000000000000, denom),
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_all_balances_query_handler() -> QuerierResult {
    let response = AllBalanceResponse {
        amount: vec![Coin::new(1000000000000000, "inj")],
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

pub trait HandlesSmartQuery {
    fn handle(&self, contract_addr: &str, msg: &Binary) -> QuerierResult;
}

pub trait HandlesBankQuery {
    fn handle(&self, query: &BankQuery) -> QuerierResult;
}

pub trait HandlesTraderSpotOrdersToCancelUpToAmountQuery {
    fn handle(
        &self,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        base_amount: FPDecimal,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> QuerierResult;
}

pub trait HandlesTraderDerivativeOrdersToCancelUpToAmountQuery {
    fn handle(
        &self,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        quote_amount: FPDecimal,
        strategy: i32,
        reference_price: Option<FPDecimal>,
    ) -> QuerierResult;
}

pub trait HandlesMarketIdQuery {
    fn handle(&self, market_id: MarketId) -> QuerierResult;
}

pub trait HandlesSubaccountIdQuery {
    fn handle(&self, subaccount_id: SubaccountId) -> QuerierResult;
}

pub trait HandlesMarketAndSubaccountQuery {
    fn handle(&self, market_id: MarketId, subaccount_id: SubaccountId) -> QuerierResult;
}

pub trait HandlesSubaccountAndDenomQuery {
    fn handle(&self, subaccount_id: SubaccountId, denom: String) -> QuerierResult;
}

pub trait HandlesOracleVolatilityQuery {
    fn handle(
        &self,
        base_info: Option<OracleInfo>,
        quote_info: Option<OracleInfo>,
        oracle_history_options: Option<OracleHistoryOptions>,
    ) -> QuerierResult;
}

pub trait HandlesOraclePriceQuery {
    fn handle(&self, oracle_type: OracleType, base: String, quote: String) -> QuerierResult;
}

pub trait HandlesMarketVolatilityQuery {
    fn handle(&self, market_id: MarketId, trade_history_options: TradeHistoryOptions) -> QuerierResult;
}

pub trait HandlesDenomSupplyQuery {
    fn handle(&self, denom: String) -> QuerierResult;
}

pub trait HandlesFeeQuery {
    fn handle(&self) -> QuerierResult;
}

pub trait HandlesBankBalanceQuery {
    fn handle(&self, address: String, denom: String) -> QuerierResult;
}

pub trait HandlesBankAllBalancesQuery {
    fn handle(&self, address: String) -> QuerierResult;
}

pub struct WasmMockQuerier {
    pub smart_query_handler: Option<Box<dyn HandlesSmartQuery>>,
    pub bank_query_handler: Option<Box<dyn HandlesBankQuery>>,
    pub subaccount_deposit_response_handler: Option<Box<dyn HandlesSubaccountAndDenomQuery>>,
    pub spot_market_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
    pub trader_spot_orders_response_handler: Option<Box<dyn HandlesMarketAndSubaccountQuery>>,
    pub trader_spot_orders_to_cancel_up_to_amount_response_handler: Option<Box<dyn HandlesTraderSpotOrdersToCancelUpToAmountQuery>>,
    pub trader_derivative_orders_to_cancel_up_to_amount_response_handler: Option<Box<dyn HandlesTraderDerivativeOrdersToCancelUpToAmountQuery>>,
    pub derivative_market_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
    pub subaccount_positions_response_handler: Option<Box<dyn HandlesSubaccountIdQuery>>,
    pub subaccount_position_in_market_response_handler: Option<Box<dyn HandlesMarketAndSubaccountQuery>>,
    pub subaccount_effective_position_in_market_response_handler: Option<Box<dyn HandlesMarketAndSubaccountQuery>>,
    pub trader_derivative_orders_response_handler: Option<Box<dyn HandlesMarketAndSubaccountQuery>>,
    pub trader_transient_spot_orders_response_handler: Option<Box<dyn HandlesMarketAndSubaccountQuery>>,
    pub trader_transient_derivative_orders_response_handler: Option<Box<dyn HandlesMarketAndSubaccountQuery>>,
    pub perpetual_market_info_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
    pub perpetual_market_funding_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
    pub market_volatility_response_handler: Option<Box<dyn HandlesMarketVolatilityQuery>>,
    pub spot_market_mid_price_and_tob_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
    pub derivative_market_mid_price_and_tob_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
    pub oracle_volatility_response_handler: Option<Box<dyn HandlesOracleVolatilityQuery>>,
    pub oracle_price_response_handler: Option<Box<dyn HandlesOraclePriceQuery>>,
    pub token_factory_denom_total_supply_handler: Option<Box<dyn HandlesDenomSupplyQuery>>,
    pub token_factory_denom_creation_fee_handler: Option<Box<dyn HandlesFeeQuery>>,
    pub balance_query_handler: Option<Box<dyn HandlesBankBalanceQuery>>,
    pub all_balances_query_handler: Option<Box<dyn HandlesBankAllBalancesQuery>>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<InjectiveQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {:?}", e),
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
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => match &self.smart_query_handler {
                Some(handler) => handler.handle(contract_addr, msg),
                None => panic!("Unknown smart query"),
            },
            QueryRequest::Bank(query) => match query {
                BankQuery::Balance { address, denom } => match &self.balance_query_handler {
                    Some(handler) => handler.handle(address.to_string(), denom.to_string()),
                    None => default_balance_query_handler(denom),
                },
                BankQuery::AllBalances { address } => match &self.all_balances_query_handler {
                    Some(handler) => handler.handle(address.to_string()),
                    None => default_all_balances_query_handler(),
                },
                _ => panic!("unsupported"),
            },
            QueryRequest::Custom(query) => match query.query_data.clone() {
                InjectiveQuery::SubaccountDeposit { subaccount_id, denom } => match &self.subaccount_deposit_response_handler {
                    Some(handler) => handler.handle(subaccount_id, denom),
                    None => default_subaccount_deposit_response_handler(),
                },
                InjectiveQuery::SpotMarket { market_id } => match &self.spot_market_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_spot_market_response_handler(market_id),
                },
                InjectiveQuery::TraderSpotOrders { market_id, subaccount_id } => match &self.trader_spot_orders_response_handler {
                    Some(handler) => handler.handle(market_id, subaccount_id),
                    None => default_trader_spot_orders_response_handler(),
                },
                InjectiveQuery::TraderSpotOrdersToCancelUpToAmount {
                    market_id,
                    subaccount_id,
                    base_amount,
                    quote_amount,
                    strategy,
                    reference_price,
                } => match &self.trader_spot_orders_to_cancel_up_to_amount_response_handler {
                    Some(handler) => handler.handle(market_id, subaccount_id, base_amount, quote_amount, strategy, reference_price),
                    None => default_trader_spot_orders_to_cancel_up_to_amount_response_handler(),
                },
                InjectiveQuery::TraderDerivativeOrdersToCancelUpToAmount {
                    market_id,
                    subaccount_id,
                    quote_amount,
                    strategy,
                    reference_price,
                } => match &self.trader_derivative_orders_to_cancel_up_to_amount_response_handler {
                    Some(handler) => handler.handle(market_id, subaccount_id, quote_amount, strategy, reference_price),
                    None => default_trader_derivative_orders_to_cancel_up_to_amount_response_handler(),
                },
                InjectiveQuery::DerivativeMarket { market_id } => match &self.derivative_market_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_derivative_market_response_handler(market_id),
                },
                InjectiveQuery::SubaccountPositions { subaccount_id } => match &self.subaccount_positions_response_handler {
                    Some(handler) => handler.handle(subaccount_id),
                    None => default_subaccount_positions_response_handler(),
                },
                InjectiveQuery::SubaccountPositionInMarket { market_id, subaccount_id } => {
                    match &self.subaccount_position_in_market_response_handler {
                        Some(handler) => handler.handle(market_id, subaccount_id),
                        None => default_subaccount_position_in_market_response_handler(),
                    }
                }
                InjectiveQuery::SubaccountEffectivePositionInMarket { market_id, subaccount_id } => {
                    match &self.subaccount_effective_position_in_market_response_handler {
                        Some(handler) => handler.handle(market_id, subaccount_id),
                        None => default_subaccount_effective_position_in_market_response_handler(),
                    }
                }
                InjectiveQuery::TraderDerivativeOrders { market_id, subaccount_id } => match &self.trader_derivative_orders_response_handler {
                    Some(handler) => handler.handle(market_id, subaccount_id),
                    None => default_trader_derivative_orders_response_handler(),
                },
                InjectiveQuery::TraderTransientSpotOrders { market_id, subaccount_id } => match &self.trader_transient_spot_orders_response_handler {
                    Some(handler) => handler.handle(market_id, subaccount_id),
                    None => default_trader_transient_spot_orders_response_handler(),
                },
                InjectiveQuery::TraderTransientDerivativeOrders { market_id, subaccount_id } => {
                    match &self.trader_transient_derivative_orders_response_handler {
                        Some(handler) => handler.handle(market_id, subaccount_id),
                        None => default_trader_transient_derivative_orders_response_handler(),
                    }
                }
                InjectiveQuery::PerpetualMarketInfo { market_id } => match &self.perpetual_market_info_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_perpetual_market_info_response_handler(),
                },
                InjectiveQuery::PerpetualMarketFunding { market_id } => match &self.perpetual_market_funding_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_perpetual_market_funding_response_handler(),
                },
                InjectiveQuery::MarketVolatility {
                    market_id,
                    trade_history_options,
                } => match &self.market_volatility_response_handler {
                    Some(handler) => handler.handle(market_id, trade_history_options),
                    None => default_market_volatility_response_handler(),
                },
                InjectiveQuery::SpotMarketMidPriceAndTob { market_id } => match &self.spot_market_mid_price_and_tob_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_spot_market_mid_price_and_tob_response_handler(),
                },
                InjectiveQuery::DerivativeMarketMidPriceAndTob { market_id } => match &self.derivative_market_mid_price_and_tob_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_derivative_market_mid_price_and_tob_response_handler(),
                },
                InjectiveQuery::OracleVolatility {
                    base_info,
                    quote_info,
                    oracle_history_options,
                } => match &self.oracle_volatility_response_handler {
                    Some(handler) => handler.handle(base_info, quote_info, oracle_history_options),
                    None => default_oracle_volatility_response_handler(),
                },
                InjectiveQuery::OraclePrice { oracle_type, base, quote } => match &self.oracle_price_response_handler {
                    Some(handler) => handler.handle(oracle_type, base, quote),
                    None => default_oracle_volatility_response_handler(),
                },
                InjectiveQuery::TokenFactoryDenomTotalSupply { denom } => match &self.token_factory_denom_total_supply_handler {
                    Some(handler) => handler.handle(denom),
                    None => default_token_factory_denom_total_supply_handler(),
                },
                InjectiveQuery::TokenFactoryDenomCreationFee {} => match &self.token_factory_denom_creation_fee_handler {
                    Some(handler) => handler.handle(),
                    None => default_token_factory_denom_creation_fee_handler(),
                },
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
            oracle_price_response_handler: None,
            token_factory_denom_total_supply_handler: None,
            token_factory_denom_creation_fee_handler: None,
            balance_query_handler: None,
            all_balances_query_handler: None,
        }
    }
}

pub struct TestCoin {
    pub amount: FPDecimal,
    pub denom: String,
}

impl TestCoin {
    pub fn new(amount: FPDecimal, denom: String) -> TestCoin {
        TestCoin { amount, denom }
    }
}

pub struct TestDeposit {
    pub deposit: Deposit,
    pub denom: String,
}

impl TestDeposit {
    pub fn new(deposit: Deposit, denom: String) -> TestDeposit {
        TestDeposit { deposit, denom }
    }
}

pub mod handlers {
    use cosmwasm_std::{to_binary, Coin, ContractResult, QuerierResult, SystemError, SystemResult, Uint128};

    use injective_math::FPDecimal;

    use crate::exchange_mock_querier::{HandlesDenomSupplyQuery, HandlesFeeQuery};
    use crate::query::{OraclePriceResponse, TokenFactoryCreateDenomFeeResponse, TokenFactoryDenomSupplyResponse};
    use crate::OracleType;
    use crate::{
        exchange_mock_querier::TestCoin, Deposit, DerivativeMarket, DerivativeMarketResponse, EffectivePosition, FullDerivativeMarket,
        FullDerivativeMarketPerpetualInfo, HandlesMarketAndSubaccountQuery, HandlesMarketIdQuery, HandlesOracleVolatilityQuery,
        HandlesSubaccountAndDenomQuery, HandlesTraderSpotOrdersToCancelUpToAmountQuery, MarketId, MetadataStatistics, OracleVolatilityResponse,
        Position, SpotMarket, SpotMarketMidPriceAndTOBResponse, SpotMarketResponse, SubaccountDepositResponse,
        SubaccountEffectivePositionInMarketResponse, SubaccountId, SubaccountPositionInMarketResponse, TradeRecord, TraderDerivativeOrdersResponse,
        TraderSpotOrdersResponse, TrimmedDerivativeLimitOrder, TrimmedSpotLimitOrder,
    };

    use super::{HandlesOraclePriceQuery, TestDeposit};

    pub fn create_subaccount_deposit_handler(coins: Vec<TestCoin>) -> Option<Box<dyn HandlesSubaccountAndDenomQuery>> {
        struct Temp {
            coins: Vec<TestCoin>,
        }
        impl HandlesSubaccountAndDenomQuery for Temp {
            fn handle(&self, _: SubaccountId, denom: String) -> QuerierResult {
                let iter = IntoIterator::into_iter(&self.coins);
                let matching_coins: Vec<&TestCoin> = iter.filter(|c| c.denom == denom).collect();
                if matching_coins.is_empty() || matching_coins.len() > 1 {
                    panic!("Expected to find one coin with denom '{}', but found {}", denom, matching_coins.len())
                }

                let response = SubaccountDepositResponse {
                    deposits: Deposit {
                        available_balance: matching_coins.first().unwrap().amount,
                        total_balance: matching_coins.first().unwrap().amount,
                    },
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { coins }))
    }

    pub fn create_subaccount_deposit_complex_handler(deposits: Vec<TestDeposit>) -> Option<Box<dyn HandlesSubaccountAndDenomQuery>> {
        struct Temp {
            deposits: Vec<TestDeposit>,
        }
        impl HandlesSubaccountAndDenomQuery for Temp {
            fn handle(&self, _: SubaccountId, denom: String) -> QuerierResult {
                let iter = IntoIterator::into_iter(&self.deposits);
                let matching_deposits: Vec<&TestDeposit> = iter.filter(|c| c.denom == denom).collect();
                if matching_deposits.is_empty() || matching_deposits.len() > 1 {
                    panic!(
                        "Expected to find one deposit with denom '{}', but found {}",
                        denom,
                        matching_deposits.len()
                    )
                }

                let response = SubaccountDepositResponse {
                    deposits: matching_deposits.first().unwrap().deposit.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { deposits }))
    }

    pub fn create_subaccount_deposit_err_returning_handler() -> Option<Box<dyn HandlesSubaccountAndDenomQuery>> {
        struct A();
        impl HandlesSubaccountAndDenomQuery for A {
            fn handle(&self, _: SubaccountId, _: String) -> QuerierResult {
                SystemResult::Err(SystemError::Unknown {})
            }
        }
        Some(Box::new(A()))
    }

    pub fn create_spot_market_handler(market: Option<SpotMarket>) -> Option<Box<dyn HandlesMarketIdQuery>> {
        struct Temp {
            market: Option<SpotMarket>,
        }
        impl HandlesMarketIdQuery for Temp {
            fn handle(&self, _: MarketId) -> QuerierResult {
                let response = SpotMarketResponse {
                    market: self.market.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { market }))
    }

    pub type SpotUpToAmountConsumingFunction = fn(MarketId, SubaccountId, FPDecimal, FPDecimal, i32, Option<FPDecimal>);

    pub fn create_spot_orders_up_to_amount_handler(
        orders: Option<Vec<TrimmedSpotLimitOrder>>,
        assertion: Option<SpotUpToAmountConsumingFunction>,
    ) -> Option<Box<dyn HandlesTraderSpotOrdersToCancelUpToAmountQuery>> {
        struct Temp {
            orders: Option<Vec<TrimmedSpotLimitOrder>>,
            assertion: Option<SpotUpToAmountConsumingFunction>,
        }
        impl HandlesTraderSpotOrdersToCancelUpToAmountQuery for Temp {
            fn handle(
                &self,
                market_id: MarketId,
                subaccount_id: SubaccountId,
                base_amount: FPDecimal,
                quote_amount: FPDecimal,
                strategy: i32,
                reference_price: Option<FPDecimal>,
            ) -> QuerierResult {
                if self.assertion.is_some() {
                    self.assertion.as_ref().unwrap()(market_id, subaccount_id, base_amount, quote_amount, strategy, reference_price)
                }
                let response = TraderSpotOrdersResponse {
                    orders: self.orders.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { orders, assertion }))
    }

    pub fn create_derivative_market_handler(
        market: Option<DerivativeMarket>,
        info: Option<FullDerivativeMarketPerpetualInfo>,
        mark_price: FPDecimal,
    ) -> Option<Box<dyn HandlesMarketIdQuery>> {
        struct Temp {
            market: Option<DerivativeMarket>,
            info: Option<FullDerivativeMarketPerpetualInfo>,
            mark_price: FPDecimal,
        }
        impl HandlesMarketIdQuery for Temp {
            fn handle(&self, _: MarketId) -> QuerierResult {
                let response = DerivativeMarketResponse {
                    market: FullDerivativeMarket {
                        market: self.market.to_owned(),
                        info: self.info.to_owned(),
                        mark_price: self.mark_price.to_owned(),
                    },
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { market, info, mark_price }))
    }

    pub fn create_trader_spot_orders_handler(orders: Option<Vec<TrimmedSpotLimitOrder>>) -> Option<Box<dyn HandlesMarketAndSubaccountQuery>> {
        struct Temp {
            orders: Option<Vec<TrimmedSpotLimitOrder>>,
        }
        impl HandlesMarketAndSubaccountQuery for Temp {
            fn handle(&self, _: MarketId, _: SubaccountId) -> QuerierResult {
                let response = TraderSpotOrdersResponse {
                    orders: self.orders.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { orders }))
    }

    pub fn create_trader_derivative_orders_handler(
        orders: Option<Vec<TrimmedDerivativeLimitOrder>>,
    ) -> Option<Box<dyn HandlesMarketAndSubaccountQuery>> {
        struct Temp {
            orders: Option<Vec<TrimmedDerivativeLimitOrder>>,
        }
        impl HandlesMarketAndSubaccountQuery for Temp {
            fn handle(&self, _: MarketId, _: SubaccountId) -> QuerierResult {
                let response = TraderDerivativeOrdersResponse {
                    orders: self.orders.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { orders }))
    }

    pub fn create_subaccount_effective_position_in_market_handler(
        position: Option<EffectivePosition>,
    ) -> Option<Box<dyn HandlesMarketAndSubaccountQuery>> {
        struct Temp {
            position: Option<EffectivePosition>,
        }

        impl HandlesMarketAndSubaccountQuery for Temp {
            fn handle(&self, _: MarketId, _: SubaccountId) -> QuerierResult {
                let response = SubaccountEffectivePositionInMarketResponse {
                    state: self.position.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }

        Some(Box::new(Temp { position }))
    }

    pub fn create_subaccount_position_in_market_handler(position: Option<Position>) -> Option<Box<dyn HandlesMarketAndSubaccountQuery>> {
        struct Temp {
            position: Option<Position>,
        }

        impl HandlesMarketAndSubaccountQuery for Temp {
            fn handle(&self, _: MarketId, _: SubaccountId) -> QuerierResult {
                let response = SubaccountPositionInMarketResponse {
                    state: self.position.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }

        Some(Box::new(Temp { position }))
    }

    pub fn create_spot_market_mid_price_and_tob_handler(mid_price: Option<FPDecimal>) -> Option<Box<dyn HandlesMarketIdQuery>> {
        struct Temp {
            mid_price: Option<FPDecimal>,
        }
        impl HandlesMarketIdQuery for Temp {
            fn handle(&self, _: MarketId) -> QuerierResult {
                let response = SpotMarketMidPriceAndTOBResponse {
                    mid_price: self.mid_price.to_owned(),
                    best_bid: None,
                    best_ask: None,
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { mid_price }))
    }

    pub fn create_oracle_volatility_handler(
        volatility: Option<FPDecimal>,
        history_metadata: Option<MetadataStatistics>,
        raw_history: Option<Vec<TradeRecord>>,
    ) -> Option<Box<dyn HandlesOracleVolatilityQuery>> {
        struct Temp {
            volatility: Option<FPDecimal>,
            history_metadata: Option<MetadataStatistics>,
            raw_history: Option<Vec<TradeRecord>>,
        }
        impl HandlesOracleVolatilityQuery for Temp {
            fn handle(
                &self,
                _: Option<crate::OracleInfo>,
                _: Option<crate::OracleInfo>,
                _: Option<crate::oracle::OracleHistoryOptions>,
            ) -> QuerierResult {
                let response = OracleVolatilityResponse {
                    volatility: self.volatility.to_owned(),
                    history_metadata: self.history_metadata.to_owned(),
                    raw_history: self.raw_history.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp {
            volatility,
            history_metadata,
            raw_history,
        }))
    }

    pub fn create_oracle_query_handler(price: FPDecimal) -> Option<Box<dyn HandlesOraclePriceQuery>> {
        struct Temp {
            price: FPDecimal,
        }
        impl HandlesOraclePriceQuery for Temp {
            fn handle(&self, _: OracleType, _: String, _: String) -> QuerierResult {
                let response = OraclePriceResponse {
                    price: self.price.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { price }))
    }

    pub fn create_denom_supply_handler(supply: Uint128) -> Option<Box<dyn HandlesDenomSupplyQuery>> {
        struct Temp {
            supply: Uint128,
        }
        impl HandlesDenomSupplyQuery for Temp {
            fn handle(&self, _denom: String) -> QuerierResult {
                let response = TokenFactoryDenomSupplyResponse { total_supply: self.supply };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { supply }))
    }

    pub fn create_denom_creation_fee_handler(fee: Vec<Coin>) -> Option<Box<dyn HandlesFeeQuery>> {
        struct Temp {
            fee: Vec<Coin>,
        }
        impl HandlesFeeQuery for Temp {
            fn handle(&self) -> QuerierResult {
                let response = TokenFactoryCreateDenomFeeResponse { fee: self.fee.to_owned() };
                SystemResult::Ok(ContractResult::from(to_binary(&response)))
            }
        }
        Some(Box::new(Temp { fee }))
    }
}
