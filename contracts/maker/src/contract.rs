use crate::derivative::{base_deriv, head_to_tail_deriv, inv_imbalance_deriv, tail_to_head_deriv};
use crate::error::ContractError;
use crate::exchange::{
    Deposit, DerivativeLimitOrder, DerivativeMarket, DerivativeOrder, OrderData, OrderInfo, PerpetualMarketFunding, PerpetualMarketInfo, Position,
    WrappedDerivativeLimitOrder, WrappedDerivativeMarket, WrappedPosition,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, WrappedGetActionResponse};
use crate::risk_management::{check_tail_dist, get_alloc_bal_new_orders, only_owner};
use crate::state::{config, config_read, State};
use crate::utils::{bp_to_dec, div_dec, sub_abs, wrap};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, CosmosMsg, Decimal256 as Decimal, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, Uint128, Uint256, WasmMsg,
};
use std::str::FromStr;

use cw20::{Cw20ExecuteMsg, MinterResponse};
use injective_bindings::{create_batch_update_orders_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper};

use crate::exchange::{ExchangeMsg, MsgBatchUpdateOrders};
use cw0::parse_reply_instantiate_data;

const INSTANTIATE_REPLY_ID: u64 = 1u64;

#[entry_point]
pub fn instantiate(deps: DepsMut<InjectiveQueryWrapper>, env: Env, _info: MessageInfo, msg: InstantiateMsg) -> Result<Response, StdError> {
    let state = State {
        market_id: msg.market_id.to_string(),
        subaccount_id: msg.subaccount_id,
        fee_recipient: msg.fee_recipient,
        is_deriv: msg.is_deriv,
        leverage: Decimal::from_str(&msg.leverage).unwrap(),
        order_density: Uint256::from_str(&msg.order_density).unwrap(),
        max_market_data_delay: msg.max_market_data_delay.parse::<i64>().unwrap(),
        reservation_param: Decimal::from_str(&msg.reservation_param).unwrap(),
        spread_param: Decimal::from_str(&msg.spread_param).unwrap(),
        active_capital: Decimal::from_str(&msg.active_capital).unwrap(),
        head_chg_tol: bp_to_dec(Decimal::from_str(&msg.head_chg_tol_bp).unwrap()),
        tail_dist_from_mid: bp_to_dec(Decimal::from_str(&msg.tail_dist_from_mid_bp).unwrap()),
        min_tail_dist: bp_to_dec(Decimal::from_str(&msg.min_tail_dist_bp).unwrap()),
        lp_token_address: "LP-Token".to_string(),
    };

    config(deps.storage).save(&state)?;

    let code_id = msg.cw20_code_id.parse::<u64>().unwrap();
    let decimals = msg.lp_decimals.parse::<u8>().unwrap();

    let cw20_instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: msg.lp_name,
        symbol: msg.lp_symbol,
        decimals: decimals,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: env.contract.address.to_string(),
            cap: None,
        }),
        marketing: None,
    };

    let instantiate_message = WasmMsg::Instantiate {
        admin: None,
        code_id,
        msg: to_binary(&cw20_instantiate_msg)?,
        funds: vec![],
        label: "LP-Token".to_string(),
    };

    let submessage = SubMsg::reply_on_success(instantiate_message, INSTANTIATE_REPLY_ID);
    Ok(Response::new().add_submessage(submessage))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg).unwrap();
    let child_contract = deps.api.addr_validate(&res.contract_address)?;

    let mut state = config_read(deps.storage).load().unwrap();
    state.lp_token_address = child_contract.to_string();

    config(deps.storage).save(&state)?;

    return Ok(Response::default());
}

#[entry_point]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::MintToUser {
            subaccount_id_sender,
            amount,
        } => mint_to_user(deps, env, info.sender, subaccount_id_sender, amount),
        ExecuteMsg::BurnFromUser {
            subaccount_id_sender,
            amount,
        } => burn_from_user(deps, env, info.sender, subaccount_id_sender, amount),
        ExecuteMsg::BeginBlocker {} => begin_blocker(deps, env, info.sender),
    }
}

