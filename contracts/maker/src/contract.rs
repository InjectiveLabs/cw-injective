use crate::derivative::{
    create_orders_between_bounds_deriv, create_orders_head_to_tail_deriv, create_orders_tail_to_head_deriv, inventory_imbalance_deriv,
};
use crate::error::ContractError;
use crate::exchange::{
    Deposit, DerivativeLimitOrder, DerivativeMarket, DerivativeOrder, OrderData, OrderInfo, PerpetualMarketFunding, PerpetualMarketInfo, Position,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MarketIdResponse, QueryMsg, TotalSupplyResponse};
use crate::risk_management::{check_tail_dist, only_owner, total_marginable_balance_for_new_orders};
use crate::state::{config, config_read, State};
use crate::utils::{decode_bech32, div_dec, sub_abs, sub_no_overflow};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, CosmosMsg, Decimal256 as Decimal, Deps, DepsMut, Empty, Env, MessageInfo, QuerierWrapper, Reply, Response,
    StdError, StdResult, SubMsg, Uint128, Uint256, WasmMsg, WasmQuery,
};
use cw20_base::msg::InstantiateMsg as cw20_instantiate_msg;
use std::ops::Deref;
use std::str::FromStr;

use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg, MinterResponse, TokenInfoResponse};
use injective_bindings::{create_batch_update_orders_msg, InjectiveMsgWrapper, InjectiveQuerier, InjectiveQueryWrapper};

use cw0::parse_reply_instantiate_data;

const INSTANTIATE_REPLY_ID: u64 = 1u64;

#[entry_point]
pub fn instantiate(deps: DepsMut<InjectiveQueryWrapper>, env: Env, _info: MessageInfo, msg: InstantiateMsg) -> Result<Response, StdError> {
    let state = State {
        market_id: msg.market_id.to_string(),
        subaccount_id: decode_bech32(&env.contract.address),
        fee_recipient: env.contract.address.to_string(),
        leverage: Decimal::from_str(&msg.leverage).unwrap(),
        order_density: Uint256::from_str(&msg.order_density).unwrap(),
        reservation_price_sensitivity_ratio: Decimal::from_str(&msg.reservation_price_sensitivity_ratio).unwrap(),
        reservation_spread_sensitivity_ratio: Decimal::from_str(&msg.reservation_spread_sensitivity_ratio).unwrap(),
        max_active_capital_utilization_ratio: Decimal::from_str(&msg.max_active_capital_utilization_ratio).unwrap(),
        head_change_tolerance_ratio: Decimal::from_str(&msg.head_change_tolerance_ratio).unwrap(),
        mid_price_tail_deviation_ratio: Decimal::from_str(&msg.mid_price_tail_deviation_ratio).unwrap(),
        min_head_to_tail_deviation_ratio: Decimal::from_str(&msg.min_head_to_tail_deviation_ratio).unwrap(),
        lp_token_address: None,
    };

    config(deps.storage).save(&state)?;

    let marketing = match msg.cw20_marketing_info {
        Some(info) => Some(cw20_base::msg::InstantiateMarketingInfo {
            project: info.project,
            description: info.description,
            marketing: info.marketing,
            logo: info.logo,
        }),
        None => None,
    };

    let cw20_instantiate_msg = cw20_instantiate_msg {
        name: msg.lp_name,
        symbol: msg.lp_symbol,
        decimals: 6,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: env.contract.address.to_string(),
            cap: None,
        }),
        marketing,
    };

    let instantiate_message = WasmMsg::Instantiate {
        admin: None,
        code_id: msg.cw20_code_id.parse::<u64>().unwrap(),
        msg: to_binary(&cw20_instantiate_msg)?,
        funds: vec![],
        label: msg.cw20_label,
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
    let lp_token_address = Cw20Contract(child_contract);

    let mut state = config_read(deps.storage).load().unwrap();
    state.lp_token_address = Some(lp_token_address);

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
    let lp_token_address = state.lp_token_address.unwrap().addr().to_string();

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
    let lp_token_address = state.lp_token_address.unwrap().addr().to_string();

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
    let positions_res = querier.query_subaccount_position(state.market_id.clone(), state.subaccount_id.clone())?;
    let open_orders_res = querier.query_trader_derivative_orders(state.market_id.clone(), state.subaccount_id.clone())?;
    let perpetual_market_info_res = querier.query_perpetual_market_info(state.market_id.clone())?;
    let perpetual_market_funding_res = querier.query_perpetual_market_funding(state.market_id.clone())?;

    let market = DerivativeMarket::from_query(market_res.market.market);
    let open_orders_res_val = open_orders_res.orders.unwrap_or_default();
    let open_orders = open_orders_res_val
        .into_iter()
        .map(|order| {
            // OrderType_BUY         OrderType = 1
            // OrderType_SELL        OrderType = 2
            // TODO add PO order types
            let order_type = if order.isBuy { 1 } else { 2 };
            let order_info = OrderInfo::new(state.subaccount_id.clone(), state.fee_recipient.clone(), order.price, order.quantity);
            DerivativeLimitOrder::new(order.margin, order.fillable, order.order_hash, None, order_type, order_info)
        })
        .collect::<Vec<DerivativeLimitOrder>>();

    let deposit = Deposit::from_query(deposit_res.deposits);

    let first_position: Option<Position> = if positions_res.state.is_none() {
        None
    } else {
        Some(Position::from_query(positions_res.state.unwrap()))
    };

    let perpetual_market_info: Option<PerpetualMarketInfo> = if perpetual_market_info_res.info.is_none() {
        None
    } else {
        Some(PerpetualMarketInfo::from_query(perpetual_market_info_res.info.unwrap()))
    };
    let perpetual_market_funding: Option<PerpetualMarketFunding> = if perpetual_market_funding_res.state.is_none() {
        None
    } else {
        Some(PerpetualMarketFunding::from_query(perpetual_market_funding_res.state.unwrap()))
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
        market_res.market.mark_price,
        Decimal::from_str("80000").unwrap(), // TODO
        market_res.market.mark_price,
    );

    Ok(Response::new()
        .set_data(Binary::from(b"just some test data"))
        .add_messages(action_response.unwrap()))
}

