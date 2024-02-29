use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, ReplyOn, Reply};
use cw2::set_contract_version;
use injective_cosmwasm::{create_deposit_msg, create_spot_market_order_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper, OrderInfo, OrderType, SpotOrder};


#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use injective_math::FPDecimal;

const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CREATE_SPOT_ORDER_REPLY_ID: u64 = 0u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::TestDepositMsg { subaccount_id, amount } => {
            Ok(Response::new().add_message(create_deposit_msg(env.contract.address, subaccount_id, amount)))
        }
        ExecuteMsg::TestTraderTransientSpotOrders { market_id, subaccount_id } => {
            let order_info = OrderInfo{
                subaccount_id,
                fee_recipient: None,
                price: FPDecimal::must_from_str("1"),
                quantity: FPDecimal::must_from_str("1"),
                cid: None,
            };
            let spot_order = SpotOrder{
                market_id,
                order_info,
                order_type: OrderType::Buy,
                trigger_price: None
            };
            let spot_order_message = create_spot_market_order_msg(info.sender, spot_order);

            let spot_order_message = SubMsg{
                id: CREATE_SPOT_ORDER_REPLY_ID,
                msg: spot_order_message,
                gas_limit: None,
                reply_on: ReplyOn::Success,
            };
            Ok(Response::new().add_submessage(spot_order_message))
        }
        ExecuteMsg::TestTraderTransientDerivativeOrders { market_id, subaccount_id } => {
            // to_json_binary(&querier.query_trader_transient_derivative_orders(&market_id, &subaccount_id)?)
            Ok(Default::default())
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


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        CREATE_SPOT_ORDER_REPLY_ID => {
            deps.api.debug("I am here");
            Ok(Default::default())
        },
        _ => Err(ContractError::UnrecognizedReply(msg.id)),
    }
}