pub fn mint_to_user(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    subaccount_id_sender: String,
    amount: Uint128,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let state = config_read(deps.storage).load().unwrap();
    let lp_token_address = state.lp_token_address.clone();

    // Ensure that only exchange module calls this method
    only_owner(&env.contract.address, &sender);

    let mint = Cw20ExecuteMsg::Mint {
        recipient: subaccount_id_sender,
        amount: amount,
    };
    let message = WasmMsg::Execute {
        contract_addr: lp_token_address.into(),
        msg: to_binary(&mint)?,
        funds: vec![],
    };

    Ok(Response::new().add_message(message))
}

pub fn burn_from_user(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    sender: Addr,
    subaccount_id_sender: String,
    amount: Uint128,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let state = config_read(deps.storage).load().unwrap();
    let lp_token_address = state.lp_token_address.clone();

    // Ensure that only exchange module calls this method
    only_owner(&env.contract.address, &sender);

    let burn = Cw20ExecuteMsg::BurnFrom {
        owner: subaccount_id_sender,
        amount: amount,
    };
    let message = WasmMsg::Execute {
        contract_addr: lp_token_address.into(),
        msg: to_binary(&burn)?,
        funds: vec![],
    };

    Ok(Response::new().add_message(message))
}

// Pass via get_action instead
/// This is an external, state changing method that will give the bot a more accurate perspective of the
/// current state of markets. It updates the volatility and the mid_price properties on the state struct.
/// The method should be called on some repeating interval.
// pub fn update_market_state(
//     deps: DepsMut<InjectiveQueryWrapper>,
//     env: Env,
//     info: MessageInfo,
//     mid_price: String,
//     volatility: String,
// ) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
//     let mut state = config(deps.storage).load().unwrap();

//     // Ensure that only the contract creator has permission to update market data
//     only_owner(&env.contract.address, &info.sender);

//     // Update the mid price
//     state.mid_price = Decimal::from_str(&mid_price).unwrap();

//     // Update the volatility
//     state.volatility = Decimal::from_str(&volatility).unwrap();

//     // Update the timestamp of this most recent update
//     let time_of_update = Utc::now().timestamp();
//     state.last_update_utc = time_of_update;

//     let res = Response::new();
//     Ok(res)
// }

