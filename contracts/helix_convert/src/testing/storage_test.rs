use cosmwasm_std::Addr;

use injective_cosmwasm::{
    inj_mock_deps, MarketId, OwnedDepsExt, TEST_MARKET_ID_1, TEST_MARKET_ID_2, TEST_MARKET_ID_3,
};

use crate::contract::{delete_route, set_route};
use crate::state::{read_swap_route, store_swap_route, CONFIG};
use crate::testing::test_utils::{TEST_CONTRACT_ADDR, TEST_USER_ADDR};
use crate::types::{Config, SwapRoute};

#[test]
fn store_and_read_swap_route() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "foo";
    let quote_denom = "bar";

    let route = SwapRoute {
        steps: vec![
            MarketId::unchecked(TEST_MARKET_ID_1),
            MarketId::unchecked(TEST_MARKET_ID_2),
        ],
        denom_1: base_denom.to_string(),
        denom_2: quote_denom.to_string(),
    };

    store_swap_route(deps.as_mut().storage, &route).unwrap();

    let stored_route = read_swap_route(&deps.storage, base_denom, quote_denom).unwrap();
    assert_eq!(stored_route, route, "stored route was not read correctly");

    // Read with reversed denoms
    let stored_route_reversed = read_swap_route(&deps.storage, quote_denom, base_denom).unwrap();
    assert_eq!(stored_route_reversed, route);

    let non_existent_route = read_swap_route(&deps.storage, "nonexistent", "route");
    assert!(non_existent_route.is_err(), "non-existent route was read");
}

#[test]
fn update_and_read_swap_route() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "foo";
    let quote_denom = "bar";

    let route = SwapRoute {
        steps: vec![
            MarketId::unchecked(TEST_MARKET_ID_1),
            MarketId::unchecked(TEST_MARKET_ID_2),
        ],
        denom_1: base_denom.to_string(),
        denom_2: quote_denom.to_string(),
    };

    store_swap_route(deps.as_mut().storage, &route).unwrap();

    let mut stored_route = read_swap_route(&deps.storage, base_denom, quote_denom).unwrap();
    assert_eq!(stored_route, route, "stored route was not read correctly");

    let updated_route = SwapRoute {
        steps: vec![
            MarketId::unchecked(TEST_MARKET_ID_1),
            MarketId::unchecked(TEST_MARKET_ID_3),
        ],
        denom_1: base_denom.to_string(),
        denom_2: quote_denom.to_string(),
    };

    store_swap_route(deps.as_mut().storage, &updated_route).unwrap();

    stored_route = read_swap_route(&deps.storage, base_denom, quote_denom).unwrap();
    assert_eq!(stored_route, updated_route, "stored route was not updated");
}

#[test]
fn test_set_route_as_owner() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "inj".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route.clone(),
    );

    assert!(result.is_ok(), "result was not ok");

    let response = result.unwrap();
    assert_eq!(response.attributes[0].key, "method", "method attribute was not set");
    assert_eq!(response.attributes[0].value, "set_route", "method attribute was not set");

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom).unwrap();
    assert_eq!(stored_route.steps, route, "route was not set correctly");
    assert_eq!(stored_route.denom_1, base_denom, "route was not set correctly");
    assert_eq!(stored_route.denom_2, quote_denom, "route was not set correctly");
}

#[test]
fn test_set_route_single_step_route() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "usd".to_string();
    let route = vec![MarketId::unchecked(TEST_MARKET_ID_1)];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route.clone(),
    );

    assert!(result.is_ok(), "result was not ok");

    let response = result.unwrap();
    assert_eq!(
        response.attributes[0].key, "method",
        "method attribute was not set"
    );
    assert_eq!(
        response.attributes[0].value, "set_route",
        "method attribute was not set"
    );

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom).unwrap();
    assert_eq!(stored_route.steps, route, "route was not stored correctly");
    assert_eq!(
        stored_route.denom_1, base_denom,
        "denom_1 was not stored correctly"
    );
    assert_eq!(
        stored_route.denom_2, quote_denom,
        "denom_2 was not stored correctly"
    );
}

#[test]
fn test_set_route_same_denom() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "eth".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };

    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route.clone(),
    );

    assert!(
        result.is_err(),
        "Could set a route with the same denom being source and target!"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Cannot set a route with the same denom being source and target"),
        "wrong error message"
    );

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom);
    assert!(
        stored_route.is_err(),
        "Could read a route with the same denom being source and target!"
    );
}

#[test]
fn test_set_route_no_market_id() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "atom".to_string();
    let route = vec![];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };

    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route.clone(),
    );

    assert!(result.is_err(), "Could set a route without any steps");
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Route must have at least 1 step"),
        "wrong error message"
    );

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom);
    assert!(
        stored_route.is_err(),
        "Could read a route without any steps"
    );
}

#[test]
fn test_set_route_same_market_id() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "ATOM".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_1),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };

    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route.clone(),
    );

    assert!(
        result.is_err(),
        "Could set a route that begins and ends with the same market"
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Route cannot begin and end with the same market"),
        "wrong error message"
    );

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom);
    assert!(
        stored_route.is_err(),
        "Could read a route that begins and ends with the same market"
    );
}

#[test]
fn test_set_route_as_unauthorised() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "inj".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_CONTRACT_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route,
    );

    assert!(result.is_err(), "expected error");
    assert!(
        result.unwrap_err().to_string().contains("Unauthorized"),
        "wrong error message"
    );

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom).unwrap_err();
    assert!(
        stored_route
            .to_string()
            .contains("No swap route not found from eth to inj"),
        "wrong error message"
    );
}

#[test]
fn test_delete_existing_route_as_admin() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "inj".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let set_result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route,
    );

    assert!(set_result.is_ok(), "expected success on set");

    let delete_result = delete_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
    );

    assert!(delete_result.is_ok(), "expected success on delete");

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom).unwrap_err();
    assert!(
        stored_route
            .to_string()
            .contains("No swap route not found from eth to inj"),
        "route was not deleted and could be read"
    );
}

#[test]
fn test_delete_non_existent_route_as_admin() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "inj".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let set_result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route,
    );

    assert!(set_result.is_ok(), "expected success on set");

    let delete_result = delete_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        "mietek".to_string(),
    );

    assert!(set_result.is_ok(), "expected success on delete");

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom);
    assert!(stored_route.is_ok(), "route was deleted");
}

#[test]
fn test_delete_route_as_unauthorised() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "inj".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let config = Config {
        fee_recipient: Addr::unchecked(TEST_USER_ADDR),
        admin: Addr::unchecked(TEST_USER_ADDR),
    };
    CONFIG
        .save(deps.as_mut_deps().storage, &config)
        .expect("could not save config");

    let set_result = set_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_USER_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
        route,
    );

    assert!(set_result.is_ok(), "expected success on set");

    let delete_result = delete_route(
        deps.as_mut(),
        &Addr::unchecked(TEST_CONTRACT_ADDR),
        base_denom.clone(),
        quote_denom.clone(),
    );

    assert!(delete_result.is_err(), "expected error on delete");

    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom);
    assert!(stored_route.is_ok(), "route was deleted");
}
