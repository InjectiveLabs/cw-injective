use crate::msg::{InstantiateMsg, QueryStargateResponse, MSG_CREATE_DERIVATIVE_LIMIT_ORDER_ENDPOINT, MSG_CREATE_SPOT_LIMIT_ORDER_ENDPOINT};
use cosmwasm_std::{coin, Addr, Coin};
use injective_cosmwasm::{checked_address_to_subaccount_id, SubaccountId};
use injective_math::{scale::Scaled, FPDecimal};
use injective_std::{
    shim::{Any, Timestamp},
    types::{
        cosmos::{
            authz::v1beta1::{GenericAuthorization, Grant, MsgGrant, MsgRevoke, MsgRevokeResponse},
            bank::v1beta1::{MsgSend, SendAuthorization},
            base::v1beta1::Coin as BaseCoin,
            gov::{
                v1::{MsgSubmitProposal, MsgVote},
                v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1,
            },
        },
        injective::{
            exchange::v1beta1::{
                DerivativeOrder, MsgCreateDerivativeLimitOrder, MsgCreateSpotLimitOrder, MsgInstantPerpetualMarketLaunch, MsgInstantSpotMarketLaunch,
                OrderInfo, OrderType, QueryDerivativeMarketsRequest, QuerySpotMarketsRequest, SpotOrder,
            },
            insurance::v1beta1::MsgCreateInsuranceFund,
            oracle::v1beta1::{
                GrantPriceFeederPrivilegeProposal, MsgRelayPriceFeedPrice, MsgRelayPythPrices, MsgUpdateParams, OracleType, Params, PriceAttestation,
            },
        },
    },
};
use injective_test_tube::{
    injective_cosmwasm::get_default_subaccount_id_for_checked_address, Account, Authz, Bank, Exchange, ExecuteResponse, Gov, InjectiveTestApp,
    Insurance, Module, Oracle, Runner, RunnerResult, SigningAccount, Wasm,
};
use injective_testing::human_to_i64;
use prost::Message;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, ops::Neg, str::FromStr};

pub const EXCHANGE_DECIMALS: i32 = 18i32;
pub const BASE_DECIMALS: i32 = 18i32;
pub const ATOM_DECIMALS: i32 = 8i32;
pub const QUOTE_DECIMALS: i32 = 6i32;

pub const ATOM_DENOM: &str = "atom";
pub const BASE_DENOM: &str = "inj";
pub const QUOTE_DENOM: &str = "usdt";
pub const INJ_PYTH_PRICE_ID: &str = "0x7a5bc1d2b56ad029048cd63964b3ad2776eadf812edc1a43a31406cb54bff592";
pub const USDT_PYTH_PRICE_ID: &str = "0x1fc18861232290221461220bd4e2acd1dcdfbc89c84092c93c18bdc7756c1588";
pub const GOV_MODULE_ADDRESS: &str = "inj10d07y265gmmuvt4z0w9aw880jnsr700jstypyt";

pub struct UserInfo {
    pub account: SigningAccount,
    pub subaccount_id: SubaccountId,
}
pub struct Setup {
    pub app: InjectiveTestApp,
    pub owner: SigningAccount,
    pub signer: SigningAccount,
    pub validator: SigningAccount,
    pub users: Vec<UserInfo>,
    pub denoms: HashMap<String, String>,
    pub contract_address: String,
    pub code_id: u64,
    pub market_id: Option<String>,
}

pub enum ExchangeType {
    Spot,
    Derivative,
    None,
}

