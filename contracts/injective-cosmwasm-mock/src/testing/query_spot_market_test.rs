use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::{Addr, Coin};
use injective_cosmwasm::{checked_address_to_subaccount_id, MarketId};
use injective_math::{scale::Scaled, FPDecimal};
use injective_std::types::injective::exchange::v1beta1::{MsgInstantSpotMarketLaunch, QuerySpotMarketsRequest};
use injective_test_tube::{injective_cosmwasm::SpotMarketResponse, Account, Exchange, InjectiveTestApp, Module, Wasm};

pub const BASE_DENOM: &str = "inj";
pub const QUOTE_DENOM: &str = "usdt";

pub fn dec_to_proto(val: FPDecimal) -> String {
    val.scaled(18).to_string()
}

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
    let signer = app.init_account(&[Coin::new(1_000_000_000_000_000_000_000_000_000, "inj")]).unwrap();

    // `Wasm` is the module we use to interact with cosmwasm releated logic on the appchain
    // it implements `Module` trait which you will see more later.
    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);

    // Load compiled wasm bytecode
    let wasm_byte_code = std::fs::read("../../artifacts/injective_cosmwasm_mock-aarch64.wasm").unwrap();
    let code_id = wasm.store_code(&wasm_byte_code, None, buyer).unwrap().data.code_id;

    // Instantiate contract
    let contract_address: String = wasm
        .instantiate(code_id, &InstantiateMsg {}, Some(&seller.address()), Some("mock-contract"), &[], &seller)
        .unwrap()
        .data
        .address;
    assert!(contract_address.len() > 0, "Contract address is empty");

    // Execute contract
    let buyer_subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(buyer.address()), 1u32);
    let res = wasm.execute(
        &contract_address,
        &ExecuteMsg::TestDepositMsg {
            subaccount_id: buyer_subaccount_id,
            amount: Coin::new(100, "usdt"),
        },
        &[Coin::new(100, "usdt")],
        &buyer,
    );
    assert!(res.is_ok(), "Execution failed with error: {:?}", res.unwrap_err());

    // Instantiate spot market
    let ticker = "INJ/USDT".to_string();
    exchange
        .instant_spot_market_launch(
            MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: ticker.clone(),
                base_denom: BASE_DENOM.to_string(),
                quote_denom: QUOTE_DENOM.to_string(),
                min_price_tick_size: dec_to_proto(FPDecimal::must_from_str("0.000000000000001")),
                min_quantity_tick_size: dec_to_proto(FPDecimal::must_from_str("1000000000000000")),
            },
            &signer,
        )
        .unwrap();

    let spot_markets = exchange
        .query_spot_markets(&QuerySpotMarketsRequest {
            status: "Active".to_string(),
            market_ids: vec![],
        })
        .unwrap()
        .markets;

    let market = spot_markets.iter().find(|m| m.ticker == ticker).unwrap();
    let spot_market_id = market.market_id.to_string();

    // Query
    let market_id = MarketId::new(spot_market_id.clone()).unwrap();
    let query_msg = QueryMsg::TestSpotMarketQuery { market_id };
    let res: SpotMarketResponse = wasm.query(&contract_address, &query_msg).unwrap();
    assert_eq!(res.market.clone().unwrap().market_id.as_str(), spot_market_id.clone());
}