#[entry_point]
pub fn query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
        QueryMsg::GetMarketId {} => to_binary(&get_market_id(deps)?),
        QueryMsg::GetTotalLpSupply {} => to_binary(&get_total_lp_supply(deps)?),
    }
}

fn get_market_id(deps: Deps<InjectiveQueryWrapper>) -> StdResult<MarketIdResponse> {
    let state = config_read(deps.storage).load().unwrap();
    Ok(MarketIdResponse {
        market_id: state.market_id.clone(),
    })
}

fn get_total_lp_supply(deps: Deps<InjectiveQueryWrapper>) -> StdResult<TotalSupplyResponse> {
    let state = config_read(deps.storage).load().unwrap();
    let lp_token_address = state.lp_token_address.unwrap().addr().to_string();

    let msg = Cw20QueryMsg::TokenInfo {};
    let query = WasmQuery::Smart {
        contract_addr: lp_token_address,
        msg: to_binary(&msg)?,
    }
    .into();

    let querier = QuerierWrapper::<Empty>::new(deps.querier.deref());
    let res: TokenInfoResponse = querier.query(&query)?;

    Ok(TotalSupplyResponse {
        total_supply: res.total_supply,
    })
}

fn get_action(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    market: DerivativeMarket,
    _perpetual_market_info: Option<PerpetualMarketInfo>,
    _perpetual_market_funding: Option<PerpetualMarketFunding>,
    open_orders: Vec<DerivativeLimitOrder>,
    deposit: Deposit,
    position: Option<Position>,
    oracle_price: Decimal,
    volatility: Decimal,
    mid_price: Decimal,
) -> StdResult<Vec<CosmosMsg<InjectiveMsgWrapper>>> {
    // Load state
    let state = config_read(deps.storage).load().unwrap();

    // Calculate inventory imbalance parameter
    let (inventory_imbalance_ratio, imbalance_is_long) = inventory_imbalance_deriv(
        &position,
        oracle_price,
        state.max_active_capital_utilization_ratio,
        deposit.total_balance,
        &state,
    );

    // Calculate reservation price
    let reservation_price = reservation_price(
        mid_price,
        inventory_imbalance_ratio,
        volatility,
        state.reservation_price_sensitivity_ratio,
        imbalance_is_long,
    );

    // Calculate the new head prices
    let (new_buy_head, new_sell_head) = new_head_prices(volatility, reservation_price, state.reservation_spread_sensitivity_ratio);

    // Split open orders
    let (open_buy_orders, open_sell_orders) = split_open_orders(open_orders);

    // Ensure that the heads have changed enough that we are willing to make an action
    if should_take_action(&open_buy_orders, new_buy_head, state.head_change_tolerance_ratio)
        || should_take_action(&open_sell_orders, new_sell_head, state.head_change_tolerance_ratio)
    {
        // Get new tails
        let (new_buy_tail, new_sell_tail) = new_tail_prices(
            new_buy_head,
            new_sell_head,
            mid_price,
            state.mid_price_tail_deviation_ratio,
            state.min_head_to_tail_deviation_ratio,
        );

        // Get information for buy order creation/cancellation
        let (buy_orders_to_cancel, buy_orders_to_keep, buy_agg_margin_of_orders_kept, buy_vacancy_is_near_head) =
            orders_to_cancel(open_buy_orders, new_buy_head, new_buy_tail, true, &state, &market);

        // Get information for sell order creation/cancellation
        let (sell_orders_to_cancel, sell_orders_to_keep, sell_agg_margin_of_orders_kept, sell_vacancy_is_near_head) =
            orders_to_cancel(open_sell_orders, new_sell_head, new_sell_tail, false, &state, &market);

        // Get new buy/sell orders
        let (buy_orders_to_open, additional_buys_to_cancel) = create_orders(
            new_buy_head,
            new_buy_tail,
            deposit.total_balance,
            buy_orders_to_keep,
            buy_agg_margin_of_orders_kept,
            &position,
            buy_vacancy_is_near_head,
            true,
            &state,
            &market,
        );
        let (sell_orders_to_open, additional_sells_to_cancel) = create_orders(
            new_sell_head,
            new_sell_tail,
            deposit.total_balance,
            sell_orders_to_keep,
            sell_agg_margin_of_orders_kept,
            &position,
            sell_vacancy_is_near_head,
            false,
            &state,
            &market,
        );

        let batch_order = create_batch_update_orders_msg(
            env.contract.address.to_string(),
            String::from(""),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            serde_json_wasm::from_str(
                &serde_json_wasm::to_string(
                    &vec![
                        buy_orders_to_cancel,
                        sell_orders_to_cancel,
                        additional_buys_to_cancel,
                        additional_sells_to_cancel,
                    ]
                    .concat(),
                )
                .unwrap(),
            )
            .unwrap(),
            Vec::new(),
            serde_json_wasm::from_str(&serde_json_wasm::to_string(&vec![buy_orders_to_open, sell_orders_to_open].concat()).unwrap()).unwrap(),
        );

        Ok(vec![batch_order])
    } else {
        Ok(Vec::new())
    }
}

