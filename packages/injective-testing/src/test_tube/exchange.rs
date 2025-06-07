use crate::{
    mocks::{
        MOCK_ATOM_DECIMALS, MOCK_ATOM_DENOM, MOCK_BASE_DECIMALS, MOCK_BASE_DENOM, MOCK_QUOTE_DECIMALS, MOCK_QUOTE_DENOM, MOCK_STAKE_DECIMALS,
        MOCK_STAKE_DENOM, MOCK_USDC_DENOM,
    },
    utils::{dec_to_proto, scale_price_quantity_perp_market, scale_price_quantity_spot_market, str_coin},
};

use cosmwasm_std::{Addr, Uint128};
use injective_cosmwasm::{get_default_subaccount_id_for_checked_address, SubaccountId};
use injective_math::FPDecimal;
use injective_std::types::cosmos::gov;
use injective_test_tube::{
    injective_std::{
        shim::Any,
        types::{
            cosmos::{
                base::v1beta1::Coin as BaseCoin,
                gov::v1::{MsgSubmitProposal, MsgVote},
            },
            injective::exchange::v1beta1::{
                BatchExchangeModificationProposal, DenomDecimals, DenomMinNotional, DenomMinNotionalProposal, DerivativeOrder, MsgBatchUpdateOrders,
                MsgBatchUpdateOrdersResponse, MsgCancelDerivativeOrder, MsgCreateDerivativeLimitOrder, MsgCreateDerivativeLimitOrderResponse,
                MsgCreateSpotLimitOrder, MsgInstantSpotMarketLaunch, MsgUpdateParams, OrderInfo, OrderType, PerpetualMarketFunding, Position,
                QueryDerivativeMarketsRequest, QueryExchangeParamsRequest, QueryExchangeParamsResponse, QuerySpotMarketsRequest,
                QuerySubaccountDepositsRequest, QuerySubaccountEffectivePositionInMarketRequest, SpotOrder, UpdateDenomDecimalsProposal,
            },
            injective::exchange::v2,
        },
    },
    Account, Exchange, Gov, InjectiveTestApp, Module, Runner, SigningAccount,
};
use prost::Message;
use std::str::FromStr;

pub fn add_exchange_admin(app: &InjectiveTestApp, validator: &SigningAccount, admin_address: String) {
    let gov = Gov::new(app);

    let res: QueryExchangeParamsResponse = app
        .query("/injective.exchange.v1beta1.Query/QueryExchangeParams", &QueryExchangeParamsRequest {})
        .unwrap();

    let mut exchange_params = res.params.unwrap();
    exchange_params.exchange_admins.push(admin_address);
    exchange_params.max_derivative_order_side_count = 300u32;

    // NOTE: this could change in the future
    let governance_module_address = "inj10d07y265gmmuvt4z0w9aw880jnsr700jstypyt";

    let mut buf = vec![];
    MsgUpdateParams::encode(
        &MsgUpdateParams {
            authority: governance_module_address.to_string(),
            params: Some(exchange_params),
        },
        &mut buf,
    )
    .unwrap();

    let res = gov
        .submit_proposal(
            MsgSubmitProposal {
                messages: vec![Any {
                    type_url: MsgUpdateParams::TYPE_URL.to_string(),
                    value: buf,
                }],
                initial_deposit: vec![BaseCoin {
                    amount: "100000000000000000000".to_string(),
                    denom: "inj".to_string(),
                }],
                proposer: validator.address(),
                metadata: "".to_string(),
                title: "Update params".to_string(),
                summary: "Basically updating the params".to_string(),
                expedited: false,
            },
            validator,
        )
        .unwrap();

    let proposal_id = res.events.iter().find(|e| e.ty == "submit_proposal").unwrap().attributes[0].value.clone();

    gov.vote(
        MsgVote {
            proposal_id: u64::from_str(&proposal_id).unwrap(),
            voter: validator.address(),
            option: 1i32,
            metadata: "".to_string(),
        },
        validator,
    )
    .unwrap();

    // Increase time to pass the proposal
    app.increase_time(20u64);
}

