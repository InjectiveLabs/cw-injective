use std::str::FromStr;

use cosmwasm_std::{coin, from_binary, Addr};
use injective_std::types::cosmos::bank::v1beta1::{QueryAllBalancesRequest, QueryBalanceRequest};
use injective_test_tube::{Account, Bank, Exchange, InjectiveTestApp, Module, Runner, Wasm};

use injective_cosmwasm::MarketId;
use injective_math::FPDecimal;

use crate::msg::{ExecuteMsg, FeeRecipient, InstantiateMsg, QueryMsg};
use crate::testing::test_utils::{create_limit_order, launch_spot_market, store_code};

#[test]
fn basic_swap_test() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let bank = Bank::new(&app);

    let coin_from = "eth";
    let coin_to = "atom";
    let quote = "usdt";
    let inj = "inj";
    let _signer = app
        .init_account(&[coin(1_000_000_000_000_000_000_000_000, inj)])
        .unwrap();

    let _validator = app
        .get_first_validator_signing_account(inj.to_string(), 1.2f64)
        .unwrap();
    let owner = app
        .init_account(&[
            coin(1_000_000_000_000_000_000_000_000, coin_from),
            coin(1_000_000_000_000_000_000_000_000, coin_to),
            coin(1_000_000_000_000, quote.clone()),
            coin(1_000_000_000_000_000_000_000_000, inj),
        ])
        .unwrap();

    // set the market
    let spot_market_1_id =
        launch_spot_market(&exchange, &owner, coin_from.to_string(), quote.to_string());
    let spot_market_2_id =
        launch_spot_market(&exchange, &owner, coin_to.to_string(), quote.to_string());

    let code_id = store_code(&wasm, &owner, "helix_converter".to_string());
    let contr_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                fee_recipient: FeeRecipient::SwapContract,
                admin: Addr::unchecked(owner.address()),
            },
            Some(&owner.address()),
            Some("Swap"),
            &vec![coin(10_000_000_000, quote)],
            &owner,
        )
        .unwrap()
        .data
        .address;

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::SetRoute {
            denom_1: coin_from.to_string(),
            denom_2: coin_to.to_string(),
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
            coin(10_000_000_000_000_000_000_000_000, coin_from),
            coin(123456_000_000_000_000_000_000_000_000, quote),
            coin(9999_000_000_000_000_000_000_000_000, coin_to),
            coin(10_000_000_000_000_000_000_000_000, inj),
        ])
        .unwrap();

    let trader2 = app
        .init_account(&[
            coin(10_000_000_000_000_000_000_000_000, coin_from),
            coin(123456_000_000_000_000_000_000_000_000, quote),
            coin(9999_000_000_000_000_000_000_000_000, coin_to),
            coin(10_000_000_000_000_000_000_000_000, inj),
        ])
        .unwrap();

    let trader3 = app
        .init_account(&[
            coin(10_000_000_000_000_000_000_000_000, coin_from),
            coin(123456_000_000_000_000_000_000_000_000, quote),
            coin(9999_000_000_000_000_000_000_000_000, coin_to),
            coin(10_000_000_000_000_000_000_000_000, inj),
        ])
        .unwrap();

    create_limit_order(&app, &trader1, &spot_market_1_id, true, 201000, 5);
    create_limit_order(&app, &trader2, &spot_market_1_id, true, 195000, 4);
    create_limit_order(&app, &trader2, &spot_market_1_id, true, 192000, 3);

    create_limit_order(&app, &trader1, &spot_market_2_id, false, 800, 800);
    create_limit_order(&app, &trader2, &spot_market_2_id, false, 810, 800);
    create_limit_order(&app, &trader3, &spot_market_2_id, false, 820, 800);
    create_limit_order(&app, &trader1, &spot_market_2_id, false, 830, 800);

    app.increase_time(10);

    let swapper = app
        .init_account(&[
            coin(12, coin_from),
            coin(5_000_000_000_000_000_000_000_000_000, inj),
        ])
        .unwrap();

    let query_result: FPDecimal = wasm
        .query(
            &contr_addr,
            &QueryMsg::GetExecutionQuantity {
                from_denom: coin_from.to_string(),
                to_denom: coin_to.to_string(),
                from_quantity: FPDecimal::from(12u128),
            },
        )
        .unwrap();
    assert_eq!(query_result, FPDecimal::from_str("2893.888").unwrap());

    let contract_balances = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: contr_addr.clone(),
            pagination: None,
        })
        .unwrap()
        .balances;
    println!("contract balances before: {:?}", contract_balances);

    wasm.execute(
        &contr_addr,
        &ExecuteMsg::Swap {
            target_denom: coin_to.to_string(),
            min_quantity: FPDecimal::from(2800u128),
        },
        &vec![coin(12, coin_from)],
        &swapper,
    )
    .unwrap();

    let from_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: swapper.address(),
            denom: coin_from.to_string(),
        })
        .unwrap()
        .balance
        .unwrap()
        .amount;
    let to_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: swapper.address(),
            denom: coin_to.to_string(),
        })
        .unwrap()
        .balance
        .unwrap()
        .amount;
    assert_eq!(from_balance, "0");
    assert_eq!(to_balance, "2893");

    let contract_balances = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: contr_addr.clone(),
            pagination: None,
        })
        .unwrap()
        .balances;

    assert_eq!(contract_balances.len(), 1);
    assert_eq!(contract_balances[0].amount, "10000000000");
}
