use cosmwasm_std::coin;

use injective_test_tube::RunnerError::{ExecuteError, QueryError};
use injective_test_tube::{
    Account, Bank, Exchange, Gov, InjectiveTestApp, Module, RunnerError, RunnerResult, Wasm,
};

use injective_math::{round_to_min_tick, FPDecimal};

use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::test_utils::{
    create_limit_order, fund_account_with_some_inj, init_contract_and_get_address,
    init_contract_with_fee_recipient_and_get_address, launch_spot_market,
    must_init_account_with_funds, pause_spot_market, query_all_bank_balances, query_bank_balance,
    set_route_and_assert_success, OrderSide,
};

const ETH: &str = "eth";
const ATOM: &str = "atom";
const SOL: &str = "sol";
const USDT: &str = "usdt";
const USDC: &str = "usdc";
const INJ: &str = "inj";

const DEFAULT_TAKER_FEE: f64 = 0.001;
const DEFAULT_ATOMIC_MULTIPLIER: f64 = 2.5;
const DEFAULT_SELF_RELAYING_FEE_PART: f64 = 0.6;
const DEFAULT_RELAYER_SHARE: f64 = 1.0 - DEFAULT_SELF_RELAYING_FEE_PART;

#[test]
fn happy_path_two_hops_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    // set the market
    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: FPDecimal = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        )
        .unwrap();
    assert_eq!(
        query_result,
        FPDecimal::must_from_str("2893.888"),
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    )
    .unwrap();

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::zero(),
        "some of the original amount wasn't swapped"
    );
    assert_eq!(
        to_balance,
        FPDecimal::must_from_str("2893"),
        "swapper did not receive expected amount"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn happy_path_two_hops_single_price_level_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(3, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let expected_atom_estimate_quantity = FPDecimal::must_from_str("751.492");
    let query_result: FPDecimal = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(3u128),
            },
        )
        .unwrap();
    assert_eq!(
        query_result, expected_atom_estimate_quantity,
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(750u128),
        },
        &[coin(3, ETH)],
        &swapper,
    )
    .unwrap();

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::zero(),
        "some of the original amount wasn't swapped"
    );
    assert_eq!(
        to_balance,
        expected_atom_estimate_quantity.int(),
        "swapper did not receive expected amount"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn happy_path_three_hops_quote_conversion_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000, USDC),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDC);
    let spot_market_3_id = launch_spot_market(&exchange, &owner, USDC, USDT);

    let contr_addr = init_contract_and_get_address(
        &wasm,
        &owner,
        &[coin(10_000_000_000, USDC), coin(10_000_000_000, USDT)],
    );
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_3_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
            coin(10_000_000_000_000_000_000_000_000, USDC),
        ],
    );

    //ETH-USDT
    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    //ATOM-USDC
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    //USDT-USDC
    create_limit_order(
        &app,
        &trader3,
        &spot_market_3_id,
        OrderSide::Sell,
        1,
        100_000_000,
    );

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: FPDecimal = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        )
        .unwrap();
    // expected amount is a bit lower, even though 1 USDT = 1 USDC, because of the fees
    assert_eq!(
        query_result,
        FPDecimal::must_from_str("2889.64"),
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        2,
        "wrong number of denoms in contract balances"
    );

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    )
    .unwrap();

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::zero(),
        "some of the original amount wasn't swapped"
    );
    assert_eq!(
        to_balance,
        FPDecimal::must_from_str("2889"),
        "swapper did not receive expected amount"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        2,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn happy_path_simple_sell_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        USDT,
        vec![spot_market_1_id.as_str().into()],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: USDT.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );
    let orders_nominal_total_value = FPDecimal::from(201000u128) * FPDecimal::from(5u128)
        + FPDecimal::from(195000u128) * FPDecimal::from(4u128)
        + FPDecimal::from(192000u128) * FPDecimal::from(3u128);
    let expected_query_result = orders_nominal_total_value
        * (FPDecimal::one()
            - FPDecimal::must_from_str(&format!(
                "{}",
                DEFAULT_TAKER_FEE * DEFAULT_ATOMIC_MULTIPLIER * DEFAULT_SELF_RELAYING_FEE_PART
            )));
    assert_eq!(
        query_result.unwrap(),
        expected_query_result,
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: USDT.to_string(),
            min_quantity: FPDecimal::from(2357458u128),
        },
        &[coin(12, ETH)],
        &swapper,
    )
    .unwrap();

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, USDT, swapper.address().as_str());
    let expected_execute_result = expected_query_result.int();

    assert_eq!(
        from_balance,
        FPDecimal::zero(),
        "some of the original amount wasn't swapped"
    );
    assert_eq!(
        to_balance, expected_execute_result,
        "swapper did not receive expected amount"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn happy_path_simple_buy_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(100_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        USDT,
        vec![spot_market_1_id.as_str().into()],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(
        &app,
        &trader1,
        &spot_market_1_id,
        OrderSide::Sell,
        201_000,
        5,
    );
    create_limit_order(
        &app,
        &trader2,
        &spot_market_1_id,
        OrderSide::Sell,
        195_000,
        4,
    );
    create_limit_order(
        &app,
        &trader2,
        &spot_market_1_id,
        OrderSide::Sell,
        192_000,
        3,
    );

    app.increase_time(1);

    let swapper_usdt = 2_360_995;
    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(swapper_usdt, USDT),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    // calculate how much ETH we can buy with USDT we have
    let available_usdt_after_fee = FPDecimal::from(swapper_usdt)
        / (FPDecimal::one()
            + FPDecimal::must_from_str(&format!(
                "{}",
                DEFAULT_TAKER_FEE * DEFAULT_ATOMIC_MULTIPLIER * DEFAULT_SELF_RELAYING_FEE_PART
            )));
    let usdt_left_for_most_expensive_order = available_usdt_after_fee
        - (FPDecimal::from(195_000u128) * FPDecimal::from(4u128)
            + FPDecimal::from(192_000u128) * FPDecimal::from(3u128));
    let most_expensive_order_quantity =
        usdt_left_for_most_expensive_order / FPDecimal::from(201000u128);
    let expected_quantity =
        most_expensive_order_quantity + (FPDecimal::from(4u128) + FPDecimal::from(3u128));

    // round to min tick
    let expected_quantity_rounded =
        round_to_min_tick(expected_quantity, FPDecimal::must_from_str("0.001"));

    // calculate dust notional value as this will be the portion of user's funds that will stay in the contract
    let dust = expected_quantity - expected_quantity_rounded;
    // we need to use worst priced order
    let dust_value = dust * FPDecimal::from(201_000u128);

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: USDT.to_string(),
            to_denom: ETH.to_string(),
            from_quantity: FPDecimal::from(swapper_usdt),
        },
    );
    assert_eq!(
        query_result.unwrap(),
        expected_quantity_rounded,
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ETH.to_string(),
            min_quantity: FPDecimal::from(11u128),
        },
        &[coin(swapper_usdt, USDT)],
        &swapper,
    )
    .unwrap();

    let from_balance = query_bank_balance(&bank, USDT, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let expected_execute_result = expected_quantity.int();

    assert_eq!(
        from_balance,
        FPDecimal::zero(),
        "some of the original amount wasn't swapped"
    );
    assert_eq!(
        to_balance, expected_execute_result,
        "swapper did not receive expected amount"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    let mut expected_contract_balances_after =
        FPDecimal::must_from_str(contract_balances_before[0].amount.as_str()) + dust_value;
    expected_contract_balances_after = expected_contract_balances_after.int();

    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        FPDecimal::must_from_str(contract_balances_after[0].amount.as_str()),
        expected_contract_balances_after,
        "contract balance changed unexpectedly after swap"
    );
}

