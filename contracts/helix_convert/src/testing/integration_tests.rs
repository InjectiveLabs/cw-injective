use std::str::FromStr;

use cosmwasm_std::{coin};

use injective_test_tube::{Account, Bank, Exchange, InjectiveTestApp, Module, Runner, RunnerError, RunnerResult, Wasm};
use injective_test_tube::RunnerError::{QueryError};


use injective_math::{FPDecimal, round_to_min_tick};

use crate::msg::{ExecuteMsg, QueryMsg};
use crate::testing::test_utils::{create_limit_order, launch_spot_market, OrderSide, init_contract_and_get_address, must_init_account_with_funds, query_all_bank_balances, query_bank_balance, set_route_and_assert_success};


const ETH: &str = "eth";
const ATOM: &str = "atom";
const SOL: &str = "sol";
const USDT: &str = "usdt";
const INJ: &str = "inj";

const DEFAULT_TAKER_FEE: f64 = 0.001;
const DEFAULT_ATOMIC_MULTIPLIER: f64 = 2.5;
const DEFAULT_SELF_RELAYING_FEE_PART: f64 = 0.6;

#[test]
fn happy_path_two_hops_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
            coin(1_000_000_000_000_000_000_000_000, ETH),
            coin(1_000_000_000_000_000_000_000_000, ATOM),
            coin(1_000_000_000_000, USDT),
            coin(1_000_000_000_000_000_000_000_000, INJ),
        ]);

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 =must_init_account_with_funds(&app,&[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ]);

    let trader2 = must_init_account_with_funds(&app,&[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ]);

    let trader3 = must_init_account_with_funds(&app,&[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app,&[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ]);

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
    assert_eq!(query_result, FPDecimal::must_from_str("2893.888"), "incorrect swap result estimate returned by query");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
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
    assert_eq!(from_balance, FPDecimal::zero(), "some of the original amount wasn't swapped");
    assert_eq!(to_balance, FPDecimal::must_from_str("2893"), "swapper did not receive expected amount");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn happy_path_simple_sell_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, USDT, vec![spot_market_1_id.as_str().into()]);

    let trader1 =must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app,&[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: USDT.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    let orders_nominal_total_value = FPDecimal::from(201000u128) * FPDecimal::from(5u128) + FPDecimal::from(195000u128) * FPDecimal::from(4u128) + FPDecimal::from(192000u128) * FPDecimal::from(3u128);
    let expected_query_result = orders_nominal_total_value * (FPDecimal::one() - FPDecimal::must_from_str(&format!("{}", DEFAULT_TAKER_FEE * DEFAULT_ATOMIC_MULTIPLIER * DEFAULT_SELF_RELAYING_FEE_PART)));
    assert_eq!(query_result.unwrap(), expected_query_result, "incorrect swap result estimate returned by query");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");

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

    assert_eq!(from_balance, FPDecimal::zero(), "some of the original amount wasn't swapped");
    assert_eq!(to_balance, expected_execute_result, "swapper did not receive expected amount");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn happy_path_simple_buy_swap() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(100_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, USDT, vec![spot_market_1_id.as_str().into()]);

    let trader1 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Sell, 201_000, 6);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Sell, 195_000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Sell, 192_000, 3);

    app.increase_time(1);

    // let swapper_usdt = 2_360_995;
    let swapper_usdt : u128 = (FPDecimal::from(2_362_014u128) * FPDecimal::from_str("1.0015").unwrap()).into();
    let swapper = must_init_account_with_funds(&app,&[
        coin(swapper_usdt, USDT),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let available_usdt_after_fee = FPDecimal::from(swapper_usdt) / (FPDecimal::one() + FPDecimal::must_from_str(&format!("{}", DEFAULT_TAKER_FEE * DEFAULT_ATOMIC_MULTIPLIER * DEFAULT_SELF_RELAYING_FEE_PART)));
    println!("available_usdt_after_fee: {}", available_usdt_after_fee);
    let usdt_left_for_most_expensive_order = available_usdt_after_fee - (FPDecimal::from(195_000u128) * FPDecimal::from(4u128) + FPDecimal::from(192_000u128) * FPDecimal::from(3u128));
    println!("usdt_left_for_most_expensive_order: {}", usdt_left_for_most_expensive_order);
    let most_expensive_order_quantity = usdt_left_for_most_expensive_order / FPDecimal::from(201000u128);
    println!("most_expensive_order_quantity: {}", most_expensive_order_quantity);
    let expected_quantity = most_expensive_order_quantity + (FPDecimal::from(4u128) + FPDecimal::from(3u128));
    println!("expected_quantity: {}", expected_quantity);
    let expected_quantity_rounded = round_to_min_tick(
        expected_quantity,
            FPDecimal::must_from_str("0.001"),
    );
    println!("expected_quantity: {}", expected_quantity_rounded);
    let dust = expected_quantity - expected_quantity_rounded;
    println!("tick size-related dust: {}", dust);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: USDT.to_string(),
                to_denom: ETH.to_string(),
                from_quantity: FPDecimal::from(swapper_usdt)
            },
        );
    assert_eq!(query_result.unwrap(), expected_quantity_rounded, "incorrect swap result estimate returned by query");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");

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
    println!("from_balance: {}", from_balance);
    let to_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    println!("to_balance: {}", to_balance);
    let expected_execute_result = expected_quantity.int();

    assert_eq!(from_balance, FPDecimal::zero(), "some of the original amount wasn't swapped");
    assert_eq!(to_balance, expected_execute_result, "swapper did not receive expected amount");

    let dust_value = dust * FPDecimal::from(201_000u128);
    println!("{}", &format!("dust_value: {dust_value}, before + dust_value: {}", FPDecimal::must_from_str(contract_balances_before[0].amount.as_str()) + dust_value));

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    let mut expected_contract_balances_after = FPDecimal::must_from_str(contract_balances_before[0].amount.as_str()) + dust_value;
    expected_contract_balances_after = expected_contract_balances_after.int();

    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(FPDecimal::must_from_str(contract_balances_after[0].amount.as_str()), expected_contract_balances_after, "contract balance changed unexpectedly after swap"); //76 diff comes from funds passed - available funds (without fee)
}

