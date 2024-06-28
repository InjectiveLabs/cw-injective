use crate::{
    error::ContractError,
    handle::{handle_test_market_spot_order, handle_test_transient_derivative_order, handle_test_transient_spot_order},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{
        handle_aggregate_account_volume_query, handle_aggregate_market_volume_query, handle_contract_registration_info_query,
        handle_derivative_market_mid_price_and_tob_query, handle_derivative_market_orderbook_query, handle_derivative_market_query,
        handle_derivative_orders_to_cancel_up_to_amount_query, handle_effective_subaccount_position_query, handle_exchange_params_query,
        handle_market_atomic_execution_fee_multiplier_query, handle_market_volatility_query, handle_oracle_price_query,
        handle_oracle_volatility_query, handle_perpetual_market_funding_query, handle_perpetual_market_info_query, handle_pyth_price_query,
        handle_spot_market_mid_price_and_tob_query, handle_spot_market_orderbook_query, handle_spot_market_query,
        handle_spot_orders_to_cancel_up_to_amount_query, handle_staked_amount_query, handle_subaccount_deposit_query,
        handle_token_factory_creation_fee, handle_token_factory_denom_total_supply, handle_trader_derivative_orders_query,
        handle_trader_spot_orders_query, handle_vanilla_subaccount_position_query,
    },
    reply::{handle_create_derivative_order_reply, handle_create_order_reply},
};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;
use injective_cosmwasm::{create_deposit_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper};

const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CREATE_SPOT_ORDER_REPLY_ID: u64 = 0u64;
pub const CREATE_DERIVATIVE_ORDER_REPLY_ID: u64 = 1u64;
pub const MSG_EXEC: &str = "/cosmos.authz.v1beta1.MsgExec";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::TestDepositMsg { subaccount_id, amount } => {
            Ok(Response::new().add_message(create_deposit_msg(env.contract.address, subaccount_id, amount)))
        }
        ExecuteMsg::TestTraderTransientSpotOrders {
            market_id,
            subaccount_id,
            price,
            quantity,
        } => handle_test_transient_spot_order(deps, env, &info, market_id, subaccount_id, price, quantity),
        ExecuteMsg::TestTraderTransientDerivativeOrders {
            market_id,
            subaccount_id,
            price,
            quantity,
            margin,
        } => handle_test_transient_derivative_order(deps, env, &info, market_id, subaccount_id, price, quantity, margin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let querier = InjectiveQuerier::new(&deps.querier);
    match msg {
        QueryMsg::TestExchangeParamsQuery {} => handle_exchange_params_query(&querier),
        QueryMsg::TestSubAccountDepositQuery { subaccount_id, denom } => handle_subaccount_deposit_query(&querier, &subaccount_id, denom),
        QueryMsg::TestSpotMarketQuery { market_id } => handle_spot_market_query(&querier, &market_id),
        QueryMsg::TestDerivativeMarketQuery { market_id } => handle_derivative_market_query(&querier, &market_id),
        QueryMsg::TestEffectiveSubaccountPosition { market_id, subaccount_id } => {
            handle_effective_subaccount_position_query(&querier, &market_id, &subaccount_id)
        }
        QueryMsg::TestVanillaSubaccountPosition { market_id, subaccount_id } => {
            handle_vanilla_subaccount_position_query(&querier, &market_id, &subaccount_id)
        }
        QueryMsg::TestTraderDerivativeOrders { market_id, subaccount_id } => {
            handle_trader_derivative_orders_query(&querier, &market_id, &subaccount_id)
        }
        QueryMsg::TestTraderSpotOrders { market_id, subaccount_id } => handle_trader_spot_orders_query(&querier, &market_id, &subaccount_id),
        QueryMsg::TestSpotOrdersToCancelUpToAmount {
            market_id,
            subaccount_id,
            base_amount,
            quote_amount,
            strategy,
            reference_price,
        } => handle_spot_orders_to_cancel_up_to_amount_query(
            &querier,
            &market_id,
            &subaccount_id,
            base_amount,
            quote_amount,
            strategy,
            reference_price,
        ),
        QueryMsg::TestDerivativeOrdersToCancelUpToAmount {
            market_id,
            subaccount_id,
            quote_amount,
            strategy,
            reference_price,
        } => handle_derivative_orders_to_cancel_up_to_amount_query(&querier, &market_id, &subaccount_id, quote_amount, strategy, reference_price),
        QueryMsg::TestPerpetualMarketInfo { market_id } => handle_perpetual_market_info_query(&querier, &market_id),
        QueryMsg::TestPerpetualMarketFunding { market_id } => handle_perpetual_market_funding_query(&querier, &market_id),
        QueryMsg::TestMarketVolatility {
            market_id,
            trade_grouping_sec,
            max_age,
            include_raw_history,
            include_metadata,
        } => handle_market_volatility_query(&querier, &market_id, trade_grouping_sec, max_age, include_raw_history, include_metadata),
        QueryMsg::TestDerivativeMarketMidPriceAndTob { market_id } => handle_derivative_market_mid_price_and_tob_query(&querier, &market_id),
        QueryMsg::TestAggregateMarketVolume { market_id } => handle_aggregate_market_volume_query(&querier, &market_id),
        QueryMsg::TestAggregateAccountVolume { account_id } => handle_aggregate_account_volume_query(&querier, account_id),
        QueryMsg::TestSpotMarketMidPriceAndTob { market_id } => handle_spot_market_mid_price_and_tob_query(&querier, &market_id),
        QueryMsg::TestSpotMarketOrderbook {
            market_id,
            side,
            limit_cumulative_quantity,
            limit_cumulative_notional,
        } => handle_spot_market_orderbook_query(&querier, &market_id, side, limit_cumulative_quantity, limit_cumulative_notional),
        QueryMsg::TestDerivativeMarketOrderbook {
            market_id,
            limit_cumulative_notional,
        } => handle_derivative_market_orderbook_query(&querier, &market_id, limit_cumulative_notional),
        QueryMsg::TestMarketAtomicExecutionFeeMultiplier { market_id } => handle_market_atomic_execution_fee_multiplier_query(&querier, &market_id),
        QueryMsg::TestQueryOracleVolatility {
            base_info,
            quote_info,
            max_age,
            include_raw_history,
            include_metadata,
        } => handle_oracle_volatility_query(&querier, base_info, quote_info, max_age, include_raw_history, include_metadata),
        QueryMsg::TestQueryOraclePrice { oracle_type, base, quote } => handle_oracle_price_query(&querier, &oracle_type, base, quote, None),
        QueryMsg::TestQueryPythPrice { price_id } => handle_pyth_price_query(&querier, price_id),
        QueryMsg::TestQueryStakedAmount {
            delegator_address,
            max_delegations,
        } => handle_staked_amount_query(&querier, deps.api.addr_validate(delegator_address.as_str())?, max_delegations),
        QueryMsg::TestQueryTokenFactoryDenomTotalSupply { denom } => handle_token_factory_denom_total_supply(&querier, denom),
        QueryMsg::TestQueryTokenFactoryCreationFee {} => handle_token_factory_creation_fee(&querier),
        QueryMsg::TestQueryContractRegistrationInfo { contract_address } => handle_contract_registration_info_query(&querier, contract_address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<InjectiveQueryWrapper>, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        CREATE_SPOT_ORDER_REPLY_ID => handle_create_order_reply(deps, &msg),
        CREATE_DERIVATIVE_ORDER_REPLY_ID => handle_create_derivative_order_reply(deps, &msg),
        _ => Err(ContractError::UnrecognizedReply(msg.id)),
    }
}