#[test]
fn happy_path_external_fee_receiver() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let fee_recipient = must_init_account_with_funds(&app, &[]);
    let contr_addr = init_contract_with_fee_recipient_and_get_address(
        &wasm,
        &owner,
        &[coin(10_000_000_000, USDT)],
        &fee_recipient,
    );
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    let buy_orders_nominal_total_value = FPDecimal::from(201000u128) * FPDecimal::from(5u128)
        + FPDecimal::from(195000u128) * FPDecimal::from(4u128)
        + FPDecimal::from(192000u128) * FPDecimal::from(3u128);
    let relayer_sell_fee = buy_orders_nominal_total_value
        * FPDecimal::must_from_str(&format!(
            "{}",
            DEFAULT_TAKER_FEE * DEFAULT_ATOMIC_MULTIPLIER * DEFAULT_RELAYER_SHARE
        ));

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    let expected_nominal_buy_most_expensive_match_quantity =
        FPDecimal::must_from_str("488.2222155454736648");
    let sell_orders_nominal_total_value = FPDecimal::from(800u128) * FPDecimal::from(800u128)
        + FPDecimal::from(810u128) * FPDecimal::from(800u128)
        + FPDecimal::from(820u128) * FPDecimal::from(800u128)
        + FPDecimal::from(830u128) * expected_nominal_buy_most_expensive_match_quantity;
    let relayer_buy_fee = sell_orders_nominal_total_value
        * FPDecimal::must_from_str(&format!(
            "{}",
            DEFAULT_TAKER_FEE * DEFAULT_ATOMIC_MULTIPLIER * DEFAULT_RELAYER_SHARE
        ));
    let expected_fee_for_fee_recipient = relayer_buy_fee + relayer_sell_fee;

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: FPDecimal = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        )
        .unwrap();
    assert_eq!(
        query_result,
        FPDecimal::must_from_str("2888.222"),
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2888u128),
        },
        &[coin(12, ETH)],
        &swapper,
    )
    .unwrap();

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::zero(),
        "some of the original amount wasn't swapped"
    );
    assert_eq!(
        to_balance,
        FPDecimal::must_from_str("2888"),
        "swapper did not receive expected amount"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );

    let fee_recipient_balance = query_all_bank_balances(&bank, &fee_recipient.address());
    assert_eq!(
        fee_recipient_balance.len(),
        1,
        "wrong number of denoms in fee recipient's balances"
    );
    assert_eq!(
        fee_recipient_balance[0].denom, USDT,
        "fee recipient did not receive fee in expected denom"
    );
    assert_eq!(
        FPDecimal::must_from_str(fee_recipient_balance[0].amount.as_str()),
        expected_fee_for_fee_recipient.int(),
        "fee recipient did not receive expected fee"
    );
}