/// # Description
/// Determines which orders to cancel and which to leave resting on the book depending on the placement of the new tails
/// # Arguments
/// * `open_orders` - The buy or sell side orders from the previous block
/// * `new_head` - The new buy or sell head
/// * `new_tail` - The new buy or sell tail
/// * `is_buy` - If this block of orders is on the buyside
/// * `state` - State that the contract was initialized with
/// * `market` - Derivative market information
/// # Returns
/// * `orders_to_cancel` - The orders we've decided to cancel
/// * `orders_to_keep` - The orders that we would like to keep resting on the book
/// * `agg_margin_of_orders_kept` - The total aggregated margined value of the orders we would like to keep
/// * `vacancy_is_near_head` - True if the space between new head and closest order to keep is greater than that of new tail and its
///   respective closest order. This will be used to determine if we need to do a head to tail or tail to head later.
pub fn orders_to_cancel(
    open_orders: Vec<DerivativeLimitOrder>,
    new_head: Decimal,
    new_tail: Decimal,
    is_buy: bool,
    state: &State,
    market: &DerivativeMarket,
) -> (Vec<OrderData>, Vec<DerivativeLimitOrder>, Decimal, bool) {
    // If there are any open orders, we need to check them to see if we should cancel
    if open_orders.len() > 0 {
        let mut agg_margin_of_orders_kept = Decimal::zero();
        let mut orders_to_cancel: Vec<OrderData> = Vec::new();
        let mut orders_to_keep: Vec<DerivativeLimitOrder> = Vec::new();
        open_orders.into_iter().for_each(|o| {
            let keep_if_buy = o.order_info.price <= new_head && o.order_info.price >= new_tail;
            let keep_if_sell = o.order_info.price >= new_head && o.order_info.price <= new_tail;
            let keep = (keep_if_buy && is_buy) || (keep_if_sell && !is_buy);
            if keep {
                agg_margin_of_orders_kept = agg_margin_of_orders_kept + o.margin;
                orders_to_keep.push(o);
            } else {
                orders_to_cancel.push(OrderData::new(o.order_hash.clone(), state, market));
            }
        });
        if orders_to_keep.len() == 0 {
            (orders_to_cancel, Vec::new(), Decimal::zero(), true)
        } else {
            // Determine if we need to append to new orders to the new head or if we need to
            // append to the end of the block of orders we will be keeping
            let vacancy_is_near_head = sub_abs(new_head, orders_to_keep.first().unwrap().order_info.price)
                > sub_abs(orders_to_keep.last().unwrap().order_info.price, new_tail);
            (orders_to_cancel, orders_to_keep, agg_margin_of_orders_kept, vacancy_is_near_head)
        }
    } else {
        (Vec::new(), Vec::new(), Decimal::zero(), true)
    }
}

