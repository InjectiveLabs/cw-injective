use std::marker::PhantomData;
use std::str::FromStr;

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    from_json, to_json_binary, Addr, AllBalanceResponse, BalanceResponse, BankQuery, Binary, Coin, ContractResult, OwnedDeps, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};

use injective_math::FPDecimal;

use crate::exchange::{
    derivative_market::DerivativeMarket,
    response::QueryOrderbookResponse,
    types::{AtomicMarketOrderAccessLevel, MarketVolume, Params, PriceLevel, VolumeByType},
};
use crate::oracle::{
    types::{OracleHistoryOptions, OracleType, PriceState, PythPriceState},
    volatility::TradeHistoryOptions,
};
use crate::tokenfactory::response::{TokenFactoryCreateDenomFeeResponse, TokenFactoryDenomSupplyResponse};
use crate::wasmx::response::QueryContractRegistrationInfoResponse;
use crate::{
    CancellationStrategy, Deposit, DerivativeMarketResponse, ExchangeParamsResponse, FullDerivativeMarket, InjectiveQuery, InjectiveQueryWrapper,
    MarketMidPriceAndTOBResponse, MarketStatus, MarketVolatilityResponse, OracleInfo, OracleVolatilityResponse, OrderSide,
    PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, PythPriceResponse, QueryAggregateMarketVolumeResponse, QueryAggregateVolumeResponse,
    QueryDenomDecimalResponse, QueryDenomDecimalsResponse, QueryMarketAtomicExecutionFeeMultiplierResponse, SpotMarket, SpotMarketResponse,
    SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse, TraderDerivativeOrdersResponse,
    TraderSpotOrdersResponse,
};
use crate::{MarketId, SubaccountId};

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InjectiveQueryWrapper> {
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new();

    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

fn default_subaccount_deposit_response_handler() -> QuerierResult {
    let response = SubaccountDepositResponse {
        deposits: Deposit {
            // NOTE: this is 100 with 8 decimal places
            available_balance: FPDecimal::from(10_000_000_000u128),
            total_balance: FPDecimal::from(10_000_000_000u128),
        },
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_spot_market_response_handler(market_id: MarketId) -> QuerierResult {
    let response = SpotMarketResponse {
        market: Some(SpotMarket {
            ticker: "INJ/USDT".to_string(),
            base_denom: "INJ".to_string(),
            quote_denom: "USDT".to_string(),
            maker_fee_rate: FPDecimal::from_str("-0.0001").unwrap(),
            taker_fee_rate: FPDecimal::from_str("0.001").unwrap(),
            relayer_fee_share_rate: FPDecimal::from_str("0.4").unwrap(),
            market_id,
            status: MarketStatus::Active,
            min_price_tick_size: FPDecimal::from_str("0.01").unwrap(),
            min_quantity_tick_size: FPDecimal::from_str("1000000000000000.0").unwrap(),
        }),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_trader_spot_orders_response_handler() -> QuerierResult {
    let response = TraderSpotOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_trader_spot_orders_to_cancel_up_to_amount_response_handler() -> QuerierResult {
    let response = TraderSpotOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_trader_derivative_orders_to_cancel_up_to_amount_response_handler() -> QuerierResult {
    let response = TraderDerivativeOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_derivative_market_response_handler(market_id: MarketId) -> QuerierResult {
    let response = DerivativeMarketResponse {
        market: Some(FullDerivativeMarket {
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
                status: MarketStatus::Active,
                min_price_tick_size: FPDecimal::from_str("100000.0").unwrap(),
                min_quantity_tick_size: FPDecimal::from_str("0.0001").unwrap(),
            }),
            info: None,
            mark_price: FPDecimal::ONE,
        }),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_subaccount_positions_response_handler() -> QuerierResult {
    todo!()
}

fn default_subaccount_position_in_market_response_handler() -> QuerierResult {
    let response = SubaccountPositionInMarketResponse { state: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_subaccount_effective_position_in_market_response_handler() -> QuerierResult {
    let response = SubaccountEffectivePositionInMarketResponse { state: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_trader_derivative_orders_response_handler() -> QuerierResult {
    let response = TraderDerivativeOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_trader_transient_spot_orders_response_handler() -> QuerierResult {
    let response = TraderSpotOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_trader_transient_derivative_orders_response_handler() -> QuerierResult {
    let response = TraderDerivativeOrdersResponse { orders: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_perpetual_market_info_response_handler() -> QuerierResult {
    let response = PerpetualMarketInfoResponse { info: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_perpetual_market_funding_response_handler() -> QuerierResult {
    let response = PerpetualMarketFundingResponse { state: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_market_volatility_response_handler() -> QuerierResult {
    let response = MarketVolatilityResponse {
        volatility: Some(FPDecimal::ONE),
        history_metadata: None,
        raw_history: None,
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_spot_market_mid_price_and_tob_response_handler() -> QuerierResult {
    let response = MarketMidPriceAndTOBResponse {
        mid_price: Some(FPDecimal::from_str("200000").unwrap()),
        best_buy_price: None,
        best_sell_price: None,
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_derivative_market_mid_price_and_tob_response_handler() -> QuerierResult {
    let response = MarketMidPriceAndTOBResponse {
        mid_price: Some(FPDecimal::from_str("200000").unwrap()),
        best_buy_price: None,
        best_sell_price: None,
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_aggregate_market_volume_handler() -> QuerierResult {
    let response = QueryAggregateMarketVolumeResponse {
        volume: VolumeByType {
            maker_volume: FPDecimal::from(100u128),
            taker_volume: FPDecimal::from(100u128),
        },
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_aggregate_account_volume_handler() -> QuerierResult {
    let response = QueryAggregateVolumeResponse {
        aggregate_volumes: vec![
            MarketVolume {
                market_id: MarketId::unchecked("market_id_1"),
                volume: VolumeByType {
                    maker_volume: FPDecimal::from(10000000u128),
                    taker_volume: FPDecimal::from(14000000u128),
                },
            },
            MarketVolume {
                market_id: MarketId::unchecked("market_id_2"),
                volume: VolumeByType {
                    maker_volume: FPDecimal::from(20000000u128),
                    taker_volume: FPDecimal::from(25000000u128),
                },
            },
        ],
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_denom_decimal_handler() -> QuerierResult {
    let response = QueryDenomDecimalResponse { decimals: 6 };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_denom_decimals_handler() -> QuerierResult {
    let response = QueryDenomDecimalsResponse { denom_decimals: vec![] };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_oracle_volatility_response_handler() -> QuerierResult {
    let response = OracleVolatilityResponse {
        volatility: Some(FPDecimal::ONE),
        history_metadata: None,
        raw_history: None,
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_pyth_price_response_handler() -> QuerierResult {
    let response = PythPriceResponse {
        price_state: Some(PythPriceState {
            price_id: "0xff0ec26442c57d7456695b843694e7379b15cf1b250b27e0e47e657f1955aaff".to_string(),
            ema_price: FPDecimal::ONE,
            ema_conf: FPDecimal::ONE,
            conf: FPDecimal::ONE,
            publish_time: 1i64,
            price_state: PriceState {
                price: FPDecimal::ONE,
                cumulative_price: FPDecimal::ONE,
                timestamp: 1i64,
            },
        }),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_token_factory_denom_total_supply_handler() -> QuerierResult {
    let response = TokenFactoryDenomSupplyResponse {
        total_supply: Uint128::from(1000u128),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_token_factory_denom_creation_fee_handler() -> QuerierResult {
    let response = TokenFactoryCreateDenomFeeResponse {
        fee: vec![Coin::new(10, "inj")],
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_contract_registration_info_response_handler() -> QuerierResult {
    let response = QueryContractRegistrationInfoResponse { contract: None };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_balance_bank_query_handler(denom: impl Into<String>) -> QuerierResult {
    let response = BalanceResponse {
        amount: Coin::new(1000000000000000, denom),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_all_balances_bank_query_handler() -> QuerierResult {
    let response = AllBalanceResponse {
        amount: vec![Coin::new(1000000000000000, "inj")],
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_exchange_params_response_handler() -> QuerierResult {
    let denom = "inj";

    let response = ExchangeParamsResponse {
        params: Some(Params {
            spot_market_instant_listing_fee: Coin::new(100000000000000000000, denom),
            derivative_market_instant_listing_fee: Coin::new(1000000000000000000000, denom),
            default_spot_maker_fee_rate: FPDecimal::must_from_str("-0.0001"),
            default_spot_taker_fee_rate: FPDecimal::must_from_str("0.001"),
            default_derivative_maker_fee_rate: FPDecimal::must_from_str("-0.0001"),
            default_derivative_taker_fee_rate: FPDecimal::must_from_str("0.001"),
            default_initial_margin_ratio: FPDecimal::must_from_str("0.05"),
            default_maintenance_margin_ratio: FPDecimal::must_from_str("0.02"),
            default_funding_interval: 3600i64,
            // default_multiple: 3600i64,
            relayer_fee_share_rate: FPDecimal::must_from_str("0.4"),
            default_hourly_funding_rate_cap: FPDecimal::must_from_str("0.000625"),
            default_hourly_interest_rate: FPDecimal::must_from_str("0.00000416666"),
            max_derivative_order_side_count: 20u32,
            inj_reward_staked_requirement_threshold: FPDecimal::must_from_str("25000000000000000000"),
            trading_rewards_vesting_duration: 1209600i64,
            liquidator_reward_share_rate: FPDecimal::must_from_str("0.05"),
            binary_options_market_instant_listing_fee: Coin::new(10000000000000000000, denom),
            atomic_market_order_access_level: AtomicMarketOrderAccessLevel::SmartContractsOnly,
            spot_atomic_market_order_fee_multiplier: FPDecimal::must_from_str("2.0"),
            derivative_atomic_market_order_fee_multiplier: FPDecimal::must_from_str("2.0"),
            binary_options_atomic_market_order_fee_multiplier: FPDecimal::must_from_str("2.0"),
            minimal_protocol_fee_rate: FPDecimal::must_from_str("0.00001"),
            is_instant_derivative_market_launch_enabled: Some(true),
        }),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_spot_market_orderbook_response_handler() -> QuerierResult {
    let response = QueryOrderbookResponse {
        buys_price_level: vec![PriceLevel::new(9u128.into(), 10u128.into()), PriceLevel::new(8u128.into(), 10u128.into())],
        sells_price_level: vec![
            PriceLevel::new(11u128.into(), 10u128.into()),
            PriceLevel::new(12u128.into(), 10u128.into()),
        ],
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
}

fn default_derivative_market_orderbook_response_handler() -> QuerierResult {
    let response = QueryOrderbookResponse {
        buys_price_level: vec![PriceLevel::new(9u128.into(), 10u128.into()), PriceLevel::new(8u128.into(), 10u128.into())],
        sells_price_level: vec![
            PriceLevel::new(11u128.into(), 10u128.into()),
            PriceLevel::new(12u128.into(), 10u128.into()),
        ],
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}

fn default_market_atomic_execution_fee_multiplier_response_handler() -> QuerierResult {
    let response = QueryMarketAtomicExecutionFeeMultiplierResponse {
        multiplier: FPDecimal::from_str("2.0").unwrap(),
    };
    SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
        strategy: CancellationStrategy,
        reference_price: Option<FPDecimal>,
    ) -> QuerierResult;
}

pub trait HandlesTraderDerivativeOrdersToCancelUpToAmountQuery {
    fn handle(
        &self,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        quote_amount: FPDecimal,
        strategy: CancellationStrategy,
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

pub trait HandlesStakedAmountQuery {
    fn handle(&self, delegator_address: Addr, max_delegations: u16) -> QuerierResult;
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

pub trait HandlesPythPriceQuery {
    fn handle(&self, price_id: String) -> QuerierResult;
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

pub trait HandlesByAddressQuery {
    fn handle(&self, address: String) -> QuerierResult;
}

pub trait HandlesMarketVolumeQuery {
    fn handle(&self, market_id: MarketId) -> QuerierResult;
}

pub trait HandlesAccountVolumeQuery {
    fn handle(&self, account: String) -> QuerierResult;
}

pub trait HandlesDenomDecimalQuery {
    fn handle(&self, denom: String) -> QuerierResult;
}

pub trait HandlesDenomDecimalsQuery {
    fn handle(&self, denoms: Vec<String>) -> QuerierResult;
}

pub trait HandlesPriceLevelsQuery {
    fn handle(&self, market_id: MarketId, order_side: OrderSide) -> QuerierResult;
}

pub trait HandlesDerivativePriceLevelsQuery {
    fn handle(&self, market_id: MarketId) -> QuerierResult;
}

pub trait HandlesExchangeParamsQuery {
    fn handle(&self) -> QuerierResult;
}

pub struct WasmMockQuerier {
    pub smart_query_handler: Option<Box<dyn HandlesSmartQuery>>,
    pub subaccount_deposit_response_handler: Option<Box<dyn HandlesSubaccountAndDenomQuery>>,
    pub exchange_params_response_handler: Option<Box<dyn HandlesExchangeParamsQuery>>,
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
    pub aggregate_market_volume_handler: Option<Box<dyn HandlesMarketVolumeQuery>>,
    pub aggregate_account_volume_handler: Option<Box<dyn HandlesAccountVolumeQuery>>,
    pub denom_decimal_handler: Option<Box<dyn HandlesDenomDecimalQuery>>,
    pub denom_decimals_handler: Option<Box<dyn HandlesDenomDecimalsQuery>>,
    pub staked_amount_handler: Option<Box<dyn HandlesStakedAmountQuery>>,
    pub oracle_volatility_response_handler: Option<Box<dyn HandlesOracleVolatilityQuery>>,
    pub oracle_price_response_handler: Option<Box<dyn HandlesOraclePriceQuery>>,
    pub pyth_price_response_handler: Option<Box<dyn HandlesPythPriceQuery>>,
    pub token_factory_denom_total_supply_handler: Option<Box<dyn HandlesDenomSupplyQuery>>,
    pub token_factory_denom_creation_fee_handler: Option<Box<dyn HandlesFeeQuery>>,
    pub balance_query_handler: Option<Box<dyn HandlesBankBalanceQuery>>,
    pub all_balances_query_handler: Option<Box<dyn HandlesBankAllBalancesQuery>>,
    pub registered_contract_info_query_handler: Option<Box<dyn HandlesByAddressQuery>>,
    pub spot_market_orderbook_response_handler: Option<Box<dyn HandlesPriceLevelsQuery>>,
    pub derivative_market_orderbook_response_handler: Option<Box<dyn HandlesDerivativePriceLevelsQuery>>,
    pub market_atomic_execution_fee_multiplier_response_handler: Option<Box<dyn HandlesMarketIdQuery>>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<InjectiveQueryWrapper> = match from_json(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {e:?}"),
                    request: bin_request.into(),
                });
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
                    None => default_balance_bank_query_handler(denom),
                },
                BankQuery::AllBalances { address } => match &self.all_balances_query_handler {
                    Some(handler) => handler.handle(address.to_string()),
                    None => default_all_balances_bank_query_handler(),
                },
                _ => panic!("unsupported"),
            },
            QueryRequest::Custom(query) => match query.query_data.clone() {
                InjectiveQuery::Grants {
                    granter: _,
                    grantee: _,
                    msg_type_url: _,
                    pagination: _,
                } => todo!(),
                InjectiveQuery::GranteeGrants { grantee: _, pagination: _ } => todo!(),
                InjectiveQuery::GranterGrants { granter: _, pagination: _ } => todo!(),
                InjectiveQuery::SubaccountDeposit { subaccount_id, denom } => match &self.subaccount_deposit_response_handler {
                    Some(handler) => handler.handle(subaccount_id, denom),
                    None => default_subaccount_deposit_response_handler(),
                },
                InjectiveQuery::ExchangeParams {} => match &self.exchange_params_response_handler {
                    Some(handler) => handler.handle(),
                    None => default_exchange_params_response_handler(),
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
                InjectiveQuery::AggregateMarketVolume { market_id } => match &self.aggregate_market_volume_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_aggregate_market_volume_handler(),
                },
                InjectiveQuery::AggregateAccountVolume { account } => match &self.aggregate_account_volume_handler {
                    Some(handler) => handler.handle(account),
                    None => default_aggregate_account_volume_handler(),
                },
                InjectiveQuery::DenomDecimal { denom } => match &self.denom_decimal_handler {
                    Some(handler) => handler.handle(denom),
                    None => default_denom_decimal_handler(),
                },
                InjectiveQuery::DenomDecimals { denoms } => match &self.denom_decimals_handler {
                    Some(handler) => handler.handle(denoms),
                    None => default_denom_decimals_handler(),
                },
                InjectiveQuery::StakedAmount {
                    delegator_address,
                    max_delegations,
                } => match &self.staked_amount_handler {
                    Some(handler) => handler.handle(delegator_address, max_delegations),
                    None => default_oracle_volatility_response_handler(),
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
                InjectiveQuery::PythPrice { price_id } => match &self.pyth_price_response_handler {
                    Some(handler) => handler.handle(price_id),
                    None => default_pyth_price_response_handler(),
                },
                InjectiveQuery::TokenFactoryDenomTotalSupply { denom } => match &self.token_factory_denom_total_supply_handler {
                    Some(handler) => handler.handle(denom),
                    None => default_token_factory_denom_total_supply_handler(),
                },
                InjectiveQuery::TokenFactoryDenomCreationFee {} => match &self.token_factory_denom_creation_fee_handler {
                    Some(handler) => handler.handle(),
                    None => default_token_factory_denom_creation_fee_handler(),
                },
                InjectiveQuery::WasmxRegisteredContractInfo { contract_address } => match &self.registered_contract_info_query_handler {
                    Some(handler) => handler.handle(contract_address),
                    None => default_contract_registration_info_response_handler(),
                },
                InjectiveQuery::SpotOrderbook { order_side, market_id, .. } => match &self.spot_market_orderbook_response_handler {
                    Some(handler) => handler.handle(market_id, order_side),
                    None => default_spot_market_orderbook_response_handler(),
                },
                InjectiveQuery::DerivativeOrderbook { market_id, .. } => match &self.derivative_market_orderbook_response_handler {
                    Some(handler) => handler.handle(market_id),
                    None => default_derivative_market_orderbook_response_handler(),
                },
                InjectiveQuery::MarketAtomicExecutionFeeMultiplier { market_id } => {
                    match &self.market_atomic_execution_fee_multiplier_response_handler {
                        Some(handler) => handler.handle(market_id),
                        None => default_market_atomic_execution_fee_multiplier_response_handler(),
                    }
                }
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
            subaccount_deposit_response_handler: None,
            exchange_params_response_handler: None,
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
            aggregate_account_volume_handler: None,
            denom_decimal_handler: None,
            aggregate_market_volume_handler: None,
            staked_amount_handler: None,
            oracle_volatility_response_handler: None,
            oracle_price_response_handler: None,
            pyth_price_response_handler: None,
            token_factory_denom_total_supply_handler: None,
            token_factory_denom_creation_fee_handler: None,
            balance_query_handler: None,
            all_balances_query_handler: None,
            registered_contract_info_query_handler: None,
            denom_decimals_handler: None,
            spot_market_orderbook_response_handler: None,
            derivative_market_orderbook_response_handler: None,
            market_atomic_execution_fee_multiplier_response_handler: None,
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
    use cosmwasm_std::{
        to_json_binary, AllBalanceResponse, BalanceResponse, Binary, Coin, ContractResult, QuerierResult, StdResult, SystemError, SystemResult,
        Uint128,
    };
    use std::collections::HashMap;

    use injective_math::FPDecimal;

    use crate::exchange::response::QueryOrderbookResponse;
    use crate::exchange_mock_querier::{HandlesByAddressQuery, HandlesDenomSupplyQuery, HandlesFeeQuery};
    use crate::oracle::{response::OraclePriceResponse, types::PricePairState};
    use crate::tokenfactory::response::{TokenFactoryCreateDenomFeeResponse, TokenFactoryDenomSupplyResponse};
    use crate::wasmx::{response::QueryContractRegistrationInfoResponse, types::RegisteredContract};
    use crate::{
        exchange_mock_querier::TestCoin, CancellationStrategy, Deposit, DerivativeMarket, DerivativeMarketResponse, EffectivePosition,
        FullDerivativeMarket, FullDerivativeMarketPerpetualInfo, HandlesMarketAndSubaccountQuery, HandlesMarketIdQuery, HandlesOracleVolatilityQuery,
        HandlesPriceLevelsQuery, HandlesSmartQuery, HandlesSubaccountAndDenomQuery, HandlesTraderSpotOrdersToCancelUpToAmountQuery, MarketId,
        MetadataStatistics, OracleVolatilityResponse, OrderSide, Position, PriceLevel, QueryMarketAtomicExecutionFeeMultiplierResponse, SpotMarket,
        SpotMarketResponse, SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse, SubaccountId, SubaccountPositionInMarketResponse,
        TradeRecord, TraderDerivativeOrdersResponse, TraderSpotOrdersResponse, TrimmedDerivativeLimitOrder, TrimmedSpotLimitOrder,
    };
    use crate::{
        HandlesBankAllBalancesQuery, HandlesBankBalanceQuery, HandlesTraderDerivativeOrdersToCancelUpToAmountQuery, MarketMidPriceAndTOBResponse,
        OracleType,
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp { market }))
    }

    pub fn create_orderbook_response_handler(orderbooks: HashMap<MarketId, Vec<PriceLevel>>) -> Option<Box<dyn HandlesPriceLevelsQuery>> {
        struct Temp {
            orderbooks: HashMap<MarketId, Vec<PriceLevel>>,
        }
        impl HandlesPriceLevelsQuery for Temp {
            fn handle(&self, market_id: MarketId, order_side: OrderSide) -> QuerierResult {
                let price_levels_opt = self.orderbooks.get(&market_id);
                if price_levels_opt.is_none() {
                    return SystemResult::Err(SystemError::Unknown {});
                }
                let price_levels = price_levels_opt.unwrap().clone();
                let response = QueryOrderbookResponse {
                    buys_price_level: if order_side != OrderSide::Sell { price_levels.clone() } else { vec![] },
                    sells_price_level: if order_side != OrderSide::Buy { price_levels } else { vec![] },
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp { orderbooks }))
    }

    pub fn create_spot_multi_market_handler(markets: HashMap<MarketId, SpotMarket>) -> Option<Box<dyn HandlesMarketIdQuery>> {
        struct Temp {
            markets: HashMap<MarketId, SpotMarket>,
        }

        impl HandlesMarketIdQuery for Temp {
            fn handle(&self, market_id: MarketId) -> QuerierResult {
                let response = SpotMarketResponse {
                    market: self.markets.get(&market_id).cloned(),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }

        Some(Box::new(Temp { markets }))
    }

    pub type SpotUpToAmountConsumingFunction = fn(MarketId, SubaccountId, FPDecimal, FPDecimal, CancellationStrategy, Option<FPDecimal>);

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
                strategy: CancellationStrategy,
                reference_price: Option<FPDecimal>,
            ) -> QuerierResult {
                if self.assertion.is_some() {
                    self.assertion.as_ref().unwrap()(market_id, subaccount_id, base_amount, quote_amount, strategy, reference_price)
                }
                let response = TraderSpotOrdersResponse {
                    orders: self.orders.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp { orders, assertion }))
    }

    pub type DerivativeUpToAmountConsumingFunction = fn(MarketId, SubaccountId, FPDecimal, CancellationStrategy, Option<FPDecimal>);

    pub fn create_derivative_orders_up_to_amount_handler(
        orders: Option<Vec<TrimmedDerivativeLimitOrder>>,
        assertion: Option<DerivativeUpToAmountConsumingFunction>,
    ) -> Option<Box<dyn HandlesTraderDerivativeOrdersToCancelUpToAmountQuery>> {
        struct Temp {
            orders: Option<Vec<TrimmedDerivativeLimitOrder>>,
            assertion: Option<DerivativeUpToAmountConsumingFunction>,
        }
        impl HandlesTraderDerivativeOrdersToCancelUpToAmountQuery for Temp {
            fn handle(
                &self,
                market_id: MarketId,
                subaccount_id: SubaccountId,
                quote_amount: FPDecimal,
                strategy: CancellationStrategy,
                reference_price: Option<FPDecimal>,
            ) -> QuerierResult {
                if self.assertion.is_some() {
                    self.assertion.as_ref().unwrap()(market_id, subaccount_id, quote_amount, strategy, reference_price)
                }
                let response = TraderDerivativeOrdersResponse {
                    orders: self.orders.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                    market: Some(FullDerivativeMarket {
                        market: self.market.to_owned(),
                        info: self.info.to_owned(),
                        mark_price: self.mark_price.to_owned(),
                    }),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }

        Some(Box::new(Temp { position }))
    }

    pub fn create_market_mid_price_and_tob_handler(
        mid_price: Option<FPDecimal>,
        best_buy_price: Option<FPDecimal>,
        best_sell_price: Option<FPDecimal>,
    ) -> Option<Box<dyn HandlesMarketIdQuery>> {
        struct Temp {
            mid_price: Option<FPDecimal>,
            best_buy_price: Option<FPDecimal>,
            best_sell_price: Option<FPDecimal>,
        }
        impl HandlesMarketIdQuery for Temp {
            fn handle(&self, _: MarketId) -> QuerierResult {
                let response = MarketMidPriceAndTOBResponse {
                    mid_price: self.mid_price.to_owned(),
                    best_buy_price: self.best_buy_price.to_owned(),
                    best_sell_price: self.best_sell_price.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp {
            mid_price,
            best_buy_price,
            best_sell_price,
        }))
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
                _: Option<crate::oracle::types::OracleHistoryOptions>,
            ) -> QuerierResult {
                let response = OracleVolatilityResponse {
                    volatility: self.volatility.to_owned(),
                    history_metadata: self.history_metadata.to_owned(),
                    raw_history: self.raw_history.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp {
            volatility,
            history_metadata,
            raw_history,
        }))
    }

    pub fn create_oracle_query_handler(
        pair_price: FPDecimal,
        base_price: FPDecimal,
        quote_price: FPDecimal,
        base_cumulative_price: FPDecimal,
        quote_cumulative_price: FPDecimal,
        base_timestamp: i64,
        quote_timestamp: i64,
    ) -> Option<Box<dyn HandlesOraclePriceQuery>> {
        struct Temp {
            pair_price: FPDecimal,
            base_price: FPDecimal,
            quote_price: FPDecimal,
            base_cumulative_price: FPDecimal,
            quote_cumulative_price: FPDecimal,
            base_timestamp: i64,
            quote_timestamp: i64,
        }
        impl HandlesOraclePriceQuery for Temp {
            fn handle(&self, _: OracleType, _: String, _: String) -> QuerierResult {
                let response = OraclePriceResponse {
                    price_pair_state: Some(PricePairState {
                        pair_price: self.pair_price.to_owned(),
                        base_price: self.base_price.to_owned(),
                        quote_price: self.quote_price.to_owned(),
                        base_cumulative_price: self.base_cumulative_price.to_owned(),
                        quote_cumulative_price: self.quote_cumulative_price.to_owned(),
                        base_timestamp: self.base_timestamp.to_owned(),
                        quote_timestamp: self.quote_timestamp.to_owned(),
                    }),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp {
            pair_price,
            base_price,
            quote_price,
            base_cumulative_price,
            quote_cumulative_price,
            base_timestamp,
            quote_timestamp,
        }))
    }

    pub fn create_denom_supply_handler(supply: Uint128) -> Option<Box<dyn HandlesDenomSupplyQuery>> {
        struct Temp {
            supply: Uint128,
        }
        impl HandlesDenomSupplyQuery for Temp {
            fn handle(&self, _denom: String) -> QuerierResult {
                let response = TokenFactoryDenomSupplyResponse { total_supply: self.supply };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
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
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp { fee }))
    }

    pub fn create_registered_contract_info_query_handler(contract: Option<RegisteredContract>) -> Option<Box<dyn HandlesByAddressQuery>> {
        struct Temp {
            contract: Option<RegisteredContract>,
        }
        impl HandlesByAddressQuery for Temp {
            fn handle(&self, _address: String) -> QuerierResult {
                let response = QueryContractRegistrationInfoResponse {
                    contract: self.contract.to_owned(),
                };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp { contract }))
    }

    pub fn create_simple_balance_bank_query_handler(balances: Vec<Coin>) -> Option<Box<dyn HandlesBankBalanceQuery>> {
        struct Temp {
            balances: Vec<Coin>,
        }
        impl HandlesBankBalanceQuery for Temp {
            fn handle(&self, _: String, denom: String) -> QuerierResult {
                let balances = self.balances.to_owned();
                let empty = Coin::new(0, denom.clone());
                let balance = balances.iter().find(|b| -> bool { b.denom == denom }).unwrap_or(&empty);
                let res = BalanceResponse { amount: balance.to_owned() };

                SystemResult::Ok(ContractResult::from(to_json_binary(&res)))
            }
        }
        Some(Box::new(Temp { balances }))
    }

    pub fn create_simple_all_balances_bank_query_handler(balances: Vec<Coin>) -> Option<Box<dyn HandlesBankAllBalancesQuery>> {
        struct Temp {
            balances: Vec<Coin>,
        }
        impl HandlesBankAllBalancesQuery for Temp {
            fn handle(&self, _: String) -> QuerierResult {
                let res = AllBalanceResponse {
                    amount: self.balances.to_owned(),
                };

                SystemResult::Ok(ContractResult::from(to_json_binary(&res)))
            }
        }
        Some(Box::new(Temp { balances }))
    }

    pub fn create_atomic_order_fee_multiplier_handler(multiplier: FPDecimal) -> Option<Box<dyn HandlesMarketIdQuery>> {
        struct Temp {
            multiplier: FPDecimal,
        }
        impl HandlesMarketIdQuery for Temp {
            fn handle(&self, _market_id: MarketId) -> QuerierResult {
                let response = QueryMarketAtomicExecutionFeeMultiplierResponse { multiplier: self.multiplier };
                SystemResult::Ok(ContractResult::from(to_json_binary(&response)))
            }
        }
        Some(Box::new(Temp { multiplier }))
    }

    pub fn create_smart_query_handler(result: Result<Binary, SystemError>) -> Option<Box<dyn HandlesSmartQuery>> {
        struct Temp {
            result: Result<Binary, SystemError>,
        }
        impl HandlesSmartQuery for Temp {
            fn handle(&self, _contract_addr: &str, _msg: &Binary) -> QuerierResult {
                match self.result.clone() {
                    Ok(resp) => SystemResult::Ok(ContractResult::from(StdResult::Ok(resp))),
                    Err(err) => SystemResult::Err(err),
                }
            }
        }
        Some(Box::new(Temp { result }))
    }
}