#[test]
fn not_enough_buffer() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000, USDT)]);

    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );
    assert_eq!(
        query_result.unwrap_err(),
        RunnerError::QueryError {
            msg: "Generic error: Swap amount too high: query wasm contract failed".to_string()
        },
        "wrong error message"
    );

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    );
    let expected_error = RunnerError::ExecuteError {msg: "failed to execute message; message index: 0: dispatch: submessages: reply: Generic error: Swap amount too high: execute wasm contract failed".to_string()};
    assert_eq!(
        execute_result.unwrap_err(),
        expected_error,
        "wrong error message"
    );

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after swap"
    );
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn no_funds_passed() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);

    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[],
        &swapper,
    );
    let expected_error = RunnerError::ExecuteError {msg: "failed to execute message; message index: 0: Custom Error: \"Only 1 denom can be passed in funds, but 0 were found\": execute wasm contract failed".to_string()};
    assert_eq!(
        execute_result.unwrap_err(),
        expected_error,
        "wrong error message"
    );

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after swap"
    );
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn multiple_fund_denmos_passed() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);

    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let eth_balance = 12u128;
    let atom_balance = 10u128;

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(eth_balance, ETH),
            coin(atom_balance, ATOM),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(10u128),
        },
        &[coin(10, ATOM), coin(12, ETH)],
        &swapper,
    );
    assert!(
        execute_result
            .unwrap_err()
            .to_string()
            .contains("Only 1 denom can be passed in funds"),
        "wrong error message"
    );

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(eth_balance),
        "wrong ETH balance after failed swap"
    );
    assert_eq!(
        to_balance,
        FPDecimal::from(atom_balance),
        "wrong ATOM balance after failed swap"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn zero_minimum_amount_to_receive() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);

    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(0u128),
        },
    );
    assert!(
        query_result
            .unwrap_err()
            .to_string()
            .contains("Generic error: from_quantity must be positive: query wasm contract failed"),
        "incorrect error returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    let err = wasm
        .execute(
            &contr_addr,
            &ExecuteMsg::Swap {
                target_denom: ATOM.to_string(),
                min_quantity: FPDecimal::zero(),
            },
            &[coin(12, ETH)],
            &swapper,
        )
        .unwrap_err();
    assert!(
        err.to_string()
            .contains("Min target quantity must be positive"),
        "incorrect error returned by execute"
    );

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::must_from_str("12"),
        "swap should not have occurred"
    );
    assert_eq!(
        to_balance,
        FPDecimal::must_from_str("0"),
        "swapper should not have received any target tokens"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn negative_minimum_amount_to_receive() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);

    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::must_from_str("-1"),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert!(
        execute_result.is_err(),
        "swap with negative minimum amount to receive did not fail"
    );
    println!("error: {:?}", execute_result.err().unwrap());

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after failed swap"
    );
    assert_eq!(
        to_balance,
        FPDecimal::zero(),
        "wrong to balance after failed swap"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after failed swap"
    );
}

#[test]
fn not_enough_orders_to_satisfy_min_quantity() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    // set the market
    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 450); //not enough for minimum requested

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );
    assert_eq!(
        query_result.unwrap_err(),
        QueryError {
            msg: "Generic error: Not enough liquidity to fulfill order: query wasm contract failed"
                .to_string()
        },
        "wrong error message"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert_eq!(execute_result.unwrap_err(), RunnerError::ExecuteError { msg: "failed to execute message; message index: 0: dispatch: submessages: reply: Generic error: Not enough liquidity to fulfill order: execute wasm contract failed".to_string() }, "wrong error message");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after swap"
    );
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

