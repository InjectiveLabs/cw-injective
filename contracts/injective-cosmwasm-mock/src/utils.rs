use crate::msg::InstantiateMsg;
use cosmwasm_std::{coin, Addr, Coin};
use injective_cosmwasm::{checked_address_to_subaccount_id, SubaccountId};
use injective_math::{scale::Scaled, FPDecimal};
use injective_test_tube::{Account, Bank, Gov, InjectiveTestApp, Insurance, Module, Oracle, SigningAccount, Wasm};

use prost::Message;
use std::{collections::HashMap, str::FromStr};

use injective_std::types::injective::insurance::v1beta1::MsgCreateInsuranceFund;
use injective_std::types::injective::oracle::v1beta1::OracleType;
use injective_std::{
    shim::Any,
    types::{
        cosmos::{
            bank::v1beta1::MsgSend,
            base::v1beta1::Coin as BaseCoin,
            gov::{v1::MsgVote, v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1},
        },
        injective::oracle::v1beta1::{GrantPriceFeederPrivilegeProposal, MsgRelayPriceFeedPrice},
    },
};

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
    pub validator: SigningAccount,
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

        Self {
            app,
            owner,
            signer,
            validator,
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
