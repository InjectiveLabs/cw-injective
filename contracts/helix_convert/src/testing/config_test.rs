use cosmwasm_std::{Addr, coins};
use cosmwasm_std::testing::{mock_env, mock_info};

use injective_cosmwasm::{
    inj_mock_deps, MarketId, OwnedDepsExt, TEST_MARKET_ID_1, TEST_MARKET_ID_2, TEST_MARKET_ID_3,
};

use crate::contract::{delete_route, execute, set_route};
use crate::msg::{ExecuteMsg, FeeRecipient};
use crate::state::{read_swap_route, store_swap_route, CONFIG};
use crate::testing::test_utils::{TEST_CONTRACT_ADDR, TEST_USER_ADDR};
use crate::types::{Config, SwapRoute};

#[test]
pub fn admin_can_update_config() {
    let mut deps = inj_mock_deps(|_| {});

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_CONTRACT_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let new_admin = Addr::unchecked("new_admin");
    let new_fee_recipient = Addr::unchecked("new_fee_recipient");

    let info = mock_info(TEST_USER_ADDR, &coins(12, "eth"));

    let msg = ExecuteMsg::UpdateConfig {
        admin: Some(new_admin.clone()),
        fee_recipient: Some(FeeRecipient::Address(new_fee_recipient.clone())),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len(), "no messages expected");

    let config = CONFIG.load(deps.as_mut_deps().storage).unwrap();
    assert_eq!(config.admin, new_admin, "admin was not updated");
    assert_eq!(
        config.fee_recipient,
        new_fee_recipient,
        "fee_recipient was not updated"
    );
}

#[test]
pub fn non_admin_cannot_update_config() {
    let mut deps = inj_mock_deps(|_| {});

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_CONTRACT_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let new_admin = Addr::unchecked("new_admin");
    let new_fee_recipient = Addr::unchecked("new_fee_recipient");

    let info = mock_info("non_admin", &coins(12, "eth"));

    let msg = ExecuteMsg::UpdateConfig {
        admin: Some(new_admin.clone()),
        fee_recipient: Some(FeeRecipient::Address(new_fee_recipient.clone())),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_err(), "expected error on non-admin update config");
}