pub fn add_denom_notional_and_decimal(app: &InjectiveTestApp, validator: &SigningAccount, denom: String, min_notional: String, decimals: u64) {
    let gov = Gov::new(app);

    let proposal = BatchExchangeModificationProposal {
        title: "Update params".to_string(),
        description: "Basically updating the params".to_string(),
        spot_market_param_update_proposals: vec![],
        derivative_market_param_update_proposals: vec![],
        spot_market_launch_proposals: vec![],
        perpetual_market_launch_proposals: vec![],
        expiry_futures_market_launch_proposals: vec![],
        trading_reward_campaign_update_proposal: None,
        binary_options_market_launch_proposals: vec![],
        binary_options_param_update_proposals: vec![],
        denom_decimals_update_proposal: Some(UpdateDenomDecimalsProposal {
            title: "Update denom decimals".to_string(),
            description: "Love it!".to_string(),
            denom_decimals: vec![DenomDecimals {
                denom: denom.clone(),
                decimals,
            }],
        }),
        fee_discount_proposal: None,
        market_forced_settlement_proposals: vec![],
        denom_min_notional_proposal: Some(DenomMinNotionalProposal {
            title: "Update min notional".to_string(),
            description: "Love it!".to_string(),
            denom_min_notionals: vec![DenomMinNotional { denom, min_notional }],
        }),
    };

    let mut buf = vec![];
    proposal.encode(&mut buf).unwrap();

    let content_any = Any {
        type_url: "/injective.exchange.v2.BatchExchangeModificationProposal".to_string(),
        value: buf,
    };

    let msg_submit_proposal = gov::v1beta1::MsgSubmitProposal {
        content: Some(content_any),
        initial_deposit: vec![BaseCoin {
            amount: "100000000000000000000".to_string(),
            denom: "inj".to_string(),
        }],
        proposer: validator.address(),
    };

    let res = gov.submit_proposal_v1beta1(msg_submit_proposal, validator).unwrap();

    let proposal_id = res.events.iter().find(|e| e.ty == "submit_proposal").unwrap().attributes[0].value.clone();

    gov.vote(
        MsgVote {
            proposal_id: u64::from_str(&proposal_id).unwrap(),
            voter: validator.address(),
            option: 1i32,
            metadata: "".to_string(),
        },
        validator,
    )
    .unwrap();

    // Increase time to pass the proposal
    app.increase_time(20u64);
}

pub fn create_perp_mid_price(app: &InjectiveTestApp, market_id: &str, base_price: &str, base_quantity: &str, base_margin: &str, spread: f64) {
    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{:.1}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{:.1}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (price, quantity, margin) = scale_price_quantity_perp_market(&sell_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);
    execute_derivative_limit_order(app, market_id.to_string(), price, quantity, margin, OrderType::Sell);

    // Scaling and executing the buy order
    let (price, quantity, margin) = scale_price_quantity_perp_market(&buy_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);
    execute_derivative_limit_order(app, market_id.to_string(), price, quantity, margin, OrderType::Buy);
}

pub fn create_perp_mid_price_as(
    app: &InjectiveTestApp,
    market_id: &str,
    base_price: &str,
    base_quantity: &str,
    base_margin: &str,
    spread: f64,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) {
    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (price, quantity, margin) = scale_price_quantity_perp_market(&sell_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);
    execute_derivative_limit_order_as(
        app,
        market_id.to_string(),
        price,
        quantity,
        margin,
        OrderType::Sell,
        trader,
        subaccount_id,
    );

    // Scaling and executing the buy order
    let (price, quantity, margin) = scale_price_quantity_perp_market(&buy_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);
    execute_derivative_limit_order_as(app, market_id.to_string(), price, quantity, margin, OrderType::Buy, trader, subaccount_id);
}

pub fn create_spot_mid_price(
    app: &InjectiveTestApp,
    market_id: &str,
    base_price: &str,
    base_quantity: &str,
    base_decimals: &i32,
    quote_decimals: &i32,
    spread: f64,
) {
    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (price, quantity) = scale_price_quantity_spot_market(&sell_price, base_quantity, base_decimals, quote_decimals);
    execute_spot_limit_order(app, market_id.to_string(), price, quantity, OrderType::Sell);

    // Scaling and executing the buy order
    let (price, quantity) = scale_price_quantity_spot_market(&buy_price, base_quantity, base_decimals, quote_decimals);
    execute_spot_limit_order(app, market_id.to_string(), price, quantity, OrderType::Buy);
}

