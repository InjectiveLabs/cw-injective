use std::collections::HashMap;
use std::str::FromStr;
use cosmwasm_std::StdError;

use injective_cosmwasm::{create_mock_spot_market, create_orderbook_response_handler, create_spot_market_handler, create_spot_multi_market_handler, inj_mock_deps, Hash, OwnedDepsExt, SpotMarket, TEST_MARKET_ID_1, TEST_MARKET_ID_2, PriceLevel};
use injective_math::FPDecimal;
use crate::contract::set_route;
use crate::queries::{estimate_swap_result, find_minimum_orders};

use crate::testing::test_utils::mock_deps_eth_inj;

/// In this test we swap 1000 INJ to ETH, we assume avg price of INJ at 8 usdt and avg price of eth 2000 usdt
#[test]
fn test_calculate_swap_price() {
    // let mut deps_binding = mock_deps_eth_inj();

    let mut deps = mock_deps_eth_inj();

    set_route(deps.as_mut_deps(),  "eth".to_string(), "inj".to_string(), vec![TEST_MARKET_ID_1.into(), TEST_MARKET_ID_2.into()]).unwrap();

    let amount_inj = estimate_swap_result(
        deps.as_ref(),
        "eth".to_string(),
        FPDecimal::from_str("12").unwrap(),
        "inj".to_string(),
    )
    .unwrap();
    assert_eq!(
        amount_inj,
        FPDecimal::from_str("2879.74").unwrap(),
        "Wrong amount of INJ received"
    ); // value rounded to min tick
    println!("Got {amount_inj} inj");
}