/// # Description
/// Creates new orders. Determines what kind of order creation we need: base, head to tail, or tail to head
/// # Arguments
/// * `new_head` - The new buy or sell head
/// * `new_tail` - The new buy or sell tail
/// * `total_deposit_balance` - The total quote balance LPed
/// * `orders_to_keep` - The orders that we would like to keep resting on the book
/// * `agg_margin_of_orders_kept` - The total aggregated margined value of the orders we would like to keep
/// * `position` - The position we have taken, if any
/// * `vacancy_is_near_head` - True if the space between new head and closest order to keep is greater than that of new tail and its
/// * `is_buy` - If this block of orders is on the buyside
/// * `state` - State that the contract was initialized with
/// * `market` - Derivative market information
/// # Returns
/// * `orders_to_create` - The new orders we would like to create
/// * `additional_orders_to_cancel` - The additional orders we need to cancel
fn create_orders(
    new_head: Decimal,
    new_tail: Decimal,
    total_deposit_balance: Decimal,
    orders_to_keep: Vec<DerivativeLimitOrder>,
    agg_margin_of_orders_kept: Decimal,
    position: &Option<Position>,
    vacancy_is_near_head: bool,
    is_buy: bool,
    state: &State,
    market: &DerivativeMarket,
) -> (Vec<DerivativeOrder>, Vec<OrderData>) {
    let (position_qty_to_reduce, position_margin, position_is_same_side) = match position {
        Some(position) => {
            if position.is_long == is_buy {
                (Decimal::zero(), position.margin, true)
            } else {
                (position.quantity, position.margin, false)
            }
        }
        None => (Decimal::zero(), Decimal::zero(), false),
    };
    let total_margin_balance_for_new_orders = total_marginable_balance_for_new_orders(
        total_deposit_balance,
        position_is_same_side,
        position_margin,
        state.max_active_capital_utilization_ratio,
        agg_margin_of_orders_kept,
    );
    let num_orders_to_keep = orders_to_keep.len();
    if num_orders_to_keep == 0 {
        let (new_orders, _, _) = create_orders_between_bounds_deriv(
            new_head,
            new_tail,
            total_margin_balance_for_new_orders,
            num_orders_to_keep,
            position_qty_to_reduce,
            true,
            is_buy,
            &state,
            market,
        );
        (new_orders, Vec::new())
    } else if vacancy_is_near_head {
        create_orders_tail_to_head_deriv(
            new_head,
            total_margin_balance_for_new_orders,
            orders_to_keep,
            position_qty_to_reduce,
            is_buy,
            state,
            market,
        )
    } else {
        create_orders_head_to_tail_deriv(
            new_tail,
            total_margin_balance_for_new_orders,
            orders_to_keep,
            position_qty_to_reduce,
            is_buy,
            state,
            market,
        )
    }
}

