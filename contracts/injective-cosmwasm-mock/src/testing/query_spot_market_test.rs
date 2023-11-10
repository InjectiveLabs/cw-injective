use crate::msg::InstantiateMsg;
use cosmwasm_std::Coin;
use injective_test_tube::{Account, InjectiveTestApp, Module, Wasm};

use crate::msg::QueryMsg;

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_instantiation() {
    let app = InjectiveTestApp::new();
    // create new account with initial funds
    let accs = app
        .init_accounts(&[Coin::new(1_000_000_000_000, "usdt"), Coin::new(1_000_000_000_000, "inj")], 2)
        .unwrap();

    let admin = &accs[0];
    let new_admin = &accs[1];

    // `Wasm` is the module we use to interact with cosmwasm releated logic on the appchain
    // it implements `Module` trait which you will see more later.
    let wasm = Wasm::new(&app);

    // Load compiled wasm bytecode
    let wasm_byte_code = std::fs::read("../../artifacts/injective_cosmwasm_mock-aarch64.wasm").unwrap();
    let code_id = wasm.store_code(&wasm_byte_code, None, admin).unwrap().data.code_id;
    // instantiate contract with initial admin and make admin list mutable
}
