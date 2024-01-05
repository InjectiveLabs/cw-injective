use crate::{
    msg::{ExecuteMsg, QueryMsg},
    utils::test_setup,
};
use cosmwasm_std::{Addr, Coin};
use injective_cosmwasm::{checked_address_to_subaccount_id, MarketId};
use injective_math::{scale::Scaled, FPDecimal};
use injective_std::types::injective::exchange::v1beta1::{MsgInstantSpotMarketLaunch, QuerySpotMarketsRequest};
use injective_test_tube::{injective_cosmwasm::SpotMarketResponse, Exchange, Module, Wasm, Account};

pub const BASE_DENOM: &str = "inj";
pub const QUOTE_DENOM: &str = "usdt";

pub fn dec_to_proto(val: FPDecimal) -> String {
    val.scaled(18).to_string()
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_instantiation() {
    let (app, accs, contract_address) = test_setup();

    let wasm = Wasm::new(&app);
    let exchange = Exchange::new(&app);
    let buyer = &accs[1];
    let signer = &accs[2];

    // Execute contract
    let buyer_subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(buyer.address()), 1u32);
    let res = wasm.execute(
        &contract_address,
        &ExecuteMsg::TestDepositMsg {
            subaccount_id: buyer_subaccount_id,
            amount: Coin::new(100, "usdt"),
        },
        &[Coin::new(100, "usdt")],
        buyer,
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