pub fn create_spot_mid_price_as(
    app: &InjectiveTestApp,
    market_id: &str,
    base_price: &str,
    base_quantity: &str,
    base_decimals: &i32,
    quote_decimals: &i32,
    spread: f64,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) {
    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{:.1}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{:.1}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (price, quantity) = scale_price_quantity_spot_market(&sell_price, base_quantity, base_decimals, quote_decimals);
    execute_spot_limit_order_as(app, market_id.to_string(), price, quantity, OrderType::Sell, trader, subaccount_id);

    // Scaling and executing the buy order
    let (price, quantity) = scale_price_quantity_spot_market(&buy_price, base_quantity, base_decimals, quote_decimals);
    execute_spot_limit_order_as(app, market_id.to_string(), price, quantity, OrderType::Buy, trader, subaccount_id);
}

pub fn create_price_perp(
    app: &InjectiveTestApp,
    market_id: &str,
    base_price: &str,
    base_quantity: &str,
    base_margin: &str,
    spread: f64,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) -> MsgBatchUpdateOrdersResponse {
    let exchange = Exchange::new(app);

    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (sell_price, sell_quantity, sell_margin) = scale_price_quantity_perp_market(&sell_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);

    // Scaling and executing the buy order
    let (buy_price, buy_quantity, buy_margin) = scale_price_quantity_perp_market(&buy_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);

    exchange
        .batch_update_orders(
            MsgBatchUpdateOrders {
                sender: trader.address(),
                subaccount_id: subaccount_id.as_str().to_string(),
                derivative_market_ids_to_cancel_all: vec![market_id.to_string()],
                derivative_orders_to_create: vec![
                    DerivativeOrder {
                        market_id: market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: buy_price.clone(),
                            quantity: buy_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        margin: buy_margin.clone(),
                        order_type: OrderType::Buy.into(),
                        trigger_price: "".to_string(),
                    },
                    DerivativeOrder {
                        market_id: market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: sell_price.clone(),
                            quantity: sell_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        margin: sell_margin.clone(),
                        order_type: OrderType::Sell.into(),
                        trigger_price: "".to_string(),
                    },
                ],
                ..Default::default()
            },
            trader,
        )
        .unwrap()
        .data
}

pub fn create_price_spot(
    app: &InjectiveTestApp,
    market_id: &str,
    base_price: &str,
    base_quantity: &str,
    spread: f64,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) -> MsgBatchUpdateOrdersResponse {
    let exchange = Exchange::new(app);

    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (sell_price, sell_quantity) = scale_price_quantity_spot_market(&sell_price, base_quantity, &MOCK_BASE_DECIMALS, &MOCK_QUOTE_DECIMALS);

    // Scaling and executing the buy order
    let (buy_price, buy_quantity) = scale_price_quantity_spot_market(&buy_price, base_quantity, &MOCK_BASE_DECIMALS, &MOCK_QUOTE_DECIMALS);

    exchange
        .batch_update_orders(
            MsgBatchUpdateOrders {
                sender: trader.address(),
                subaccount_id: subaccount_id.as_str().to_string(),
                spot_market_ids_to_cancel_all: vec![market_id.to_string()],
                spot_orders_to_create: vec![
                    SpotOrder {
                        market_id: market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: buy_price.clone(),
                            quantity: buy_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        order_type: OrderType::Buy.into(),
                        trigger_price: "".to_string(),
                    },
                    SpotOrder {
                        market_id: market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: sell_price.clone(),
                            quantity: sell_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        order_type: OrderType::Sell.into(),
                        trigger_price: "".to_string(),
                    },
                ],
                ..Default::default()
            },
            trader,
        )
        .unwrap()
        .data
}

