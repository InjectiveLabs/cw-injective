use crate::msg::InstantiateMsg;
use cosmwasm_std::Coin;
use injective_test_tube::{Account, InjectiveTestApp, Module, Wasm};
use test_tube_inj::SigningAccount;

pub fn test_setup() -> (InjectiveTestApp, Vec<SigningAccount>, String) {
    let app = InjectiveTestApp::new();
    let mut accs = app
        .init_accounts(&[Coin::new(1_000_000_000_000, "usdt"), Coin::new(1_000_000_000_000, "inj")], 2)
        .unwrap();
    accs.push(app.init_account(&[Coin::new(1_000_000_000_000_000_000_000_000_000, "inj")]).unwrap());

    let seller = &accs[0];
    let buyer = &accs[1];

    // `Wasm` is the module we use to interact with cosmwasm releated logic on the appchain
    // it implements `Module` trait which you will see more later.
    let wasm = Wasm::new(&app);

    // Load compiled wasm bytecode
    let wasm_byte_code = std::fs::read("../../artifacts/injective_cosmwasm_mock-aarch64.wasm").unwrap();
    let code_id = wasm.store_code(&wasm_byte_code, None, buyer).unwrap().data.code_id;

    // Instantiate contract
    let contract_address: String = wasm
        .instantiate(code_id, &InstantiateMsg {}, Some(&seller.address()), Some("mock-contract"), &[], seller)
        .unwrap()
        .data
        .address;
    assert!(!contract_address.is_empty(), "Contract address is empty");

    (app, accs, contract_address)
}