/// # Description
/// Uses the inventory imbalance and its direction to calculate a price around which we will center the mid price
/// # Formulas
/// * `reservation price (case: position is long)` =  mid price - (inventory imbalance ratio * volatility * reservation price sensitivity ratio)
/// * `reservation price (case: position is short)` =  mid price + (inventory imbalance ratio * volatility * reservation price sensitivity ratio)
/// # Arguments
/// * `mid_price` - The true center between the best bid and ask
/// * `inventory_imbalance_ratio` - A relationship between margined position and total deposit balance (margin/total_deposit_balance). Is
///    zero if there is no position open.
/// * `volatility` - A measure of volatility that we update on a block by block basis
/// * `reservation_price_sensitivity_ratio` - The constant to control the sensitivity of the volatility param
/// * `imbalance_is_long` - True if the imbalance is skewed towards being long
/// # Returns
/// * `reservation_price` - The price around which we will center both heads
fn reservation_price(
    mid_price: Decimal,
    inventory_imbalance_ratio: Decimal,
    volatility: Decimal,
    reservation_price_sensitivity_ratio: Decimal,
    imbalance_is_long: bool,
) -> Decimal {
    let shift_from_mid_price = inventory_imbalance_ratio * volatility * reservation_price_sensitivity_ratio;
    if imbalance_is_long {
        sub_no_overflow(mid_price, shift_from_mid_price)
    } else {
        mid_price + shift_from_mid_price
    }
}

/// # Description
/// Uses the reservation price and volatility to calculate where the buy/sell heads should be. Both buy and
/// sell heads will be equi-distant from the reservation price.
/// # Formulas
/// * `buy head` = reservation_price - ((volatility * sensitivity) / 2)
/// * `sell head` = reservation_price + ((volatility * sensitivity) / 2)
/// # Arguments
/// * `volatility` - A measure of volatility that we update on a block by block basis
/// * `reservation_price` - The price around which we will center both heads
/// * `reservation_spread_sensitivity_ratio` - The constant to control the sensitivity of the spread around the reservation_price
/// # Returns
/// * `buy_head` - The new buy head
/// * `sell_head` - The new sell head
fn new_head_prices(volatility: Decimal, reservation_price: Decimal, reservation_spread_sensitivity_ratio: Decimal) -> (Decimal, Decimal) {
    let dist_from_reservation_price = div_dec(volatility * reservation_spread_sensitivity_ratio, Decimal::from_str("2").unwrap());
    (
        sub_no_overflow(reservation_price, dist_from_reservation_price),
        reservation_price + dist_from_reservation_price,
    )
}

/// # Description
/// Determines whether the new head for the next block will be different enough than that of the previous to warrant
/// order cancellation and the creation of new orders.
/// # Formulas (case: no open orders)
/// * `should take action` = true
/// # Formulas (case: some open orders)
/// * `should take action` = (Abs(old head - new head) / old head) > head change tolerance ratio
/// # Arguments
/// * `open_orders` - The buy or sell side orders from the previous block
/// * `new_head` - The new proposed buy or sell head
/// * `head_change_tolerance_ratio` - A constant between 0..1 that serves as a threshold for which we actually want to take action in the new block
/// # Returns
/// * `should_take_action` - Whether we should cancel and place new orders with respect to the new head and tails
fn should_take_action(open_orders: &Vec<DerivativeLimitOrder>, new_head: Decimal, head_change_tolerance_ratio: Decimal) -> bool {
    if open_orders.len() == 0 {
        true
    } else {
        let old_head = open_orders.first().unwrap().order_info.price;
        div_dec(sub_abs(old_head, new_head), old_head) > head_change_tolerance_ratio
    }
}