pub fn create_price_spot_and_perp_market(
    app: &InjectiveTestApp,
    perp_market_id: &str,
    spot_market_id: &str,
    base_price: &str,
    base_quantity: &str,
    base_margin: &str,
    spread: f64,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) -> MsgBatchUpdateOrdersResponse {
    let exchange = Exchange::new(app);

    // Calculate adjusted prices for buy and sell based on the spread
    let sell_price = format!("{}", base_price.parse::<f64>().unwrap() + spread);
    let buy_price = format!("{}", base_price.parse::<f64>().unwrap() - spread);

    // Scaling and executing the sell order
    let (perp_sell_price, perp_sell_quantity, perp_sell_margin) =
        scale_price_quantity_perp_market(&sell_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);

    // Scaling and executing the buy order
    let (perp_buy_price, perp_buy_quantity, perp_buy_margin) =
        scale_price_quantity_perp_market(&buy_price, base_quantity, base_margin, &MOCK_QUOTE_DECIMALS);

    // Scaling and executing the sell order
    let (spot_sell_price, spot_sell_quantity) =
        scale_price_quantity_spot_market(&sell_price, base_quantity, &MOCK_BASE_DECIMALS, &MOCK_QUOTE_DECIMALS);

    // Scaling and executing the buy order
    let (spot_buy_price, spot_buy_quantity) = scale_price_quantity_spot_market(&buy_price, base_quantity, &MOCK_BASE_DECIMALS, &MOCK_QUOTE_DECIMALS);

    exchange
        .batch_update_orders(
            MsgBatchUpdateOrders {
                sender: trader.address(),
                subaccount_id: subaccount_id.as_str().to_string(),
                derivative_market_ids_to_cancel_all: vec![perp_market_id.to_string()],
                spot_market_ids_to_cancel_all: vec![spot_market_id.to_string()],
                derivative_orders_to_create: vec![
                    DerivativeOrder {
                        market_id: perp_market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: perp_buy_price.clone(),
                            quantity: perp_buy_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        margin: perp_buy_margin.clone(),
                        order_type: OrderType::Buy.into(),
                        trigger_price: "".to_string(),
                    },
                    DerivativeOrder {
                        market_id: perp_market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: perp_sell_price.clone(),
                            quantity: perp_sell_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        margin: perp_sell_margin.clone(),
                        order_type: OrderType::Sell.into(),
                        trigger_price: "".to_string(),
                    },
                ],
                spot_orders_to_create: vec![
                    SpotOrder {
                        market_id: spot_market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: spot_buy_price.clone(),
                            quantity: spot_buy_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        order_type: OrderType::Buy.into(),
                        trigger_price: "".to_string(),
                    },
                    SpotOrder {
                        market_id: spot_market_id.to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: subaccount_id.as_str().to_string(),
                            fee_recipient: trader.address(),
                            price: spot_sell_price.clone(),
                            quantity: spot_sell_quantity.clone(),
                            cid: "".to_string(),
                        }),
                        order_type: OrderType::Sell.into(),
                        trigger_price: "".to_string(),
                    },
                ],
                ..Default::default()
            },
            trader,
        )
        .unwrap()
        .data
}

pub fn cancel_derivative_order_as(
    app: &InjectiveTestApp,
    market_id: String,
    order_hash: String,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) {
    let exchange = Exchange::new(app);

    exchange
        .cancel_derivative_order(
            MsgCancelDerivativeOrder {
                sender: trader.address(),
                market_id,
                subaccount_id: subaccount_id.as_str().to_string(),
                order_hash,
                order_mask: 0i32,
                cid: "".to_string(),
            },
            trader,
        )
        .unwrap();
}

pub fn launch_spot_market(exchange: &Exchange<InjectiveTestApp>, signer: &SigningAccount, ticker: String) -> String {
    exchange
        .instant_spot_market_launch(
            MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: ticker.clone(),
                base_denom: MOCK_BASE_DENOM.to_string(),
                quote_denom: MOCK_QUOTE_DENOM.to_string(),
                min_price_tick_size: dec_to_proto(FPDecimal::must_from_str("0.000000000000001")),
                min_quantity_tick_size: dec_to_proto(FPDecimal::must_from_str("1000000000000000")),
                min_notional: dec_to_proto(FPDecimal::must_from_str("1")),
                base_decimals: MOCK_BASE_DECIMALS as u32,
                quote_decimals: MOCK_QUOTE_DECIMALS as u32,
            },
            signer,
        )
        .unwrap();

    get_spot_market_id(exchange, ticker)
}