#[test]
fn min_quantity_cannot_be_reached() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    // set the market
    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    let min_quantity = 3500u128;
    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(min_quantity),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert_eq!(execute_result.unwrap_err(), RunnerError::ExecuteError { msg: format!("failed to execute message; message index: 0: dispatch: submessages: reply: dispatch: submessages: reply: Min expected swap amount ({min_quantity}) not reached: execute wasm contract failed") }, "wrong error message");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after failed swap"
    );
    assert_eq!(
        to_balance,
        FPDecimal::zero(),
        "wrong to balance after failed swap"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after failed swap"
    );
}

#[test]
fn no_known_route_exists() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    // set the market
    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        SOL,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 450); //not enough for minimum requested

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );
    assert_eq!(query_result.unwrap_err(), QueryError { msg: "Generic error: No swap route not found from eth to atom: query wasm contract failed".to_string() }, "wrong error message");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert_eq!(execute_result.unwrap_err(), RunnerError::ExecuteError { msg: "failed to execute message; message index: 0: Generic error: No swap route not found from eth to atom: execute wasm contract failed".to_string() }, "wrong error message");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after swap"
    );
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after swap"
    );
}

//TODO better error if market doesn't exist?
#[test]
fn route_exists_but_market_does_not() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b";

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![spot_market_1_id.as_str().into(), spot_market_2_id.into()],
    );

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );
    assert!(
        query_result
            .unwrap_err()
            .to_string()
            .contains("market should be available"),
        "wrong error returned"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    println!("contract balances before: {:?}", contract_balances_before);

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert!(
        execute_result
            .unwrap_err()
            .to_string()
            .contains("market should be available"),
        "wrong error returned"
    );

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after failed swap"
    );
    assert_eq!(
        to_balance,
        FPDecimal::zero(),
        "wrong to balance after failed swap"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after failed swap"
    );
}

#[test]
fn paused_market() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);
    let gov = Gov::new(&app);

    let signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    fund_account_with_some_inj(&bank, &signer, &validator);
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    pause_spot_market(&gov, spot_market_1_id.as_str(), &signer, &validator);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let swapper = must_init_account_with_funds(
        &app,
        &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );

    assert!(
        query_result
            .unwrap_err()
            .to_string()
            .contains("Querier contract error"),
        "wrong error returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert!(
        execute_result
            .unwrap_err()
            .to_string()
            .contains("Querier contract error"),
        "wrong error returned by execute"
    );

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after failed swap"
    );
    assert_eq!(
        to_balance,
        FPDecimal::zero(),
        "wrong to balance after failed swap"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after failed swap"
    );
}

#[test]
fn insufficient_gas() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let signer =
        must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    fund_account_with_some_inj(&bank, &signer, &validator);
    let owner = must_init_account_with_funds(
        &app,
        &[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let spot_market_1_id = launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(
        &wasm,
        &owner,
        &contr_addr,
        ETH,
        ATOM,
        vec![
            spot_market_1_id.as_str().into(),
            spot_market_2_id.as_str().into(),
        ],
    );

    let swapper = must_init_account_with_funds(&app, &[coin(12, ETH), coin(10, INJ)]);

    let trader1 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader2 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    let trader3 = must_init_account_with_funds(
        &app,
        &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ],
    );

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let query_result: RunnerResult<FPDecimal> = wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: ETH.to_string(),
            to_denom: ATOM.to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    );

    assert_eq!(
        query_result.unwrap(),
        FPDecimal::must_from_str("2893.888"),
        "incorrect swap result estimate returned by query"
    );

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(
        contract_balances_before.len(),
        1,
        "wrong number of denoms in contract balances"
    );

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[coin(12, ETH)],
        &swapper,
    );

    assert_eq!(execute_result.unwrap_err(), ExecuteError {msg: "spendable balance 10inj is smaller than 2500inj: insufficient funds: insufficient funds".to_string()}, "wrong error returned by execute");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(
        from_balance,
        FPDecimal::from(12u128),
        "wrong from balance after failed swap"
    );
    assert_eq!(
        to_balance,
        FPDecimal::zero(),
        "wrong to balance after failed swap"
    );

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(
        contract_balances_after.len(),
        1,
        "wrong number of denoms in contract balances"
    );
    assert_eq!(
        contract_balances_after, contract_balances_before,
        "contract balance has changed after failed swap"
    );
}