#[test]
fn not_enough_buffer() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app, &[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000, USDT)]);

    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 = must_init_account_with_funds(&app, &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ]);

    let trader2 = must_init_account_with_funds(&app, &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ]);

    let trader3 = must_init_account_with_funds(&app, &[
            coin(10_000_000_000_000_000_000_000_000, ETH),
            coin(123_456_000_000_000_000_000_000_000_000, USDT),
            coin(9_999_000_000_000_000_000_000_000_000, ATOM),
            coin(10_000_000_000_000_000_000_000_000, INJ),
        ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app, &[
            coin(12, ETH),
            coin(5_000_000_000_000_000_000_000_000_000, INJ),
        ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    assert_eq!(query_result.unwrap_err(), RunnerError::QueryError {msg: "Generic error: Swap amount too high: query wasm contract failed".to_string()}, "wrong error message");

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
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
    assert_eq!(execute_result.unwrap_err(), expected_error, "wrong error message");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(from_balance, FPDecimal::from(12u128), "wrong from balance after swap");
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn no_funds_passed() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app, &[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);

    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 = must_init_account_with_funds(&app, &[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app, &[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader3 = must_init_account_with_funds(&app, &[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app, &[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    assert_eq!(query_result.unwrap(), FPDecimal::must_from_str("2893.888"), "incorrect swap result estimate returned by query");

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
    println!("contract balances before: {:?}", contract_balances_before);

    let execute_result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &[],
        &swapper,
    );
    let expected_error = RunnerError::ExecuteError {msg: "failed to execute message; message index: 0: Custom Error val: \"Wrong amount of funds deposited!\": execute wasm contract failed".to_string()};
    assert_eq!(execute_result.unwrap_err(), expected_error, "wrong error message");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(from_balance, FPDecimal::from(12u128), "wrong from balance after swap");
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn zero_minimum_amount_to_receive() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app, &[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.into(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app, &[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);

    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 = must_init_account_with_funds(&app, &[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app, &[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader3 = must_init_account_with_funds(&app, &[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app, &[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    assert_eq!(query_result.unwrap(), FPDecimal::must_from_str("2893.888"), "incorrect swap result estimate returned by query");

    let contract_balances_before = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
    println!("contract balances before: {:?}", contract_balances_before);

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: ATOM.to_string(),
            min_quantity: FPDecimal::zero(),
        },
        &[coin(12, ETH)],
        &swapper,
    )
        .unwrap();

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(from_balance, FPDecimal::zero(), "some of the original amount wasn't swapped");
    assert_eq!(to_balance, FPDecimal::must_from_str("2893"), "swapper did not receive expected amount");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn not_enough_orders_to_satisfy_min_quantity() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 =must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader3 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 450); //not enough for minimum requested

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app,&[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    assert_eq!(query_result.unwrap_err(), QueryError { msg: "Generic error: Not enough liquidity to fulfill order: query wasm contract failed".to_string() }, "wrong error message");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
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
    assert_eq!(from_balance, FPDecimal::from(12u128), "wrong from balance after swap");
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn no_known_route_exists() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, SOL, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 =must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader3 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 450); //not enough for minimum requested

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app,&[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    assert_eq!(query_result.unwrap_err(), QueryError { msg: "Generic error: No swap route not found from eth to atom: query wasm contract failed".to_string() }, "wrong error message");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
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
    assert_eq!(from_balance, FPDecimal::from(12u128), "wrong from balance after swap");
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

//TODO better error if market doesn't exist?
#[test]
fn route_exists_but_market_does_not() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id = "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b";

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.into()]);

    let trader1 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let _trader3 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app,&[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

    let query_result: RunnerResult<FPDecimal> = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: ETH.to_string(),
                to_denom: ATOM.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        );
    assert!(query_result.unwrap_err().to_string().contains("market should be available"), "wrong error returned");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
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

    assert!(execute_result.unwrap_err().to_string().contains("market should be available"), "wrong error returned");

    let from_balance = query_bank_balance(&bank, ETH, swapper.address().as_str());
    let to_balance = query_bank_balance(&bank, ATOM, swapper.address().as_str());
    assert_eq!(from_balance, FPDecimal::from(12u128), "wrong from balance after swap");
    assert_eq!(to_balance, FPDecimal::zero(), "wrong to balance after swap");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}

#[test]
fn paused_market() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let _signer = must_init_account_with_funds(&app,&[coin(1_000_000_000_000_000_000_000_000, INJ)]);

    let _validator = app
        .get_first_validator_signing_account(INJ.to_string(), 1.2f64)
        .unwrap();
    let owner = must_init_account_with_funds(&app,&[
        coin(1_000_000_000_000_000_000_000_000, ETH),
        coin(1_000_000_000_000_000_000_000_000, ATOM),
        coin(1_000_000_000_000, USDT),
        coin(1_000_000_000_000_000_000_000_000, INJ),
    ]);

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, ETH, USDT);
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, ATOM, USDT);

    //TODO: pause market

    let contr_addr = init_contract_and_get_address(&wasm, &owner, &[coin(10_000_000_000, USDT)]);
    set_route_and_assert_success(&wasm, &owner, &contr_addr, ETH, ATOM, vec![spot_market_1_id.as_str().into(), spot_market_2_id.as_str().into()]);

    let trader1 =must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader2 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    let trader3 = must_init_account_with_funds(&app,&[
        coin(10_000_000_000_000_000_000_000_000, ETH),
        coin(123_456_000_000_000_000_000_000_000_000, USDT),
        coin(9_999_000_000_000_000_000_000_000_000, ATOM),
        coin(10_000_000_000_000_000_000_000_000, INJ),
    ]);

    create_limit_order(&app, &trader1, &spot_market_1_id, OrderSide::Buy, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, OrderSide::Buy, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, OrderSide::Sell, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, OrderSide::Sell, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, OrderSide::Sell, 830, 800);

    app.increase_time(1);

    let swapper = must_init_account_with_funds(&app,&[
        coin(12, ETH),
        coin(5_000_000_000_000_000_000_000_000_000, INJ),
    ]);

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
    assert_eq!(query_result, FPDecimal::must_from_str("2893.888"), "incorrect swap result estimate returned by query");

    let contract_balances_before = query_all_bank_balances(&bank, &contr_addr);
    assert_eq!(contract_balances_before.len(), 1, "wrong number of denoms in contract balances");
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
    assert_eq!(from_balance, FPDecimal::zero(), "some of the original amount wasn't swapped");
    assert_eq!(to_balance, FPDecimal::must_from_str("2893"), "swapper did not receive expected amount");

    let contract_balances_after = query_all_bank_balances(&bank, contr_addr.as_str());
    assert_eq!(contract_balances_after.len(), 1, "wrong number of denoms in contract balances");
    assert_eq!(contract_balances_after, contract_balances_before, "contract balance has changed after swap");
}