/// # Description
/// Calculates the correct tail prices for buyside and sellside from the mid price. If either of
/// the distances between the head and tail fall below the minimum spread defined at initialization, risk
/// manager returns a tail that meets the minimum spread constraint.
/// # Formulas
/// * `proposed buy tail` = mid price * (1 - mid price deviation ratio)
/// * `proposed sell tail` = mid price * (1 + mid price deviation ratio)
/// * `max buy tail` = buy head * (1 - min head to tail deviation ratio)
/// * `min sell tail` = sell head * (1 + min head to tail deviation ratio)
/// * `new buy tail` = min(max buy tail, proposed buy tail)
/// * `new sell tail` = max(min sell tail, proposed sell tail)
/// # Arguments
/// * `new_buy_head` - The new buy head
/// * `new_sell_head` - The new sell head
/// * `mid_price` - A mid_price that we update on a block by block basis
/// * `mid_price_tail_deviation_ratio` - A constant between 0..1 that is used to determine how far we want to place our tails from the mid_price
/// * `min_head_to_tail_deviation_ratio` - A constant between 0..1 that ensures our tail is at least some distance from the head (risk management param)
/// # Returns
/// * `new_buy_tail` - The new buy tail
/// * `new_sell_tail` - The new sell tail
pub fn new_tail_prices(
    new_buy_head: Decimal,
    new_sell_head: Decimal,
    mid_price: Decimal,
    mid_price_tail_deviation_ratio: Decimal,
    min_head_to_tail_deviation_ratio: Decimal,
) -> (Decimal, Decimal) {
    let proposed_buy_tail = mid_price * sub_no_overflow(Decimal::one(), mid_price_tail_deviation_ratio);
    let proposed_sell_tail = mid_price * (Decimal::one() + mid_price_tail_deviation_ratio);
    check_tail_dist(
        new_buy_head,
        new_sell_head,
        proposed_buy_tail,
        proposed_sell_tail,
        min_head_to_tail_deviation_ratio,
    )
}