pub fn begin_blocker(deps: DepsMut<InjectiveQueryWrapper>, env: Env, sender: Addr) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let state = config(deps.storage).load().unwrap();

    // Ensure that only exchange module calls this method
    only_owner(&env.contract.address, &sender);

    let querier = InjectiveQuerier::new(&deps.querier);
    let market_res = querier.query_derivative_market(state.market_id.clone())?;
    let deposit_res = querier.query_subaccount_deposit(state.subaccount_id.clone(), market_res.market.market.quote_denom.clone())?;
    let positions_res = querier.query_subaccount_positions(state.subaccount_id.clone())?;
    let open_orders_res = querier.query_trader_derivative_orders(state.market_id.clone(), state.subaccount_id.clone())?;
    let perpetual_market_info_res = querier.query_perpetual_market_info(state.market_id.clone())?;
    let perpetual_market_funding_res = querier.query_perpetual_market_funding(state.market_id.clone())?;

    let market: DerivativeMarket = DerivativeMarket {
        ticker: market_res.market.market.ticker,
        oracle_base: market_res.market.market.oracle_base,
        oracle_quote: market_res.market.market.oracle_quote,
        oracle_type: market_res.market.market.oracle_type,
        oracle_scale_factor: market_res.market.market.oracle_scale_factor,
        quote_denom: market_res.market.market.quote_denom,
        market_id: market_res.market.market.market_id,
        initial_margin_ratio: market_res.market.market.initial_margin_ratio.to_string(),
        maintenance_margin_ratio: market_res.market.market.maintenance_margin_ratio.to_string(),
        maker_fee_rate: market_res.market.market.maker_fee_rate.to_string(),
        taker_fee_rate: market_res.market.market.taker_fee_rate.to_string(),
        isPerpetual: market_res.market.market.isPerpetual,
        status: market_res.market.market.status,
        min_price_tick_size: market_res.market.market.min_price_tick_size.to_string(),
        min_quantity_tick_size: market_res.market.market.min_quantity_tick_size.to_string(),
    };
    let open_orders_res_val = open_orders_res.orders.unwrap_or_default();
    let open_orders = open_orders_res_val
        .into_iter()
        .map(|order| {
            // OrderType_BUY         OrderType = 1
            // OrderType_SELL        OrderType = 2
            let order_type = if order.isBuy { 1 } else { 2 };
            let limit_order: DerivativeLimitOrder = DerivativeLimitOrder {
                margin: order.margin.to_string(),
                fillable: order.fillable.to_string(),
                order_hash: order.order_hash,
                trigger_price: None,
                order_type: order_type,
                order_info: OrderInfo {
                    subaccount_id: state.subaccount_id.clone(),
                    fee_recipient: state.fee_recipient.clone(),
                    price: order.price.to_string(),
                    quantity: order.quantity.to_string(),
                },
            };
            limit_order
        })
        .collect::<Vec<DerivativeLimitOrder>>();

    let deposit = Deposit {
        available_balance: deposit_res.deposits.available_balance.to_string(),
        total_balance: deposit_res.deposits.total_balance.to_string(),
    };

    // TODO change for multi market support
    let first_position_query = positions_res.state.get(0);
    let first_position: Option<Position> = if first_position_query.is_none() {
        None
    } else {
        let position = first_position_query.unwrap().position.clone();
        Some(Position {
            isLong: position.isLong,
            quantity: position.quantity.clone(),
            margin: position.margin.clone(),
            entry_price: position.entry_price.clone(),
            cumulative_funding_entry: position.cumulative_funding_entry.clone(),
        })
    };

    let perpetual_market_info: Option<PerpetualMarketInfo> = if perpetual_market_info_res.info.is_none() {
        None
    } else {
        let info = perpetual_market_info_res.info.unwrap();
        Some(PerpetualMarketInfo {
            market_id: info.market_id,
            hourly_funding_rate_cap: info.hourly_funding_rate_cap.to_string(),
            hourly_interest_rate: info.hourly_interest_rate.to_string(),
            next_funding_timestamp: info.next_funding_timestamp,
            funding_interval: info.funding_interval,
        })
    };
    let perpetual_market_funding: Option<PerpetualMarketFunding> = if perpetual_market_funding_res.state.is_none() {
        None
    } else {
        let funding = perpetual_market_funding_res.state.unwrap();
        Some(PerpetualMarketFunding {
            cumulative_funding: funding.cumulative_funding.to_string(),
            cumulative_price: funding.cumulative_price.to_string(),
            last_timestamp: funding.last_timestamp,
        })
    };

    let action_response = get_action(
        deps,
        env,
        market,
        perpetual_market_info,
        perpetual_market_funding,
        open_orders,
        deposit,
        first_position,
        market_res.market.mark_price.to_string(),
        String::from("80000"), // TODO
        market_res.market.mark_price.to_string(),
    );

    let msgs = match action_response {
        Ok(v) => v.msgs,
        Err(_) => todo!(),
    };
    let parsed_msgs = msgs.iter().map(|msg| match msg {
        ExchangeMsg::BatchUpdateOrders(batch_update_orders_msg) => create_batch_update_orders_msg(
            batch_update_orders_msg.sender.clone(),
            batch_update_orders_msg.subaccount_id.clone(),
            batch_update_orders_msg.spot_market_ids_to_cancel_all.clone(),
            batch_update_orders_msg.derivative_market_ids_to_cancel_all.clone(),
            serde_json_wasm::from_str(&serde_json_wasm::to_string(&batch_update_orders_msg.spot_orders_to_cancel).unwrap()).unwrap(),
            serde_json_wasm::from_str(&serde_json_wasm::to_string(&batch_update_orders_msg.derivative_orders_to_cancel).unwrap()).unwrap(),
            serde_json_wasm::from_str(&serde_json_wasm::to_string(&batch_update_orders_msg.spot_orders_to_create).unwrap()).unwrap(),
            serde_json_wasm::from_str(&serde_json_wasm::to_string(&batch_update_orders_msg.derivative_orders_to_create).unwrap()).unwrap(),
        ),
        ExchangeMsg::MsgCreateDerivativeMarketOrder => todo!(),
    });

    let mut messages: Vec<CosmosMsg<InjectiveMsgWrapper>> = Vec::new();
    parsed_msgs.for_each(|msg| messages.push(msg));

    Ok(Response::new().set_data(Binary::from(b"just some test data")).add_messages(messages))
}

#[entry_point]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        // QueryMsg::GetTotalLPSupply {} => to_binary(&get_total_lp_supply(deps)?),
    }
}

// fn get_total_lp_supply(deps: Deps) -> StdResult<TotalSupplyResponse> {
//     let state = config_read(deps.storage).load().unwrap();
//     let lp_token_address = state.lp_token_address.clone();

