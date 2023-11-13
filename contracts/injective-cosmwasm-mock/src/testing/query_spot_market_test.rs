use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::{Addr, Binary, Coin};
use injective_cosmwasm::{checked_address_to_subaccount_id, MarketId};
use injective_test_tube::{Account, InjectiveTestApp, Module, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_instantiation() {
    let app = InjectiveTestApp::new();
    // create new account with initial funds
    let accs = app
        .init_accounts(&[Coin::new(1_000_000_000_000, "usdt"), Coin::new(1_000_000_000_000, "inj")], 2)
        .unwrap();

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
        .instantiate(code_id, &InstantiateMsg {}, Some(&seller.address()), Some("mock-contract"), &[], &seller)
        .unwrap()
        .data
        .address;

    // Execute contract
    let buyer_subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(buyer.address()), 1u32);
    let _res = wasm
        .execute(
            &contract_address,
            &ExecuteMsg::TestDepositMsg {
                subaccount_id: buyer_subaccount_id,
                amount: Coin::new(100, "usdt"),
            },
            &[Coin::new(100, "usdt")],
            &buyer,
        )
        .unwrap();

    // Query contract
    let _res: Binary = wasm
        .query(
            &contract_address,
            &QueryMsg::TestSpotMarketQuery {
                market_id: MarketId::unchecked("0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b"),
            },
        )
        .unwrap();
}
