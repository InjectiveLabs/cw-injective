use cosmwasm_std::testing::{mock_dependencies, MockStorage};
use cosmwasm_std::{Api};
use helix_converter::state::{read_swap_route, store_swap_route, SwapRoute};
use injective_cosmwasm::MarketId;

// Replace `your_crate_name` with the name of your crate in the `use` statement above.

const  MARKET_1_ID: &str = "0x01edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16b";
const  MARKET_2_ID: &str = "0x02edfab47f124748dc89998eb33144af734484ba07099014594321729a0ca16d";

#[test]
fn store_and_read_swap_route() {
    let mut deps = mock_dependencies();
    let base_denom = "foo";
    let quote_denom = "bar";

    let route = SwapRoute {
        steps: vec![MarketId::unchecked(MARKET_1_ID), MarketId::unchecked(MARKET_2_ID)],
        denom_1: base_denom.to_string(),
        denom_2: quote_denom.to_string(),
    };

    // Store the swap route
    store_swap_route(deps.as_mut(), route.clone()).unwrap();

    // Read the stored swap route
    let stored_route = read_swap_route(deps.as_ref(), base_denom, quote_denom).unwrap();
    assert_eq!(stored_route, route);

    // Read with reversed denoms
    let stored_route_reversed = read_swap_route(deps.as_ref(), quote_denom, base_denom).unwrap();
    assert_eq!(stored_route_reversed, route);

    // Attempt to read a non-existent swap route
    let non_existent_route = read_swap_route(deps.as_ref(), "nonexistent", "route");
    assert!(non_existent_route.is_err());
}