//     let token_info_query = Cw20QueryMsg::TokenInfo {};
//     let cw20_token_info_response = WasmQuery::Smart {
//         contract_addr: lp_token_address,
//         msg: to_binary(&token_info_query)?,
//     };

//     Ok(TotalSupplyResponse { total_supply: 123 })
// }

fn get_action(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    market: DerivativeMarket,
    _perpetual_market_info: Option<PerpetualMarketInfo>,
    _perpetual_market_funding: Option<PerpetualMarketFunding>,
    open_orders: Vec<DerivativeLimitOrder>,
    deposit: Deposit,
    position: Option<Position>,
    _oracle_price: String,
    volatility: String,
    mid_price: String,
) -> StdResult<WrappedGetActionResponse> {
    // Wrap everything
    let open_orders: Vec<WrappedDerivativeLimitOrder> = open_orders.into_iter().map(|o| o.wrap().unwrap()).collect();
    let position = match position {
        None => None,
        Some(p) => Some(p.wrap().unwrap()),
    };
    let inv_val = wrap(&deposit.total_balance);
    let market = market.wrap().unwrap();

    // Update the mid price
    let mid_price = Decimal::from_str(&mid_price).unwrap();

    // Update the volatility
    let volatility = Decimal::from_str(&volatility).unwrap();

    // Load state
    let state = config_read(deps.storage).load().unwrap();

    // Calculate inventory imbalance parameter
    let (inv_imbal, imbal_is_long) = inv_imbalance_deriv(&position, inv_val);

    // Calculate reservation price
    let reservation_price = reservation_price(mid_price, inv_imbal, volatility, state.reservation_param, imbal_is_long);

    // Calculate the new head prices
    let (new_buy_head, new_sell_head) = new_head_prices(volatility, reservation_price, state.spread_param);

    // Split open orders
    let (open_buys, open_sells) = split_open_orders(open_orders);

    // Ensure that the heads have changed enough that we are willing to make an action
    if head_chg_is_gt_tol(&open_buys, new_buy_head, state.head_chg_tol) || head_chg_is_gt_tol(&open_sells, new_sell_head, state.head_chg_tol) {
        // Get new tails
        let (new_buy_tail, new_sell_tail) = new_tail_prices(new_buy_head, new_sell_head, mid_price, state.tail_dist_from_mid, state.min_tail_dist);

        // Get information for buy order creation/cancellation
        let (mut buy_orders_to_cancel, buy_orders_to_keep, buy_orders_remaining_val, buy_append_to_new_head) =
            orders_to_cancel(open_buys, new_buy_head, new_buy_tail, true, &state);

        // Get information for sell order creation/cancellation
        let (mut sell_orders_to_cancel, sell_orders_to_keep, sell_orders_remaining_val, sell_append_to_new_head) =
            orders_to_cancel(open_sells, new_sell_head, new_sell_tail, false, &state);

        // Get new buy/sell orders
        let buy_orders_to_open = create_orders(
            new_buy_head,
            new_buy_tail,
            inv_val,
            buy_orders_to_keep,
            buy_orders_remaining_val,
            position.clone(),
            buy_append_to_new_head,
            &mut buy_orders_to_cancel,
            true,
            &state,
            &market,
        );
        let sell_orders_to_open = create_orders(
            new_sell_head,
            new_sell_tail,
            inv_val,
            sell_orders_to_keep,
            sell_orders_remaining_val,
            position,
            sell_append_to_new_head,
            &mut sell_orders_to_cancel,
            false,
            &state,
            &market,
        );

        let batch_order = MsgBatchUpdateOrders {
            sender: env.contract.address.to_string(),
            subaccount_id: state.subaccount_id,
            spot_market_ids_to_cancel_all: Vec::new(),
            derivative_market_ids_to_cancel_all: vec![],
            spot_orders_to_cancel: Vec::new(),
            derivative_orders_to_cancel: vec![buy_orders_to_cancel, sell_orders_to_cancel].concat(),
            spot_orders_to_create: Vec::new(),
            derivative_orders_to_create: vec![buy_orders_to_open, sell_orders_to_open].concat(),
        };
        Ok(WrappedGetActionResponse {
            msgs: vec![ExchangeMsg::BatchUpdateOrders(batch_order)],
        })
    } else {
        Ok(WrappedGetActionResponse { msgs: Vec::new() })
    }
}

