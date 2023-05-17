use cosmwasm_std::coin;
use injective_test_tube::{Exchange, InjectiveTestApp, Module, Wasm};
use crate::testing::test_utils::{launch_spot_market, store_code};

#[test]
fn basic_swap_test() {
    let app = InjectiveTestApp::new();
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);

    let base = "inj".to_string();
    let _signer = app.init_account(&[coin(1_000_000_000_000_000_000_000_000, base.clone())]).unwrap();

    let _validator = app.get_first_validator_signing_account(base.clone(), 1.2f64).unwrap();
    let owner = app
        .init_account(&[coin(1_000_000_000_000_000_000_000_000, base), coin(1_000_000, "usdt"), coin(1_000_000_000_000_000_000_000_000, "eth")])
        .unwrap();

    // set the market
    let _spot_market_1_id = launch_spot_market(&exchange, &owner, "inj".to_string(), "usdt".to_string());
    let _spot_market_2_id = launch_spot_market(&exchange, &owner, "eth".to_string(), "usdt".to_string());

    let _code_id = store_code(&wasm, &owner, "helix_converter".to_string());
}