pub fn launch_spot_market_atom(exchange: &Exchange<InjectiveTestApp>, signer: &SigningAccount, ticker: String) -> String {
    exchange
        .instant_spot_market_launch_v2(
            v2::MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: "ATOM/USDT".to_owned(),
                base_denom: MOCK_ATOM_DENOM.to_owned(),
                quote_denom: MOCK_QUOTE_DENOM.to_owned(),
                min_price_tick_size: dec_to_proto(FPDecimal::must_from_str("0.000010000000000000")),
                min_quantity_tick_size: dec_to_proto(FPDecimal::must_from_str("100000")),
                min_notional: dec_to_proto(FPDecimal::must_from_str("1")),
                base_decimals: MOCK_ATOM_DECIMALS as u32,
                quote_decimals: MOCK_QUOTE_DECIMALS as u32,
            },
            signer,
        )
        .unwrap();

    get_spot_market_id(exchange, ticker)
}

pub fn launch_spot_market_custom(
    exchange: &Exchange<InjectiveTestApp>,
    signer: &SigningAccount,
    ticker: String,
    base_denom: String,
    quote_denom: String,
    min_price_tick_size: String,
    min_quantity_tick_size: String,
) -> String {
    exchange
        .instant_spot_market_launch(
            MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: ticker.clone(),
                base_denom,
                quote_denom,
                min_price_tick_size: dec_to_proto(FPDecimal::must_from_str(&min_price_tick_size)),
                min_quantity_tick_size: dec_to_proto(FPDecimal::must_from_str(&min_quantity_tick_size)),
                min_notional: dec_to_proto(FPDecimal::must_from_str("1")),
                base_decimals: MOCK_BASE_DECIMALS as u32,
                quote_decimals: MOCK_QUOTE_DECIMALS as u32,
            },
            signer,
        )
        .unwrap();

    get_spot_market_id(exchange, ticker)
}

pub fn launch_spot_market_custom_v2(
    exchange: &Exchange<InjectiveTestApp>,
    signer: &SigningAccount,
    ticker: String,
    base_denom: String,
    quote_denom: String,
    min_price_tick_size: String,
    min_quantity_tick_size: String,
    base_decimals: u32,
    quote_decimals: u32,
) -> String {
    exchange
        .instant_spot_market_launch_v2(
            v2::MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: ticker.clone(),
                base_denom,
                quote_denom,
                min_price_tick_size: dec_to_proto(FPDecimal::must_from_str(&min_price_tick_size)),
                min_quantity_tick_size: dec_to_proto(FPDecimal::must_from_str(&min_quantity_tick_size)),
                min_notional: dec_to_proto(FPDecimal::must_from_str("1")),
                base_decimals,
                quote_decimals,
            },
            signer,
        )
        .unwrap();

    get_spot_market_id(exchange, ticker)
}

pub fn launch_perp_market(exchange: &Exchange<InjectiveTestApp>, signer: &SigningAccount, ticker: String) -> String {
    exchange
        .instant_perpetual_market_launch_v2(
            v2::MsgInstantPerpetualMarketLaunch {
                sender: signer.address(),
                ticker: ticker.to_owned(),
                quote_denom: "usdt".to_string(),
                oracle_base: "inj".to_string(),
                oracle_quote: "usdt".to_string(),
                oracle_scale_factor: 6u32,
                oracle_type: 2i32,
                maker_fee_rate: "-000100000000000000".to_owned(),
                taker_fee_rate: "0005000000000000000".to_owned(),
                initial_margin_ratio: "195000000000000000".to_owned(),
                maintenance_margin_ratio: "50000000000000000".to_owned(),
                min_price_tick_size: "1000000000000000000000".to_owned(),
                min_quantity_tick_size: "1000000000000000".to_owned(),
                min_notional: dec_to_proto(FPDecimal::must_from_str("1")),
                reduce_margin_ratio: "150000000000000000".to_string(),
            },
            signer,
        )
        .unwrap();

    get_perpetual_market_id(exchange, ticker)
}

