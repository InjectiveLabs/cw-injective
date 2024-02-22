use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use injective_cosmwasm::{create_deposit_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::TestDepositMsg { subaccount_id, amount } => {
            Ok(Response::new().add_message(create_deposit_msg(env.contract.address, subaccount_id, amount)))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let querier: InjectiveQuerier = InjectiveQuerier::new(&deps.querier);

    match msg {
        QueryMsg::TestExchangeParamsQuery {} => to_json_binary(&querier.query_exchange_params()?),
        QueryMsg::TestSubAccountDepositQuery { subaccount_id, denom } => to_json_binary(&querier.query_subaccount_deposit(&subaccount_id, &denom)?),
        QueryMsg::TestSpotMarketQuery { market_id } => to_json_binary(&querier.query_spot_market(&market_id)?),
        QueryMsg::TestDerivativeMarketQuery { market_id } => to_json_binary(&querier.query_derivative_market(&market_id)?),
        QueryMsg::TestEffectiveSubaccountPosition { market_id, subaccount_id } => {
            to_json_binary(&querier.query_effective_subaccount_position(&market_id, &subaccount_id)?)
        }
        QueryMsg::TestVanillaSubaccountPosition { market_id, subaccount_id } => {
            to_json_binary(&querier.query_vanilla_subaccount_position(&market_id, &subaccount_id)?)
        }
        QueryMsg::TestTraderDerivativeOrders { market_id, subaccount_id } => {
            to_json_binary(&querier.query_trader_derivative_orders(&market_id, &subaccount_id)?)
        }
        QueryMsg::TestTraderTransientSpotOrders { market_id, subaccount_id } => {
            to_json_binary(&querier.query_trader_transient_spot_orders(&market_id, &subaccount_id)?)
        }
        QueryMsg::TestTraderTransientDerivativeOrders { market_id, subaccount_id } => {
            to_json_binary(&querier.query_trader_transient_derivative_orders(&market_id, &subaccount_id)?)
        }
        QueryMsg::TestTraderSpotOrders { market_id, subaccount_id } => to_json_binary(&querier.query_trader_spot_orders(&market_id, &subaccount_id)?),
        QueryMsg::TestSpotOrdersToCancelUpToAmount {
            market_id,
            subaccount_id,
            base_amount,
            quote_amount,
            strategy,
            reference_price,
        } => to_json_binary(&querier.query_spot_orders_to_cancel_up_to_amount(
            &market_id,
            &subaccount_id,
            base_amount,
            quote_amount,
            strategy,
            reference_price,
        )?),
        QueryMsg::TestDerivativeOrdersToCancelUpToAmount {
            market_id,
            subaccount_id,
            quote_amount,
            strategy,
            reference_price,
        } => to_json_binary(&querier.query_derivative_orders_to_cancel_up_to_amount(
            &market_id,
            &subaccount_id,
            quote_amount,
            strategy,
            reference_price,
        )?),
        QueryMsg::TestPerpetualMarketInfo { market_id } => to_json_binary(&querier.query_perpetual_market_info(&market_id)?),
        QueryMsg::TestPerpetualMarketFunding { market_id } => to_json_binary(&querier.query_perpetual_market_funding(&market_id)?),
        QueryMsg::TestMarketVolatility {
            market_id,
            trade_grouping_sec,
            max_age,
            include_raw_history,
            include_metadata,
        } => to_json_binary(&querier.query_market_volatility(&market_id, trade_grouping_sec, max_age, include_raw_history, include_metadata)?),
        QueryMsg::TestDerivativeMarketMidPriceAndTob { market_id } => to_json_binary(&querier.query_derivative_market_mid_price_and_tob(&market_id)?),
        QueryMsg::TestAggregateMarketVolume { market_id } => to_json_binary(&querier.query_aggregate_market_volume(&market_id)?),
        QueryMsg::TestAggregateAccountVolume { account_id } => to_json_binary(&querier.query_aggregate_account_volume(&account_id)?),
        QueryMsg::TestSpotMarketMidPriceAndTob { market_id } => to_json_binary(&querier.query_spot_market_mid_price_and_tob(&market_id)?),
        QueryMsg::TestSpotMarketOrderbook {
            market_id,
            side,
            limit_cumulative_quantity,
            limit_cumulative_notional,
        } => to_json_binary(&querier.query_spot_market_orderbook(&market_id, side, limit_cumulative_quantity, limit_cumulative_notional)?),
        QueryMsg::TestDerivativeMarketOrderbook {
            market_id,
            limit_cumulative_notional,
        } => to_json_binary(&querier.query_derivative_market_orderbook(&market_id, limit_cumulative_notional)?),
        QueryMsg::TestMarketAtomicExecutionFeeMultiplier { market_id } => {
            to_json_binary(&querier.query_market_atomic_execution_fee_multiplier(&market_id)?)
        }
    }
}