impl Setup {
    pub fn new(exchange_type: ExchangeType) -> Self {
        let app = InjectiveTestApp::new();
        let wasm = Wasm::new(&app);
        let mut market_id = None;

        let mut denoms = HashMap::new();
        denoms.insert("atom".to_string(), ATOM_DENOM.to_string());
        denoms.insert("quote".to_string(), QUOTE_DENOM.to_string());
        denoms.insert("base".to_string(), BASE_DENOM.to_string());

        let signer = app.init_account(&[str_coin("1000000", BASE_DENOM, BASE_DECIMALS)]).unwrap();

        let validator = app.get_first_validator_signing_account(BASE_DENOM.to_string(), 1.2f64).unwrap();

        let owner = app
            .init_account(&[
                str_coin("1000000", ATOM_DENOM, ATOM_DECIMALS),
                str_coin("1000000", BASE_DENOM, BASE_DECIMALS),
                str_coin("1000000", QUOTE_DENOM, QUOTE_DECIMALS),
            ])
            .unwrap();

        let mut users: Vec<UserInfo> = Vec::new();
        for _ in 0..10 {
            let user = app
                .init_account(&[
                    str_coin("1000000", ATOM_DENOM, ATOM_DECIMALS),
                    str_coin("1000000", BASE_DENOM, BASE_DECIMALS),
                    str_coin("1000", QUOTE_DENOM, QUOTE_DECIMALS),
                ])
                .unwrap();

            let user_subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(user.address()), 0u32);

            users.push(UserInfo {
                account: user,
                subaccount_id: user_subaccount_id,
            });
        }

        let wasm_byte_code = std::fs::read(wasm_file("injective_cosmwasm_stargate_example".to_string())).unwrap();
        let code_id = wasm.store_code(&wasm_byte_code, None, &owner).unwrap().data.code_id;

        // Instantiate contract
        let contract_address: String = wasm
            .instantiate(code_id, &InstantiateMsg {}, Some(&owner.address()), Some("mock-contract"), &[], &owner)
            .unwrap()
            .data
            .address;

        assert!(!contract_address.is_empty(), "Contract address is empty");

        send(&Bank::new(&app), "1000000000000000000000", BASE_DENOM, &owner, &validator);

        launch_insurance_fund(
            &app,
            &owner,
            "INJ/USDT",
            denoms["quote"].as_str(),
            denoms["base"].as_str(),
            denoms["quote"].as_str(),
            OracleType::PriceFeed,
        );

        launch_price_feed_oracle(
            &app,
            &signer,
            &validator,
            denoms["base"].as_str(),
            denoms["quote"].as_str(),
            human_to_dec("10.01", BASE_DECIMALS).to_string(),
        );

        match exchange_type {
            ExchangeType::Spot => {
                let exchange = Exchange::new(&app);
                market_id = Some(launch_spot_market(&exchange, &owner, "INJ/USDT".to_string()));
            }
            ExchangeType::Derivative => {
                let exchange = Exchange::new(&app);
                market_id = Some(launch_perp_market(&exchange, &owner, "INJ/USDT".to_string()));
            }
            ExchangeType::None => {}
        }

        Self {
            app,
            owner,
            signer,
            validator,
            users,
            denoms,
            contract_address,
            code_id,
            market_id,
        }
    }
}

impl Default for Setup {
    fn default() -> Self {
        Self::new(ExchangeType::None)
    }
}

pub fn wasm_file(contract_name: String) -> String {
    let snaked_name = contract_name.replace('-', "_");
    let arch = std::env::consts::ARCH;

    let target = format!("../../target/wasm32-unknown-unknown/release/{snaked_name}.wasm");

    let artifacts_dir = std::env::var("ARTIFACTS_DIR_PATH").unwrap_or_else(|_| "artifacts".to_string());
    let arch_target = format!("../../{artifacts_dir}/{snaked_name}-{arch}.wasm");

    if std::path::Path::new(&target).exists() {
        target
    } else if std::path::Path::new(&arch_target).exists() {
        arch_target
    } else {
        format!("../../{artifacts_dir}/{snaked_name}.wasm")
    }
}

pub fn str_coin(human_amount: &str, denom: &str, decimals: i32) -> Coin {
    let scaled_amount = human_to_dec(human_amount, decimals);
    let as_int: u128 = scaled_amount.into();
    coin(as_int, denom)
}

pub fn send(bank: &Bank<InjectiveTestApp>, amount: &str, denom: &str, from: &SigningAccount, to: &SigningAccount) {
    bank.send(
        MsgSend {
            from_address: from.address(),
            to_address: to.address(),
            amount: vec![BaseCoin {
                amount: amount.to_string(),
                denom: denom.to_string(),
            }],
        },
        from,
    )
    .unwrap();
}