/// # Description
/// Splits the vec of orders to buyside and sellside orders. Sorts them so that the head from the previous block is at index == 0. Buyside
/// orders are sorted desc. Sellside are sorted asc.
/// # Arguments
/// * `open_orders` - The open orders from both sides that were on the book as of the preceding block
/// # Returns
/// * `open_buy_orders` - The sorted buyside orders
/// * `open_sell_orders` - The sorted sellside orders
fn split_open_orders(open_orders: Vec<DerivativeLimitOrder>) -> (Vec<DerivativeLimitOrder>, Vec<DerivativeLimitOrder>) {
    if open_orders.len() > 0 {
        let mut open_buy_orders: Vec<DerivativeLimitOrder> = Vec::new();
        let mut open_sell_orders: Vec<DerivativeLimitOrder> = open_orders
            .into_iter()
            .filter(|o| {
                if o.order_type == 1 {
                    open_buy_orders.push(o.clone());
                }
                o.order_type == 2
            })
            .collect();

        // Sort both so the head is at index 0
        open_buy_orders.sort_by(|a, b| b.order_info.price.partial_cmp(&a.order_info.price).unwrap());
        open_sell_orders.sort_by(|a, b| a.order_info.price.partial_cmp(&b.order_info.price).unwrap());

        (open_buy_orders, open_sell_orders)
    } else {
        (Vec::new(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        contract::create_orders,
        exchange::{DerivativeLimitOrder, DerivativeMarket, OrderInfo, Position},
        state::State,
        utils::{div_dec, sub_no_overflow},
    };

    use super::{new_tail_prices, orders_to_cancel, should_take_action, split_open_orders};
    use cosmwasm_std::{Decimal256 as Decimal, Uint256};
    use std::str::FromStr;

    #[test]
    fn cancellation_test() {
        let inv_val = Decimal::from_str("1000").unwrap();
        let value = Decimal::from_str("62.5").unwrap();
        let mp = Decimal::from_str("20").unwrap();
        let price_step_mult = Decimal::from_str("1").unwrap();
        let leverage = Decimal::from_str("1").unwrap();
        let market = mock_market();

        let state = mock_state("1".to_string(), "10".to_string());
        let open_buys = mock_wrapped_deriv_limit(value, mp, price_step_mult, 2, 10, true, leverage);

        open_buys.iter().for_each(|o| {
            println!("{} {} {} {}", o.order_hash, o.order_info.price, o.order_info.quantity, o.margin);
        });
        let new_head = Decimal::from_str("15").unwrap();
        let new_tail = Decimal::from_str("6").unwrap();
        let (orders_to_cancel, orders_to_keep, margined_val_from_orders_remaining, append_to_new_head) =
            orders_to_cancel(open_buys, new_head, new_tail, true, &state, &market);

        let position = Position {
            is_long: true,
            quantity: Decimal::from_str("5").unwrap(),
            entry_price: Decimal::zero(),
            margin: Decimal::from_str("125").unwrap(),
            cumulative_funding_entry: Decimal::zero(),
        };
        let (orders, cancels) = create_orders(
            new_head,
            new_tail,
            inv_val,
            orders_to_keep.clone(),
            margined_val_from_orders_remaining,
            &Some(position.clone()),
            append_to_new_head,
            true,
            &state,
            &mock_market(),
        );
        orders.iter().for_each(|o| {
            println!(" {} {} {}", o.order_info.price, o.order_info.quantity, o.margin);
        });
        orders_to_cancel.iter().for_each(|o| {
            println!("cancel from h-t {}", o.order_hash);
        });
        cancels.iter().for_each(|o| {
            println!("cancel from new o {}", o.order_hash);
        });
        println!("{}", orders_to_keep.len());
        println!("{}", margined_val_from_orders_remaining);
        println!("{}", append_to_new_head);
        let mut val = Decimal::zero();
        orders.iter().for_each(|o| val = val + o.get_margin());
        orders_to_keep.iter().for_each(|o| val = val + o.margin);

        let expected_val = div_dec(inv_val, Decimal::from_str("2").unwrap());
        if position.is_long {
            println!("{} {}", val + position.margin, expected_val);
        } else {
            println!("{} {}", val, expected_val);
        }
    }

    #[test]
    fn should_take_action_test() {
        let mut open_orders: Vec<DerivativeLimitOrder> = Vec::new();
        let new_head = Decimal::from_str("100000000100").unwrap();
        let head_change_tolerance_ratio = Decimal::from_str("0.01").unwrap();
        let should_change = should_take_action(&open_orders, new_head, head_change_tolerance_ratio);
        assert!(should_change);

        open_orders.push(DerivativeLimitOrder {
            fillable: Default::default(),
            margin: Default::default(),
            order_info: OrderInfo {
                subaccount_id: String::from(""),
                fee_recipient: String::from(""),
                price: Decimal::from_str("100000000000").unwrap(),
                quantity: Decimal::zero(),
            },
            order_type: 1,
            trigger_price: None,
            order_hash: String::from(""),
        });

        let should_change = should_take_action(&open_orders, new_head, head_change_tolerance_ratio);
        assert!(!should_change);

        open_orders.pop();
        open_orders.push(DerivativeLimitOrder {
            fillable: Default::default(),
            margin: Default::default(),
            order_info: OrderInfo {
                subaccount_id: String::from(""),
                fee_recipient: String::from(""),
                price: Decimal::from_str("110000000000").unwrap(),
                quantity: Decimal::zero(),
            },
            order_type: 1,
            trigger_price: None,
            order_hash: String::from(""),
        });

        let should_change = should_take_action(&open_orders, new_head, head_change_tolerance_ratio);
        assert!(should_change);

        let should_change = should_take_action(&Vec::new(), new_head, Decimal::from_str("1").unwrap());
        assert!(should_change);
    }

    #[test]
    fn new_tail_prices_test() {
        let buy_head = Decimal::from_str("3999").unwrap();
        let mid_price = Decimal::from_str("4000").unwrap();
        let sell_head = Decimal::from_str("4001").unwrap();
        let max_mid_price_tail_deviation_ratio = Decimal::from_str("0.05").unwrap();
        let min_head_to_tail_deviation_ratio = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = new_tail_prices(
            buy_head,
            sell_head,
            mid_price,
            max_mid_price_tail_deviation_ratio,
            min_head_to_tail_deviation_ratio,
        );
        assert_eq!(buy_tail, mid_price * sub_no_overflow(Decimal::one(), max_mid_price_tail_deviation_ratio));
        assert_eq!(sell_tail, mid_price * (Decimal::one() + max_mid_price_tail_deviation_ratio));

        let max_mid_price_tail_deviation_ratio = Decimal::from_str("0.001").unwrap();
        let min_head_to_tail_deviation_ratio = Decimal::from_str("0.01").unwrap();
        let (buy_tail, sell_tail) = new_tail_prices(
            buy_head,
            sell_head,
            mid_price,
            max_mid_price_tail_deviation_ratio,
            min_head_to_tail_deviation_ratio,
        );
        assert_eq!(buy_tail, buy_head * sub_no_overflow(Decimal::one(), min_head_to_tail_deviation_ratio));
        assert_eq!(sell_tail, sell_head * (Decimal::one() + min_head_to_tail_deviation_ratio));
    }

    #[test]
    fn split_open_orders_test() {
        let mut open_orders: Vec<DerivativeLimitOrder> = Vec::new();
        let order = DerivativeLimitOrder {
            fillable: Default::default(),
            margin: Default::default(),
            order_info: OrderInfo {
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

    fn mock_wrapped_deriv_limit(
        value: Decimal,
        mp: Decimal,
        price_step_mult: Decimal,
        num_reduce_only: usize,
        num_orders: usize,
        is_buy: bool,
        leverage: Decimal,
    ) -> Vec<DerivativeLimitOrder> {
        let mut orders: Vec<DerivativeLimitOrder> = Vec::new();
        for i in 0..num_orders {
            let price = if is_buy {
                mp - (Decimal::from_str(&i.to_string()).unwrap() * price_step_mult)
            } else {
                mp + (Decimal::from_str(&i.to_string()).unwrap() * price_step_mult)
            };
            let quantity = div_dec(value, price);
            let margin = if i < num_reduce_only {
                Decimal::zero()
            } else {
                div_dec(quantity * price, leverage)
            };
            orders.push(DerivativeLimitOrder {
                trigger_price: None,
                order_info: OrderInfo {
                    subaccount_id: "".to_string(),
                    fee_recipient: "".to_string(),
                    price,
                    quantity,
                },
                order_type: 0,
                margin,
                fillable: Decimal::zero(),
                order_hash: i.to_string(),
            });
        }
        orders
    }

    fn mock_state(leverage: String, order_density: String) -> State {
        State {
            market_id: String::from(""),
            subaccount_id: String::from(""),
            order_density: Uint256::from_str(&order_density).unwrap(),
            max_active_capital_utilization_ratio: Decimal::from_str("1").unwrap(),
            min_head_to_tail_deviation_ratio: Decimal::from_str("0.03").unwrap(),
            mid_price_tail_deviation_ratio: Decimal::from_str("0.06").unwrap(),
            head_change_tolerance_ratio: Decimal::zero(),
            leverage: Decimal::from_str(&leverage).unwrap(),
            reservation_price_sensitivity_ratio: Decimal::zero(),
            reservation_spread_sensitivity_ratio: Decimal::zero(),
            fee_recipient: String::from(""),
            lp_token_address: None,
        }
    }

    fn mock_market() -> DerivativeMarket {
        DerivativeMarket {
            ticker: String::from(""),
            oracle_base: String::from(""),
            oracle_quote: String::from(""),
            oracle_type: 0,
            oracle_scale_factor: 0,
            quote_denom: String::from(""),
            market_id: String::from(""),
            initial_margin_ratio: Decimal::from_str("0").unwrap(),
            maintenance_margin_ratio: Decimal::from_str("0").unwrap(),
            maker_fee_rate: Decimal::from_str("0").unwrap(),
            taker_fee_rate: Decimal::from_str("0").unwrap(),
            isPerpetual: true,
            status: 0,
            min_price_tick_size: Decimal::from_str("1000").unwrap(),
            min_quantity_tick_size: Decimal::from_str("0.00001").unwrap(),
        }
    }
}