pub fn orders_to_cancel(
    open_orders: Vec<WrappedDerivativeLimitOrder>,
    new_head: Decimal,
    new_tail: Decimal,
    is_buy: bool,
    state: &State,
) -> (Vec<OrderData>, Vec<WrappedDerivativeLimitOrder>, Decimal, bool) {
    let mut orders_remaining_val = Decimal::zero();
    let mut orders_to_cancel: Vec<OrderData> = Vec::new();
    // If there are any open orders, we need to check them to see if we should cancel
    if open_orders.len() > 0 {
        // Use the new tail/head to filter out the orders to cancel
        let orders_to_keep: Vec<WrappedDerivativeLimitOrder> = open_orders
            .into_iter()
            .filter(|o| {
                let keep_if_buy = o.order_info.price <= new_head && o.order_info.price >= new_tail;
                let keep_if_sell = o.order_info.price >= new_head && o.order_info.price <= new_tail;
                let keep = (keep_if_buy && is_buy) || (keep_if_sell && !is_buy);
                if keep {
                    orders_remaining_val = orders_remaining_val + (o.order_info.price * o.order_info.quantity);
                } else {
                    orders_to_cancel.push(OrderData::new(&o, state));
                }
                keep
            })
            .collect();
        // Determine if we need to append to new orders to the new head or if we need to
        // append to the end of the block of orders we will be keeping
        let append_to_new_head =
            sub_abs(new_head, orders_to_keep.first().unwrap().order_info.price) > sub_abs(orders_to_keep.last().unwrap().order_info.price, new_tail);
        (orders_to_cancel, orders_to_keep, orders_remaining_val, append_to_new_head)
    } else {
        (orders_to_cancel, Vec::new(), orders_remaining_val, true)
    }
}

fn create_orders(
    new_head: Decimal,
    new_tail: Decimal,
    inv_val: Decimal,
    orders_to_keep: Vec<WrappedDerivativeLimitOrder>,
    orders_remaining_val: Decimal,
    position: Option<WrappedPosition>,
    append_to_new_head: bool,
    orders_to_cancel: &mut Vec<OrderData>,
    is_buy: bool,
    state: &State,
    market: &WrappedDerivativeMarket,
) -> Vec<DerivativeOrder> {
    let (position_qty, position_margin) = match position {
        Some(position) => {
            if position.is_long == is_buy {
                (Decimal::zero(), position.margin)
            } else {
                (position.quantity, Decimal::zero())
            }
        }
        None => (Decimal::zero(), Decimal::zero()),
    };
    let alloc_val_for_new_orders = get_alloc_bal_new_orders(inv_val, position_margin, state.active_capital) - orders_remaining_val;
    if orders_to_keep.len() == 0 {
        let (new_orders, _) = base_deriv(
            new_head,
            new_tail,
            alloc_val_for_new_orders,
            orders_to_keep.len(),
            position_qty,
            true,
            is_buy,
            &state,
            market,
        );
        new_orders
    } else if append_to_new_head {
        let (new_orders, mut additional_hashes_to_cancel) =
            tail_to_head_deriv(new_head, alloc_val_for_new_orders, orders_to_keep, position_qty, is_buy, &state, market);
        orders_to_cancel.append(&mut additional_hashes_to_cancel);
        new_orders
    } else {
        let (new_orders, mut additional_hashes_to_cancel) =
            head_to_tail_deriv(new_tail, alloc_val_for_new_orders, orders_to_keep, position_qty, is_buy, &state, market);
        orders_to_cancel.append(&mut additional_hashes_to_cancel);
        new_orders
    }
}

/// Uses the inventory imbalance to calculate a price around which we will center the mid price
/// # Arguments
/// * `mid_price` - A mid_price that we update on a block by block basis
/// * `inv_imbal` - A measure of inventory imbalance
/// * `volatility` - A measure of volatility that we update on a block by block basis
/// * `reservation_param` - The constant to control the sensitivity of the volatility param
/// * `imbal_is_long` - The direction of the inventory imbalance
/// # Returns
/// * `reservation_price` - The price around which we will center both heads
fn reservation_price(mid_price: Decimal, inv_imbal: Decimal, volatility: Decimal, reservation_param: Decimal, imbal_is_long: bool) -> Decimal {
    if inv_imbal == Decimal::zero() {
        mid_price
    } else {
        if imbal_is_long {
            mid_price - (inv_imbal * volatility * reservation_param)
        } else {
            mid_price + (inv_imbal * volatility * reservation_param)
        }
    }
}