pub fn launch_price_feed_oracle(
    app: &InjectiveTestApp,
    signer: &SigningAccount,
    validator: &SigningAccount,
    base: &str,
    quote: &str,
    dec_price: String,
) {
    let gov = Gov::new(app);
    let oracle = Oracle::new(app);

    let mut buf = vec![];
    GrantPriceFeederPrivilegeProposal::encode(
        &GrantPriceFeederPrivilegeProposal {
            title: "test-proposal".to_string(),
            description: "test-proposal".to_string(),
            base: base.to_string(),
            quote: quote.to_string(),
            relayers: vec![signer.address()],
        },
        &mut buf,
    )
    .unwrap();

    let res = gov
        .submit_proposal_v1beta1(
            MsgSubmitProposalV1Beta1 {
                content: Some(Any {
                    type_url: "/injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal".to_string(),
                    value: buf,
                }),
                initial_deposit: vec![BaseCoin {
                    amount: "100000000000000000000".to_string(),
                    denom: "inj".to_string(),
                }],
                proposer: validator.address(),
            },
            validator,
        )
        .unwrap();

    let proposal_id = res.events.iter().find(|e| e.ty == "submit_proposal").unwrap().attributes[0]
        .value
        .to_owned();

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

    // NOTE: increase the block time in order to move past the voting period
    app.increase_time(10u64);

    oracle
        .relay_price_feed(
            MsgRelayPriceFeedPrice {
                sender: signer.address(),
                base: vec![base.to_string()],
                quote: vec![quote.to_string()],
                price: vec![dec_price], // 1.2@18dp
            },
            signer,
        )
        .unwrap();
}

pub fn launch_insurance_fund(
    app: &InjectiveTestApp,
    signer: &SigningAccount,
    ticker: &str,
    quote: &str,
    oracle_base: &str,
    oracle_quote: &str,
    oracle_type: OracleType,
) {
    let insurance = Insurance::new(app);

    insurance
        .create_insurance_fund(
            MsgCreateInsuranceFund {
                sender: signer.address(),
                ticker: ticker.to_string(),
                quote_denom: quote.to_string(),
                oracle_base: oracle_base.to_string(),
                oracle_quote: oracle_quote.to_string(),
                oracle_type: oracle_type as i32,
                expiry: -1i64,
                initial_deposit: Some(BaseCoin {
                    amount: human_to_dec("1_000", QUOTE_DECIMALS).to_string(),
                    denom: quote.to_string(),
                }),
            },
            signer,
        )
        .unwrap();
}

pub fn launch_spot_market(exchange: &Exchange<InjectiveTestApp>, signer: &SigningAccount, ticker: String) -> String {
    exchange
        .instant_spot_market_launch(
            MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: ticker.clone(),
                base_denom: BASE_DENOM.to_string(),
                quote_denom: QUOTE_DENOM.to_string(),
                min_price_tick_size: dec_to_proto(FPDecimal::must_from_str("0.000000000000001")),
                min_quantity_tick_size: dec_to_proto(FPDecimal::must_from_str("1")),
                min_notional: dec_to_proto(FPDecimal::must_from_str("0.000000000000002")),
            },
            signer,
        )
        .unwrap();

    get_spot_market_id(exchange, ticker)
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

