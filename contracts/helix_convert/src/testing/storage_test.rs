use cosmwasm_std::Addr;
use injective_cosmwasm::{inj_mock_deps, MarketId, TEST_MARKET_ID_1, TEST_MARKET_ID_2};
use crate::contract::set_route;
use crate::state::{read_swap_route, store_swap_route};
use crate::testing::test_utils::TEST_USER_ADDR;
use crate::types::SwapRoute;

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

    // Store the swap route
    store_swap_route(deps.as_mut().storage, &route).unwrap();

    // Read the stored swap route
    let stored_route = read_swap_route(&deps.storage, base_denom, quote_denom).unwrap();
    assert_eq!(stored_route, route);

    // Read with reversed denoms
    let stored_route_reversed = read_swap_route(&deps.storage, quote_denom, base_denom).unwrap();
    assert_eq!(stored_route_reversed, route);

    // Attempt to read a non-existent swap route
    let non_existent_route = read_swap_route(&deps.storage, "nonexistent", "route");
    assert!(non_existent_route.is_err());
}

#[test]
fn test_set_route() {
    let mut deps = inj_mock_deps(|_| {});
    let base_denom = "eth".to_string();
    let quote_denom = "inj".to_string();
    let route = vec![
        MarketId::unchecked(TEST_MARKET_ID_1),
        MarketId::unchecked(TEST_MARKET_ID_2),
    ];

    let result = set_route(deps.as_mut(), &Addr::unchecked(TEST_USER_ADDR), base_denom.clone(), quote_denom.clone(), route.clone());

    // Test that the function returned successfully
    assert!(result.is_ok());

    // Test that the Response has the correct attribute
    let response = result.unwrap();
    assert_eq!(response.attributes[0].key, "method");
    assert_eq!(response.attributes[0].value, "set_route");

    // Test that the correct route was stored
    let stored_route = read_swap_route(&deps.storage, &base_denom, &quote_denom).unwrap();
    assert_eq!(stored_route.steps, route);
    assert_eq!(stored_route.denom_1, base_denom);
    assert_eq!(stored_route.denom_2, quote_denom);
}
