

use std::str::FromStr;

use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, Addr, BankMsg, Binary, CosmosMsg, Reply, SubMsgResponse, SubMsgResult, Uint128, Coin};
use cosmwasm_std::StdError::GenericErr;
use injective_cosmwasm::InjectiveMsg::CreateSpotMarketOrder;
use injective_cosmwasm::{get_default_subaccount_id_for_checked_address, inj_mock_env, InjectiveRoute, MarketId, OrderInfo, OrderType, OwnedDepsExt, SpotOrder, TEST_MARKET_ID_1, TEST_MARKET_ID_2};
use injective_math::FPDecimal;
use injective_protobuf::proto::tx::{MsgCreateSpotMarketOrderResponse, SpotMarketOrderResults};

use crate::contract::{reply, ATOMIC_ORDER_REPLY_ID, set_route, start_swap_flow};
use crate::helpers::{get_message_data, i32_to_dec};


use crate::testing::test_utils::{mock_deps_eth_inj, MultiplierQueryBehaviour, TEST_CONTRACT_ADDR, TEST_USER_ADDR};
use protobuf::Message;
use crate::queries::{estimate_single_swap_execution, estimate_swap_result};
use crate::state::CONFIG;
use crate::types::{Config, FPCoin};


#[test]
fn test_swap_2_markets() {
    let deps_binding = mock_deps_eth_inj(MultiplierQueryBehaviour::Success);
    let mut deps = deps_binding;

    let config = Config{ fee_recipient: Addr::unchecked(TEST_CONTRACT_ADDR), admin: Addr::unchecked(TEST_USER_ADDR) };
    CONFIG.save(deps.as_mut_deps().storage, &config).expect("could not save config");

    set_route(deps.as_mut_deps(), &Addr::unchecked(TEST_USER_ADDR),"eth".to_string(), "inj".to_string(), vec![TEST_MARKET_ID_1.into(), TEST_MARKET_ID_2.into()]).unwrap();

    let info = mock_info(TEST_USER_ADDR, &coins(12, "eth"));

    let response_1 = start_swap_flow(deps.as_mut_deps(), inj_mock_env(), info, "inj".to_string(), FPDecimal::from(2879u128)).unwrap();
    let subaccount_id = get_default_subaccount_id_for_checked_address(&Addr::unchecked(TEST_CONTRACT_ADDR));
    let expected_atomic_sell_message = CreateSpotMarketOrder {
            sender: inj_mock_env().contract.address,
            order: SpotOrder {
                market_id: TEST_MARKET_ID_1.into(),
                order_info: OrderInfo {
                    subaccount_id: subaccount_id.clone(),
                    fee_recipient: Some(Addr::unchecked(TEST_CONTRACT_ADDR)),
                    price: i32_to_dec(192000),
                    quantity: i32_to_dec(12),
                },
                order_type: OrderType::SellAtomic,
                trigger_price: None,
            },
        };
    let order_message = get_message_data(&response_1.messages, 0);
    assert_eq!(
        InjectiveRoute::Exchange,
        order_message.route,
        "route was incorrect"
    );
    assert_eq!(
        expected_atomic_sell_message, order_message.msg_data,
        "spot create order had incorrect content"
    );

    let mut order_results = SpotMarketOrderResults::default();
    order_results.set_price("196750000000000000000000".to_string());
    order_results.set_quantity("2351556000000000000000000".to_string());
    order_results.set_fee("9444000000000000000000".to_string());

    let mut atomic_response = MsgCreateSpotMarketOrderResponse::default();
    atomic_response.set_order_hash("ORDER_HASH1".to_string());
    atomic_response.set_results(order_results);

    let encoded: Binary = atomic_response.write_to_bytes().unwrap().into();
    println!("Encoded: {encoded}");

    let reply_msg = Reply {
        id: ATOMIC_ORDER_REPLY_ID,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(encoded),
        }),
    };

    // simulate and handle results of eth->usdt exchange
    let response_2 = reply(deps.as_mut_deps(), inj_mock_env(), reply_msg ).unwrap();
    let expected_atomic_buy_message = CreateSpotMarketOrder {
        sender: inj_mock_env().contract.address,
        order: SpotOrder {
            market_id: TEST_MARKET_ID_2.into(),
            order_info: OrderInfo {
                subaccount_id,
                fee_recipient: Some(Addr::unchecked(TEST_CONTRACT_ADDR)),
                price: i32_to_dec(830),
                quantity: FPDecimal::from_str("2879.74").unwrap(),
            },
            order_type: OrderType::BuyAtomic,
            trigger_price: None,
        },
    };
    let order_message = get_message_data(&response_2.messages, 0);
    assert_eq!(
        InjectiveRoute::Exchange,
        order_message.route,
        "route was incorrect"
    );
    assert_eq!(
        expected_atomic_buy_message, order_message.msg_data,
        "spot create order had incorrect content"
    );


    // simulate usdt->inj exchange
    let mut order_results = SpotMarketOrderResults::default();
    order_results.set_quantity("2879740000000000000000".to_string());

    let mut atomic_response = MsgCreateSpotMarketOrderResponse::default();
    atomic_response.set_order_hash("ORDER_HASH2".to_string());
    atomic_response.set_results(order_results);

    let encoded: Binary = atomic_response.write_to_bytes().unwrap().into();
    let reply_msg = Reply {
        id: ATOMIC_ORDER_REPLY_ID,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(encoded),
        }),
    };
    let response_3 = reply(deps.as_mut_deps(), inj_mock_env(), reply_msg ).unwrap();
    if let CosmosMsg::Bank(BankMsg::Send { to_address, amount }) = &response_3.messages[0].msg {
        assert_eq!(to_address, TEST_USER_ADDR);
        assert_eq!(1, amount.len());
        assert_eq!(amount[0].denom, "inj");
        assert_eq!(amount[0].amount, Uint128::from(2879u128));
    } else {
        panic!("Wrong message type!");
    }
}

#[test]
fn failing_multiplier_query() {
    let env = mock_env();
    let deps_binding = mock_deps_eth_inj(MultiplierQueryBehaviour::Fail);
    let mut deps = deps_binding;

    let config = Config{ fee_recipient: Addr::unchecked(TEST_USER_ADDR), admin: Addr::unchecked(TEST_USER_ADDR) };
    CONFIG.save(deps.as_mut_deps().storage, &config).expect("could not save config");

    set_route(deps.as_mut_deps(), &Addr::unchecked(TEST_USER_ADDR),"eth".to_string(), "inj".to_string(), vec![TEST_MARKET_ID_1.into(), TEST_MARKET_ID_2.into()]).unwrap();

    let response_1 = estimate_single_swap_execution(
    &deps.as_mut_deps().as_ref(),
        &env,
        &MarketId::unchecked(TEST_MARKET_ID_1.to_string()),
    FPCoin::from(Coin::new(1000000000000000000u128, "eth".to_string())));

    assert!(response_1.is_err(), "should have failed");
    assert!(response_1.unwrap_err().to_string().contains("Querier system error: Unknown system error"), "wrong error message");
}