pub fn launch_perp_market(exchange: &Exchange<InjectiveTestApp>, signer: &SigningAccount, ticker: String) -> String {
    exchange
        .instant_perpetual_market_launch(
            MsgInstantPerpetualMarketLaunch {
                sender: signer.address(),
                ticker: ticker.to_owned(),
                quote_denom: "usdt".to_string(),
                oracle_base: "inj".to_string(),
                oracle_quote: "usdt".to_string(),
                oracle_scale_factor: 6u32,
                oracle_type: 2i32,
                maker_fee_rate: "0".to_owned(),
                taker_fee_rate: "0".to_owned(),
                initial_margin_ratio: "195000000000000000".to_owned(),
                maintenance_margin_ratio: "50000000000000000".to_owned(),
                min_price_tick_size: "1000000000000000000000".to_owned(),
                min_quantity_tick_size: "1000000000000000".to_owned(),
                min_notional: "1000000000000000".to_string(),
            },
            signer,
        )
        .unwrap();

    get_perpetual_market_id(exchange, ticker)
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

#[derive(Clone)]
pub struct HumanOrder {
    pub price: String,
    pub quantity: String,
    pub order_type: OrderType,
}
pub fn add_spot_order_as(app: &InjectiveTestApp, market_id: String, trader: &UserInfo, price: String, quantity: String, order_type: OrderType) {
    let exchange = Exchange::new(app);
    exchange
        .create_spot_limit_order(
            MsgCreateSpotLimitOrder {
                sender: trader.account.address().clone(),
                order: Some(SpotOrder {
                    market_id: market_id.to_owned(),
                    order_info: Some(OrderInfo {
                        subaccount_id: trader.subaccount_id.to_string(),
                        fee_recipient: trader.account.address(),
                        price,
                        quantity,
                        cid: "".to_string(),
                    }),
                    order_type: order_type.into(),
                    trigger_price: "".to_string(),
                }),
            },
            &trader.account,
        )
        .unwrap();
}

pub fn add_spot_orders(app: &InjectiveTestApp, market_id: String, orders: Vec<HumanOrder>) {
    let account = app
        .init_account(&[
            str_coin("1000000", BASE_DENOM, BASE_DECIMALS),
            str_coin("1000000", QUOTE_DENOM, QUOTE_DECIMALS),
        ])
        .unwrap();

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(account.address()), 0u32);

    let trader = UserInfo { account, subaccount_id };

    for order in orders {
        let (price, quantity) = scale_price_quantity_for_spot_market(order.price.as_str(), order.quantity.as_str(), &BASE_DECIMALS, &QUOTE_DECIMALS);
        add_spot_order_as(app, market_id.to_owned(), &trader, price, quantity, order.order_type);
    }
}

pub fn get_initial_liquidity_orders_vector() -> Vec<HumanOrder> {
    vec![
        HumanOrder {
            price: "15".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "12".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "8".to_string(),
            quantity: "5".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "5".to_string(),
            quantity: "10".to_string(),
            order_type: OrderType::Buy,
        },
    ]
}

pub fn add_spot_initial_liquidity(app: &InjectiveTestApp, market_id: String) {
    add_spot_orders(app, market_id, get_initial_liquidity_orders_vector());
}

pub fn get_initial_perp_liquidity_orders_vector() -> Vec<HumanOrder> {
    vec![
        HumanOrder {
            price: "10.2".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "10.1".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Sell,
        },
        HumanOrder {
            price: "9.9".to_string(),
            quantity: "1".to_string(),
            order_type: OrderType::Buy,
        },
        HumanOrder {
            price: "9.8".to_string(),
            quantity: "2".to_string(),
            order_type: OrderType::Buy,
        },
    ]
}

