use cosmwasm_std::{to_json_binary, Addr, Binary, StdResult};
use injective_cosmwasm::{CancellationStrategy, InjectiveQuerier, MarketId, OracleInfo, OracleType, OrderSide, SubaccountId};
use injective_math::FPDecimal;

pub fn handle_exchange_params_query(querier: &InjectiveQuerier) -> StdResult<Binary> {
    to_json_binary(&querier.query_exchange_params()?)
}

pub fn handle_subaccount_deposit_query(querier: &InjectiveQuerier, subaccount_id: &SubaccountId, denom: String) -> StdResult<Binary> {
    to_json_binary(&querier.query_subaccount_deposit(subaccount_id, &denom)?)
}

pub fn handle_spot_market_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_spot_market(market_id)?)
}

pub fn handle_derivative_market_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_derivative_market(market_id)?)
}

pub fn handle_effective_subaccount_position_query(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    subaccount_id: &SubaccountId,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_effective_subaccount_position(market_id, subaccount_id)?)
}

pub fn handle_vanilla_subaccount_position_query(querier: &InjectiveQuerier, market_id: &MarketId, subaccount_id: &SubaccountId) -> StdResult<Binary> {
    to_json_binary(&querier.query_vanilla_subaccount_position(market_id, subaccount_id)?)
}

pub fn handle_trader_derivative_orders_query(querier: &InjectiveQuerier, market_id: &MarketId, subaccount_id: &SubaccountId) -> StdResult<Binary> {
    to_json_binary(&querier.query_trader_derivative_orders(market_id, subaccount_id)?)
}

pub fn handle_trader_spot_orders_query(querier: &InjectiveQuerier, market_id: &MarketId, subaccount_id: &SubaccountId) -> StdResult<Binary> {
    to_json_binary(&querier.query_trader_spot_orders(market_id, subaccount_id)?)
}

pub fn handle_spot_orders_to_cancel_up_to_amount_query(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    subaccount_id: &SubaccountId,
    base_amount: FPDecimal,
    quote_amount: FPDecimal,
    strategy: CancellationStrategy,
    reference_price: Option<FPDecimal>,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_spot_orders_to_cancel_up_to_amount(
        market_id,
        subaccount_id,
        base_amount,
        quote_amount,
        strategy,
        reference_price,
    )?)
}

pub fn handle_derivative_orders_to_cancel_up_to_amount_query(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    subaccount_id: &SubaccountId,
    quote_amount: FPDecimal,
    strategy: CancellationStrategy,
    reference_price: Option<FPDecimal>,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_derivative_orders_to_cancel_up_to_amount(market_id, subaccount_id, quote_amount, strategy, reference_price)?)
}

pub fn handle_perpetual_market_info_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_perpetual_market_info(market_id)?)
}

pub fn handle_perpetual_market_funding_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_perpetual_market_funding(market_id)?)
}

pub fn handle_market_volatility_query(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    trade_grouping_sec: u64,
    max_age: u64,
    include_raw_history: bool,
    include_metadata: bool,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_market_volatility(market_id, trade_grouping_sec, max_age, include_raw_history, include_metadata)?)
}

pub fn handle_derivative_market_mid_price_and_tob_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_derivative_market_mid_price_and_tob(market_id)?)
}

pub fn handle_aggregate_market_volume_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_aggregate_market_volume(market_id)?)
}

pub fn handle_aggregate_account_volume_query(querier: &InjectiveQuerier, account_id: String) -> StdResult<Binary> {
    to_json_binary(&querier.query_aggregate_account_volume(&account_id)?)
}

pub fn handle_spot_market_mid_price_and_tob_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_spot_market_mid_price_and_tob(market_id)?)
}

pub fn handle_spot_market_orderbook_query(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    side: OrderSide,
    limit_cumulative_quantity: Option<FPDecimal>,
    limit_cumulative_notional: Option<FPDecimal>,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_spot_market_orderbook(market_id, side, limit_cumulative_quantity, limit_cumulative_notional)?)
}

pub fn handle_derivative_market_orderbook_query(
    querier: &InjectiveQuerier,
    market_id: &MarketId,
    limit_cumulative_notional: FPDecimal,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_derivative_market_orderbook(market_id, limit_cumulative_notional)?)
}

pub fn handle_market_atomic_execution_fee_multiplier_query(querier: &InjectiveQuerier, market_id: &MarketId) -> StdResult<Binary> {
    to_json_binary(&querier.query_market_atomic_execution_fee_multiplier(market_id)?)
}

pub fn handle_oracle_volatility_query(
    querier: &InjectiveQuerier,
    base_info: Option<OracleInfo>,
    quote_info: Option<OracleInfo>,
    max_age: u64,
    include_raw_history: bool,
    include_metadata: bool,
) -> StdResult<Binary> {
    to_json_binary(&querier.query_oracle_volatility(&base_info, &quote_info, max_age, include_raw_history, include_metadata)?)
}

pub fn handle_oracle_price_query(querier: &InjectiveQuerier, oracle_type: &OracleType, base: String, quote: String) -> StdResult<Binary> {
    to_json_binary(&querier.query_oracle_price(oracle_type, &base, &quote)?)
}

pub fn handle_pyth_price_query(querier: &InjectiveQuerier, price_id: String) -> StdResult<Binary> {
    to_json_binary(&querier.query_pyth_price(price_id.as_str())?)
}

pub fn handle_token_factory_denom_total_supply(querier: &InjectiveQuerier, denom: String) -> StdResult<Binary> {
    to_json_binary(&querier.query_token_factory_denom_total_supply(&denom)?)
}

pub fn handle_token_factory_creation_fee(querier: &InjectiveQuerier) -> StdResult<Binary> {
    to_json_binary(&querier.query_token_factory_creation_fee()?)
}

pub fn handle_staked_amount_query(querier: &InjectiveQuerier, delegator_address: Addr, max_delegations: u16) -> StdResult<Binary> {
    to_json_binary(&querier.query_staked_amount(delegator_address, max_delegations)?)
}

pub fn handle_contract_registration_info_query(querier: &InjectiveQuerier, contract_address: String) -> StdResult<Binary> {
    to_json_binary(&querier.query_contract_registration_info(&contract_address)?)
}