pub fn launch_perp_market_atom(exchange: &Exchange<InjectiveTestApp>, signer: &SigningAccount, ticker: String) -> String {
    exchange
        .instant_perpetual_market_launch_v2(
            v2::MsgInstantPerpetualMarketLaunch {
                sender: signer.address(),
                ticker: ticker.to_owned(),
                quote_denom: MOCK_QUOTE_DENOM.to_owned(),
                oracle_base: MOCK_ATOM_DENOM.to_owned(),
                oracle_quote: MOCK_QUOTE_DENOM.to_owned(),
                oracle_scale_factor: 6u32,
                oracle_type: 2i32,
                maker_fee_rate: "-000100000000000000".to_owned(),
                taker_fee_rate: "0005000000000000000".to_owned(),
                initial_margin_ratio: "195000000000000000".to_owned(),
                maintenance_margin_ratio: "50000000000000000".to_owned(),
                min_price_tick_size: "1000000000000000000000".to_owned(),
                min_quantity_tick_size: "10000000000000000".to_owned(),
                min_notional: dec_to_proto(FPDecimal::must_from_str("1")),
                reduce_margin_ratio: "150000000000000000".to_string(),
            },
            signer,
        )
        .unwrap();

    get_perpetual_market_id(exchange, ticker)
}

pub fn execute_spot_limit_order(app: &InjectiveTestApp, market_id: String, price: String, quantity: String, order_type: OrderType) {
    let trader = app
        .init_account(&[
            str_coin("1000000", MOCK_ATOM_DENOM, MOCK_ATOM_DECIMALS),
            str_coin("1000000", MOCK_BASE_DENOM, MOCK_BASE_DECIMALS),
            str_coin("1000000", MOCK_STAKE_DENOM, MOCK_STAKE_DECIMALS),
            str_coin("1000000", MOCK_QUOTE_DENOM, MOCK_QUOTE_DECIMALS),
            str_coin("1000000", MOCK_USDC_DENOM, MOCK_QUOTE_DECIMALS),
        ])
        .unwrap();

    let exchange = Exchange::new(app);

    exchange
        .create_spot_limit_order(
            MsgCreateSpotLimitOrder {
                sender: trader.address(),
                order: Some(SpotOrder {
                    market_id,
                    order_info: Some(OrderInfo {
                        subaccount_id: get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.address()))
                            .as_str()
                            .to_string(),
                        fee_recipient: trader.address(),
                        price,
                        quantity,
                        cid: "".to_string(),
                    }),
                    order_type: order_type.into(),
                    trigger_price: "".to_string(),
                }),
            },
            &trader,
        )
        .unwrap();
}

pub fn execute_spot_limit_order_as(
    app: &InjectiveTestApp,
    market_id: String,
    price: String,
    quantity: String,
    order_type: OrderType,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) {
    let exchange = Exchange::new(app);

    exchange
        .create_spot_limit_order(
            MsgCreateSpotLimitOrder {
                sender: trader.address(),
                order: Some(SpotOrder {
                    market_id,
                    order_info: Some(OrderInfo {
                        subaccount_id: subaccount_id.as_str().to_string(),
                        fee_recipient: trader.address(),
                        price,
                        quantity,
                        cid: "".to_string(),
                    }),
                    order_type: order_type.into(),
                    trigger_price: "".to_string(),
                }),
            },
            trader,
        )
        .unwrap();
}

pub fn estimate_funding_apy(funding_info: &PerpetualMarketFunding, position: &Position) -> FPDecimal {
    let cumulative_funding = FPDecimal::from_str(&funding_info.cumulative_funding).unwrap();
    let cumulative_funding_entry = FPDecimal::from_str(&position.cumulative_funding_entry).unwrap();

    cumulative_funding - cumulative_funding_entry
}

pub fn execute_derivative_limit_order(
    app: &InjectiveTestApp,
    market_id: String,
    price: String,
    quantity: String,
    margin: String,
    order_type: OrderType,
) {
    let trader = app
        .init_account(&[
            str_coin("1000000", MOCK_ATOM_DENOM, MOCK_ATOM_DECIMALS),
            str_coin("1000000", MOCK_BASE_DENOM, MOCK_BASE_DECIMALS),
            str_coin("1000000", MOCK_STAKE_DENOM, MOCK_STAKE_DECIMALS),
            str_coin("1000000", MOCK_QUOTE_DENOM, MOCK_QUOTE_DECIMALS),
        ])
        .unwrap();

    let exchange = Exchange::new(app);

    exchange
        .create_derivative_limit_order(
            MsgCreateDerivativeLimitOrder {
                sender: trader.address(),
                order: Some(DerivativeOrder {
                    market_id,
                    order_info: Some(OrderInfo {
                        subaccount_id: get_default_subaccount_id_for_checked_address(&Addr::unchecked(trader.address()))
                            .as_str()
                            .to_string(),
                        fee_recipient: trader.address(),
                        price,
                        quantity,
                        cid: "".to_string(),
                    }),
                    margin,
                    order_type: order_type.into(),
                    trigger_price: "".to_string(),
                }),
            },
            &trader,
        )
        .unwrap();
}