/// Uses the reservation price and variation to calculate where the buy/sell heads should be. Both buy and
/// sell heads will be equi-distant from the reservation price.
/// # Arguments
/// * `volatility` - A measure of volatility that we update on a block by block basis
/// * `reservation_price` - The a price that is shifted from the mid price depending on the inventory imbalance
/// * `spread_param` - The constant to control the sensitivity of the spread
/// # Returns
/// * `buy_head` - The new buy head
/// * `sell_head` - The new sell head
fn new_head_prices(volatility: Decimal, reservation_price: Decimal, spread_param: Decimal) -> (Decimal, Decimal) {
    let dist_from_reservation_price = div_dec(volatility * spread_param, Decimal::from_str("2").unwrap());
    (
        reservation_price - dist_from_reservation_price,
        reservation_price + dist_from_reservation_price,
    )
}

/// Determines whether the new head for the next block will be different enough than that of the previous to warrent
/// order cancellation and the creation of new orders.
/// # Arguments
/// * `open_orders` - The buy or sell side orders from the previous block
/// * `new_head` - The new proposed buy or sell head
/// * `head_chg_tol` - Our tolerance to change in the head price
/// # Returns
/// * `should_change` - Whether we should cancel and place new orders at the new head
fn head_chg_is_gt_tol(open_orders: &Vec<WrappedDerivativeLimitOrder>, new_head: Decimal, head_chg_tol: Decimal) -> bool {
    if open_orders.len() == 0 {
        true
    } else {
        let old_head = open_orders.first().unwrap().order_info.price;
        div_dec(sub_abs(old_head, new_head), old_head) > head_chg_tol
    }
}

/// Calculates the correct tail prices for buy and sell sides based off of the mid price. If either of
/// the distances between the head and tail fall below the minimum spread defined at initialization, risk
/// manager returns a tail that meets the minimum spread constraint.
/// # Arguments
/// * `buy_head` - The new buy head
/// * `sell_head` - The new sell head
/// * `mid_price` - A mid_price that we update on a block by block basis
/// * `tail_dist_from_mid` - The distance from the mid price, in either direction, that we want tails to be located (between 0..1)
/// * `min_tail_dist` - The min distance that can exist between any head and tail (between 0..1)
/// # Returns
/// * `buy_tail` - The new buy tail
/// * `sell_tail` - The new sell tail
pub fn new_tail_prices(
    buy_head: Decimal,
    sell_head: Decimal,
    mid_price: Decimal,
    tail_dist_from_mid: Decimal,
    min_tail_dist: Decimal,
) -> (Decimal, Decimal) {
    let proposed_buy_tail = mid_price * (Decimal::one() - tail_dist_from_mid);
    let proposed_sell_tail = mid_price * (Decimal::one() + tail_dist_from_mid);
    check_tail_dist(buy_head, sell_head, proposed_buy_tail, proposed_sell_tail, min_tail_dist)
}

