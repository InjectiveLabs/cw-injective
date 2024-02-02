use crate::msg::InstantiateMsg;
use cosmwasm_std::{coin, Addr, Coin};
use injective_cosmwasm::{checked_address_to_subaccount_id, SubaccountId};
use injective_math::{scale::Scaled, FPDecimal};
use injective_test_tube::{Account, InjectiveTestApp, Module, SigningAccount, Wasm};
use std::collections::HashMap;

pub const EXCHANGE_DECIMALS: i32 = 18i32;
pub const BASE_DECIMALS: i32 = 18i32;
pub const ATOM_DECIMALS: i32 = 8i32;
pub const QUOTE_DECIMALS: i32 = 6i32;

pub const ATOM_DENOM: &str = "atom";
pub const BASE_DENOM: &str = "inj";
pub const QUOTE_DENOM: &str = "usdt";

pub struct UserInfo {
    pub account: SigningAccount,
    pub subaccount_id: SubaccountId,
}
pub struct Setup {
    pub app: InjectiveTestApp,
    pub owner: SigningAccount,
    pub signer: SigningAccount,
    pub users: Vec<UserInfo>,
    pub denoms: HashMap<String, String>,
    pub contract_address: String,
    pub code_id: u64,
}

impl Setup {
    pub fn new() -> Self {
        let app = InjectiveTestApp::new();
        let wasm = Wasm::new(&app);

        let mut denoms = HashMap::new();
        denoms.insert("atom".to_string(), ATOM_DENOM.to_string());
        denoms.insert("quote".to_string(), QUOTE_DENOM.to_string());
        denoms.insert("base".to_string(), BASE_DENOM.to_string());

        let signer = app.init_account(&[str_coin("1000000", BASE_DENOM, BASE_DECIMALS)]).unwrap();

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

            let user_subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(user.address()), 1u32);

            users.push(UserInfo {
                account: user,
                subaccount_id: user_subaccount_id,
            });
        }

        let wasm_byte_code = std::fs::read(wasm_file("injective_cosmwasm_mock".to_string())).unwrap();
        let code_id = wasm.store_code(&wasm_byte_code, None, &owner).unwrap().data.code_id;

        // Instantiate contract
        let contract_address: String = wasm
            .instantiate(code_id, &InstantiateMsg {}, Some(&owner.address()), Some("mock-contract"), &[], &owner)
            .unwrap()
            .data
            .address;

        assert!(!contract_address.is_empty(), "Contract address is empty");

        Self {
            app,
            owner,
            signer,
            users,
            denoms,
            contract_address,
            code_id,
        }
    }
}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
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

pub fn human_to_proto(raw_number: &str, decimals: i32) -> String {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(18 + decimals).to_string()
}

pub fn human_to_dec(raw_number: &str, decimals: i32) -> FPDecimal {
    FPDecimal::must_from_str(&raw_number.replace('_', "")).scaled(decimals)
}