pub fn add_derivative_order_as(
    app: &InjectiveTestApp,
    market_id: String,
    trader: &SigningAccount,
    price: String,
    quantity: String,
    order_type: OrderType,
    margin: String,
) {
    let exchange = Exchange::new(app);
    exchange
        .create_derivative_limit_order(
            MsgCreateDerivativeLimitOrder {
                sender: trader.address(),
                order: Some(DerivativeOrder {
                    market_id: market_id.to_owned(),
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
            trader,
        )
        .unwrap();
}

pub fn add_derivative_orders(app: &InjectiveTestApp, market_id: String, orders: Vec<HumanOrder>, margin: Option<String>) {
    let trader = app
        .init_account(&[
            str_coin("1000000", BASE_DENOM, BASE_DECIMALS),
            str_coin("1000000", QUOTE_DENOM, QUOTE_DECIMALS),
        ])
        .unwrap();

    let margin = margin.unwrap_or("2".into());

    for order in orders {
        let (price, quantity, order_margin) =
            scale_price_quantity_perp_market(order.price.as_str(), order.quantity.as_str(), &margin, &QUOTE_DECIMALS);
        add_derivative_order_as(app, market_id.to_owned(), &trader, price, quantity, order.order_type, order_margin);
    }
}

pub fn add_perp_initial_liquidity(app: &InjectiveTestApp, market_id: String) {
    add_derivative_orders(app, market_id, get_initial_perp_liquidity_orders_vector(), None);
}

pub fn revoke_authorization(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, msg_type_url: String) {
    let _res: ExecuteResponse<MsgRevokeResponse> = app
        .execute_multiple(
            &[(
                MsgRevoke {
                    granter: granter.address(),
                    grantee,
                    msg_type_url,
                },
                MsgRevoke::TYPE_URL,
            )],
            granter,
        )
        .unwrap();
}

pub fn create_generic_authorization(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, msg: String, expiration: Option<Timestamp>) {
    let authz = Authz::new(app);

    let mut buf = vec![];
    GenericAuthorization::encode(&GenericAuthorization { msg }, &mut buf).unwrap();

    authz
        .grant(
            MsgGrant {
                granter: granter.address(),
                grantee,
                grant: Some(Grant {
                    authorization: Some(Any {
                        type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                        value: buf.clone(),
                    }),
                    expiration,
                }),
            },
            granter,
        )
        .unwrap();
}

pub fn create_send_authorization(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String, amount: BaseCoin, expiration: Option<Timestamp>) {
    let authz = Authz::new(app);

    let mut buf = vec![];
    SendAuthorization::encode(
        &SendAuthorization {
            spend_limit: vec![amount],
            allow_list: vec![],
        },
        &mut buf,
    )
    .unwrap();

    authz
        .grant(
            MsgGrant {
                granter: granter.address(),
                grantee,
                grant: Some(Grant {
                    authorization: Some(Any {
                        type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                        value: buf.clone(),
                    }),
                    expiration,
                }),
            },
            granter,
        )
        .unwrap();
}

pub fn execute_all_authorizations(app: &InjectiveTestApp, granter: &SigningAccount, grantee: String) {
    create_generic_authorization(app, granter, grantee.clone(), MSG_CREATE_SPOT_LIMIT_ORDER_ENDPOINT.to_string(), None);

    create_generic_authorization(
        app,
        granter,
        grantee.clone(),
        MSG_CREATE_DERIVATIVE_LIMIT_ORDER_ENDPOINT.to_string(),
        None,
    );

    create_generic_authorization(
        app,
        granter,
        grantee.clone(),
        "/injective.exchange.v1beta1.MsgCreateDerivativeMarketOrder".to_string(),
        None,
    );

    create_generic_authorization(
        app,
        granter,
        grantee.clone(),
        "/injective.exchange.v1beta1.MsgBatchUpdateOrders".to_string(),
        None,
    );

    create_generic_authorization(app, granter, grantee, "/injective.exchange.v1beta1.MsgWithdraw".to_string(), None);
}

// Human Utils
pub fn human_to_proto(raw_number: &str, decimals: i32) -> String {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(18 + decimals).to_string()
}

pub fn human_to_dec(raw_number: &str, decimals: i32) -> FPDecimal {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(decimals)
}

pub fn dec_to_proto(val: FPDecimal) -> String {
    val.scaled(18).to_string()
}

pub fn scale_price_quantity_for_spot_market(price: &str, quantity: &str, base_decimals: &i32, quote_decimals: &i32) -> (String, String) {
    let (scaled_price, scaled_quantity) = scale_price_quantity_for_spot_market_dec(price, quantity, base_decimals, quote_decimals);
    (dec_to_proto(scaled_price), dec_to_proto(scaled_quantity))
}

pub fn scale_price_quantity_for_spot_market_dec(price: &str, quantity: &str, base_decimals: &i32, quote_decimals: &i32) -> (FPDecimal, FPDecimal) {
    let price_dec = FPDecimal::must_from_str(price.replace('_', "").as_str());
    let quantity_dec = FPDecimal::must_from_str(quantity.replace('_', "").as_str());

    let scaled_price = price_dec.scaled(quote_decimals - base_decimals);
    let scaled_quantity = quantity_dec.scaled(*base_decimals);

    (scaled_price, scaled_quantity)
}

pub fn scale_price_quantity_perp_market(price: &str, quantity: &str, margin_ratio: &str, quote_decimals: &i32) -> (String, String, String) {
    let (scaled_price, scaled_quantity, scaled_margin) = scale_price_quantity_perp_market_dec(price, quantity, margin_ratio, quote_decimals);

    (dec_to_proto(scaled_price), dec_to_proto(scaled_quantity), dec_to_proto(scaled_margin))
}

pub fn scale_price_quantity_perp_market_dec(
    price: &str,
    quantity: &str,
    margin_ratio: &str,
    quote_decimals: &i32,
) -> (FPDecimal, FPDecimal, FPDecimal) {
    let price_dec = FPDecimal::must_from_str(price.replace('_', "").as_str());
    let quantity_dec = FPDecimal::must_from_str(quantity.replace('_', "").as_str());
    let margin_ratio_dec = FPDecimal::must_from_str(margin_ratio.replace('_', "").as_str());

    let scaled_price = price_dec.scaled(*quote_decimals);
    let scaled_quantity = quantity_dec;

    let scaled_margin = (price_dec * quantity_dec * margin_ratio_dec).scaled(*quote_decimals);

    (scaled_price, scaled_quantity, scaled_margin)
}

pub fn set_address_of_pyth_contract(app: &InjectiveTestApp, validator: &SigningAccount, pyth_address: &SigningAccount) {
    let gov = Gov::new(app);

    let mut buf = vec![];
    MsgUpdateParams::encode(
        &MsgUpdateParams {
            authority: GOV_MODULE_ADDRESS.to_string(),
            params: Some(Params {
                pyth_contract: pyth_address.address(),
            }),
        },
        &mut buf,
    )
    .unwrap();

    let res = gov
        .submit_proposal(
            MsgSubmitProposal {
                messages: vec![Any {
                    type_url: "/injective.oracle.v1beta1.MsgUpdateParams".to_string(),
                    value: buf,
                }],
                initial_deposit: vec![BaseCoin {
                    amount: "100000000000000000000".to_string(),
                    denom: "inj".to_string(),
                }],
                proposer: validator.address(),
                metadata: "".to_string(),
                title: "Set Pyth contract address".to_string(),
                summary: "Set Pyth contract address".to_string(),
            },
            validator,
        )
        .unwrap();

    let proposal_id = res.events.iter().find(|e| e.ty == "submit_proposal").unwrap().attributes[0]
        .value
        .to_owned();

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

    // NOTE: increase the block time in order to move past the voting period
    app.increase_time(11u64);
}

pub fn relay_pyth_price(oracle: &Oracle<InjectiveTestApp>, price_attestations: Vec<PriceAttestation>, pyth_address: &SigningAccount) {
    let pyth_price_msg = MsgRelayPythPrices {
        sender: pyth_address.address(),
        price_attestations,
    };

    oracle.relay_pyth_prices(pyth_price_msg, pyth_address).unwrap();
}

pub fn create_some_inj_price_attestation(human_price: &str, decimal_precision: i32, publish_time: i64) -> PriceAttestation {
    if decimal_precision < 0 {
        panic!("Desired exponent cannot be negative")
    };

    let (price_i64, exponent_to_use) = if decimal_precision == 1 {
        (human_price.parse::<i64>().unwrap(), 1)
    } else {
        (human_to_i64(human_price, decimal_precision), decimal_precision.neg())
    };

    PriceAttestation {
        price_id: INJ_PYTH_PRICE_ID.to_string(),
        price: price_i64,
        conf: 500,
        expo: exponent_to_use,
        ema_price: price_i64,
        ema_conf: 2000,
        ema_expo: exponent_to_use,
        publish_time,
    }
}

pub fn create_some_usdt_price_attestation(human_price: &str, decimal_precision: i32, publish_time: i64) -> PriceAttestation {
    if decimal_precision < 0 {
        panic!("Desired exponent cannot be negative")
    };

    let (price_i64, exponent_to_use) = if decimal_precision == 0 {
        (human_price.parse::<i64>().unwrap(), 0)
    } else {
        (human_to_i64(human_price, decimal_precision), decimal_precision.neg())
    };

    PriceAttestation {
        price_id: USDT_PYTH_PRICE_ID.to_string(),
        price: price_i64,
        conf: 500,
        expo: exponent_to_use,
        ema_price: price_i64,
        ema_conf: 2000,
        ema_expo: exponent_to_use,
        publish_time,
    }
}

pub fn get_stargate_query_result<T: DeserializeOwned>(contract_response: RunnerResult<QueryStargateResponse>) -> serde_json::Result<T> {
    let contract_response = contract_response.unwrap().value;
    serde_json::from_str::<T>(&contract_response).map_err(|error| {
        println!("{} \n {}", error.to_string(), contract_response);
        error
    })
}