/// Splits the vec of orders to buyside and sellside orders. Sorts them so that the head from the previous block is at index == 0. Buyside
/// orders are sorted desc. Sellside are sorted asc.
/// # Arguments
/// * `open_orders` - The open orders from both sides that were on the book as of the preceeding block
/// # Returns
/// * `buy_orders` - The sorted buyside orders
/// * `sell_orders` - The sorted sellside orders
fn split_open_orders(open_orders: Vec<WrappedDerivativeLimitOrder>) -> (Vec<WrappedDerivativeLimitOrder>, Vec<WrappedDerivativeLimitOrder>) {
    if open_orders.len() > 0 {
        let mut buy_orders: Vec<WrappedDerivativeLimitOrder> = Vec::new();
        let mut sell_orders: Vec<WrappedDerivativeLimitOrder> = open_orders
            .into_iter()
            .filter(|o| {
                if o.order_type == 1 {
                    buy_orders.push(o.clone());
                }
                o.order_type == 2
            })
            .collect();

        // Sort both so the head is at index 0
        buy_orders.sort_by(|a, b| b.order_info.price.partial_cmp(&a.order_info.price).unwrap());
        sell_orders.sort_by(|a, b| a.order_info.price.partial_cmp(&b.order_info.price).unwrap());

        (buy_orders, sell_orders)
    } else {
        (Vec::new(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::exchange::{WrappedDerivativeLimitOrder, WrappedOrderInfo};

    use super::{head_chg_is_gt_tol, new_tail_prices, split_open_orders};
    use cosmwasm_std::Decimal256 as Decimal;
    use std::str::FromStr;

    #[test]
    fn head_chg_is_gt_tol_test() {
        let mut open_orders: Vec<WrappedDerivativeLimitOrder> = Vec::new();
        let new_head = Decimal::from_str("100000000100").unwrap();
        let head_chg_tol = Decimal::from_str("0.01").unwrap();
        let should_change = head_chg_is_gt_tol(&open_orders, new_head, head_chg_tol);
        assert!(should_change);

        open_orders.push(WrappedDerivativeLimitOrder {
            fillable: Default::default(),
            margin: Default::default(),
            order_info: WrappedOrderInfo {
                subaccount_id: String::from(""),
                fee_recipient: String::from(""),
                price: Decimal::from_str("100000000000").unwrap(),
                quantity: Decimal::zero(),
            },
            order_type: 1,
            trigger_price: None,
            order_hash: String::from(""),
        });

        let should_change = head_chg_is_gt_tol(&open_orders, new_head, head_chg_tol);
        assert!(!should_change);

        open_orders.pop();
        open_orders.push(WrappedDerivativeLimitOrder {
            fillable: Default::default(),
            margin: Default::default(),
            order_info: WrappedOrderInfo {
                subaccount_id: String::from(""),
                fee_recipient: String::from(""),
                price: Decimal::from_str("110000000000").unwrap(),
                quantity: Decimal::zero(),
            },
            order_type: 1,
            trigger_price: None,
            order_hash: String::from(""),
        });

        let should_change = head_chg_is_gt_tol(&open_orders, new_head, head_chg_tol);
        assert!(should_change);

        let should_change = head_chg_is_gt_tol(&Vec::new(), new_head, Decimal::from_str("1").unwrap());
        assert!(should_change);
    }

    #[test]
    fn new_tail_prices_test() {
        let buy_head = Decimal::from_str("3999").unwrap();
        let mid_price = Decimal::from_str("4000").unwrap();
        let sell_head = Decimal::from_str("4001").unwrap();
        let tail_dist_from_mid = Decimal::from_str("0.05").unwrap();
        let min_tail_dist = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = new_tail_prices(buy_head, sell_head, mid_price, tail_dist_from_mid, min_tail_dist);
        assert_eq!(buy_tail, mid_price * (Decimal::one() - tail_dist_from_mid));
        assert_eq!(sell_tail, mid_price * (Decimal::one() + tail_dist_from_mid));

        let tail_dist_from_mid = Decimal::from_str("0.001").unwrap();
        let min_tail_dist = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = new_tail_prices(buy_head, sell_head, mid_price, tail_dist_from_mid, min_tail_dist);
        assert_eq!(buy_tail, buy_head * (Decimal::one() - min_tail_dist));
        assert_eq!(sell_tail, sell_head * (Decimal::one() + min_tail_dist));
    }

    #[test]
    fn split_open_orders_test() {
        let mut open_orders: Vec<WrappedDerivativeLimitOrder> = Vec::new();
        let order = WrappedDerivativeLimitOrder {
            fillable: Default::default(),
            margin: Default::default(),
            order_info: WrappedOrderInfo {
                subaccount_id: String::from(""),
                fee_recipient: String::from(""),
                price: Decimal::from_str("110000000000").unwrap(),
                quantity: Decimal::zero(),
            },
            order_type: 1,
            trigger_price: None,
            order_hash: String::from(""),
        };
        let mut buy = order.clone();
        let mut sell = order.clone();
        for _ in 0..25 {
            sell.order_type = 2;
            buy.order_info.price = buy.order_info.price + Decimal::one();
            sell.order_info.price = sell.order_info.price + Decimal::one();
            open_orders.push(buy.clone());
            open_orders.push(sell.clone());
        }
        let (buy_orders, sell_orders) = split_open_orders(open_orders);
        for i in 1..25 {
            assert!(buy_orders[i].order_info.price < buy_orders[i - 1].order_info.price); // These should be decreasing
            assert!(sell_orders[i].order_info.price > sell_orders[i - 1].order_info.price);
            // These should be increasing
        }
    }
}