pub fn execute_derivative_limit_order_as(
    app: &InjectiveTestApp,
    market_id: String,
    price: String,
    quantity: String,
    margin: String,
    order_type: OrderType,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) -> MsgCreateDerivativeLimitOrderResponse {
    let exchange = Exchange::new(app);

    exchange
        .create_derivative_limit_order(
            MsgCreateDerivativeLimitOrder {
                sender: trader.address(),
                order: Some(DerivativeOrder {
                    market_id,
                    order_info: Some(OrderInfo {
                        subaccount_id: subaccount_id.as_str().to_string(),
                        fee_recipient: trader.address(),
                        price,
                        quantity,
                        cid: "".to_string(),
                    }),
                    margin,
                    order_type: order_type.into(),
                    trigger_price: "".to_string(),
                }),
            },
            trader,
        )
        .unwrap()
        .data
}

pub fn remove_orders(
    app: &InjectiveTestApp,
    perp_market_id: &str,
    spot_market_id: &str,
    trader: &SigningAccount,
    subaccount_id: &SubaccountId,
) -> MsgBatchUpdateOrdersResponse {
    let exchange = Exchange::new(app);

    exchange
        .batch_update_orders(
            MsgBatchUpdateOrders {
                sender: trader.address(),
                subaccount_id: subaccount_id.as_str().to_string(),
                derivative_market_ids_to_cancel_all: vec![perp_market_id.to_string()],
                spot_market_ids_to_cancel_all: vec![spot_market_id.to_string()],
                ..Default::default()
            },
            trader,
        )
        .unwrap()
        .data
}

pub fn get_perpetual_market_id(exchange: &Exchange<InjectiveTestApp>, ticker: String) -> String {
    let perpetual_markets = exchange
        .query_derivative_markets(&QueryDerivativeMarketsRequest {
            status: "Active".to_string(),
            market_ids: vec![],
            with_mid_price_and_tob: false,
        })
        .unwrap()
        .markets;

    let market = perpetual_markets
        .iter()
        .filter(|m| m.market.is_some())
        .find(|m| m.market.as_ref().unwrap().ticker == ticker)
        .unwrap()
        .market
        .as_ref()
        .unwrap();

    market.market_id.to_string()
}

pub fn get_subaccount_total_value(exchange: &Exchange<InjectiveTestApp>, market_id: String, subaccount_id: String, denom: String) -> Uint128 {
    let trade_deposits_during = exchange
        .query_subaccount_deposits(&QuerySubaccountDepositsRequest {
            subaccount_id: subaccount_id.clone(),
            subaccount: None,
        })
        .unwrap();

    let total_balance = Uint128::from_str(&trade_deposits_during.deposits[&denom].total_balance)
    .unwrap_or(Uint128::zero()) // Use zero if the result is an Err
    / Uint128::one();

    let effective_position = exchange
        .query_subaccount_effective_position_in_market(&QuerySubaccountEffectivePositionInMarketRequest { market_id, subaccount_id })
        .unwrap();

    let effective_margin = effective_position.state.as_ref().map_or(Uint128::zero(), |state| {
        Uint128::from_str(&state.effective_margin).unwrap_or(Uint128::zero())
    }) / Uint128::one();

    total_balance + effective_margin
}

pub fn get_spot_market_id(exchange: &Exchange<InjectiveTestApp>, ticker: String) -> String {
    let spot_markets = exchange
        .query_spot_markets(&QuerySpotMarketsRequest {
            status: "Active".to_string(),
            market_ids: vec![],
        })
        .unwrap()
        .markets;

    let market = spot_markets.iter().find(|m| m.ticker == ticker).unwrap();

    market.market_id.to_string()
}
