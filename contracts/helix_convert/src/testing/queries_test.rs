use std::str::FromStr;

use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coin, Addr};

use injective_cosmwasm::{OwnedDepsExt, TEST_MARKET_ID_1, TEST_MARKET_ID_2};
use injective_math::FPDecimal;

use crate::contract::{instantiate, set_route};
use crate::msg::{FeeRecipient, InstantiateMsg};
use crate::queries::estimate_swap_result;
use crate::testing::test_utils::{mock_deps_eth_inj, TEST_USER_ADDR};

/// In this test we swap 1000 INJ to ETH, we assume avg price of INJ at 8 usdt and avg price of eth 2000 usdt
#[test]
fn test_calculate_swap_price() {
    let mut deps = mock_deps_eth_inj();
    let admin = &Addr::unchecked(TEST_USER_ADDR);

    instantiate(
        deps.as_mut_deps(),
        mock_env(),
        mock_info(admin.as_ref(), &[coin(1_000u128, "usdt")]),
        InstantiateMsg {
            fee_recipient: FeeRecipient::SwapContract,
            admin: admin.to_owned(),
        },
    )
    .unwrap();
    set_route(
        deps.as_mut_deps(),
        &Addr::unchecked(TEST_USER_ADDR),
        "eth".to_string(),
        "inj".to_string(),
        vec![TEST_MARKET_ID_1.into(), TEST_MARKET_ID_2.into()],
    )
    .unwrap();

    let amount_inj = estimate_swap_result(
        deps.as_ref(),
        mock_env(),
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
