use cosmwasm_std::{coin, from_binary};
use injective_test_tube::{Account, Exchange, InjectiveTestApp, Module, Runner, Wasm};

use injective_cosmwasm::MarketId;
use injective_math::FPDecimal;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::testing::test_utils::{create_limit_order, launch_spot_market, store_code};

#[test]
fn basic_swap_test() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);

    let base = "inj".to_string();
    let _signer = app
        .init_account(&[coin(1_000_000_000_000_000_000_000_000, base.clone())])
        .unwrap();

    let _validator = app
        .get_first_validator_signing_account(base.clone(), 1.2f64)
        .unwrap();
    let owner = app
        .init_account(&[
            coin(1_000_000_000_000_000_000_000_000, base),
            coin(1_000_000_000_000, "usdt"),
            coin(1_000_000_000_000_000_000_000_000, "eth"),
        ])
        .unwrap();

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, "eth".to_string(), "usdt".to_string());
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, "inj".to_string(), "usdt".to_string());

    let code_id = store_code(&wasm, &owner, "helix_converter".to_string());
    let contr_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {},
            Some(&owner.address()),
            Some("Swap"),
            &vec![coin(10_000_000_000, "usdt")],
            &owner,
        )
        .unwrap()
        .data
        .address;

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::SetRoute {
            denom_1: "eth".to_string(),
            denom_2: "inj".to_string(),
            route: vec![
                MarketId::unchecked(spot_market_1_id.clone()),
                MarketId::unchecked(spot_market_2_id.clone()),
            ],
        },
        &vec![],
        &owner,
    )
    .unwrap();

    let trader1 = app
        .init_account(&[
            coin(10_000_000_000_000_000_000_000_000, "eth"),
            coin(123456_000_000_000_000_000_000_000_000, "usdt"),
            coin(9999_000_000_000_000_000_000_000_000, "inj"),
        ])
        .unwrap();

    let trader2 = app
        .init_account(&[
            coin(10_000_000_000_000_000_000_000_000, "eth"),
            coin(123456_000_000_000_000_000_000_000_000, "usdt"),
            coin(9999_000_000_000_000_000_000_000_000, "inj"),
        ])
        .unwrap();

    let trader3 = app
        .init_account(&[
            coin(10_000_000_000_000_000_000_000_000, "eth"),
            coin(123456_000_000_000_000_000_000_000_000, "usdt"),
            coin(9999_000_000_000_000_000_000_000_000, "inj"),
        ])
        .unwrap();

    create_limit_order(&app, &trader1, &spot_market_1_id, true, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, true, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, true, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, false,800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, false,810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, false,820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, false,830, 800);

    app.increase_time(10);

    let swapper = app
        .init_account(&[
            coin(12, "eth"),
            coin(5_000_000_000_000_000_000_000_000_000, "inj"),
        ])
        .unwrap();

    let query_result: FPDecimal =wasm.query(
        &contr_addr,
        &QueryMsg::GetExecutionQuantity {
            from_denom: "eth".to_string(),
            to_denom: "inj".to_string(),
            from_quantity: FPDecimal::from(12u128),
        },
    ).unwrap();

    let result = wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: "inj".to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &vec![coin(12, "eth")],
        &swapper,
    ).unwrap();


}
