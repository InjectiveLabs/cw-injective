use crate::msg::MSG_CREATE_SPOT_MARKET_ORDER_ENDPOINT;
use crate::order_management::{create_spot_market_order, create_stargate_msg};
use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    types,
};

use cosmos_sdk_proto::{cosmos::authz::v1beta1::MsgExec, traits::Message, Any};
use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdResult, SubMsg};
use cw2::set_contract_version;
use injective_cosmwasm::{create_deposit_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper, OrderType};
use injective_math::FPDecimal;
use prost::Message;

const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CREATE_SPOT_ORDER_REPLY_ID: u64 = 0u64;

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
        ExecuteMsg::TestTraderTransientSpotOrders { market_id, subaccount_id } => {
            let querier: InjectiveQuerier = InjectiveQuerier::new(&deps.querier);
            let spot_market = querier.query_spot_market(&market_id).unwrap().market.unwrap();

            deps.api.debug(&info.sender.as_str());
            let order_msg = create_spot_market_order(
                FPDecimal::must_from_str("1"),
                FPDecimal::must_from_str("1"),
                OrderType::Buy,
                &info.sender.as_str(),
                subaccount_id.as_str(),
                &spot_market,
            );

            let mut order_bytes = vec![];
            types::MsgCreateSpotMarketOrder::encode(&order_msg, &mut order_bytes).unwrap();

            let msg_exec = MsgExec {
                grantee: env.contract.address.to_string(),
                msgs: vec![Any {
                    type_url: MSG_CREATE_SPOT_MARKET_ORDER_ENDPOINT.to_string(),
                    value: order_bytes,
                }],
            };

            let order_submessage = SubMsg::reply_on_success(
                create_stargate_msg(MSG_EXEC, msg_exec.encode_to_vec().into()).unwrap(),
                CREATE_SPOT_ORDER_REPLY_ID,
            );

            Ok(Response::new().add_submessage(order_submessage))
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
pub fn reply(deps: DepsMut<InjectiveQueryWrapper>, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        CREATE_SPOT_ORDER_REPLY_ID => {
            deps.api.debug("I am here");
            Ok(Default::default())
        }
        _ => Err(ContractError::UnrecognizedReply(msg.id)),
    }
}
