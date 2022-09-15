use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Addr, Binary, BlockInfo, ContractInfo, Env, Reply, SubMsgResponse, SubMsgResult, Timestamp, TransactionInfo, Uint128};

use injective_cosmwasm::InjectiveMsg::CreateSpotMarketOrder;
use injective_cosmwasm::{Deposit, InjectiveMsg, InjectiveRoute, OrderInfo, SpotOrder};

use crate::contract::{execute, instantiate, reply, ATOMIC_ORDER_REPLY_ID};
use crate::helpers::{get_message_data, i32_to_dec};
use crate::msg::{ExecuteMsg, InstantiateMsg};

use super::*;

pub const TEST_CONTRACT_ADDR: &str = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";

pub fn inj_mock_env() -> Env {
    // let mut mock_env: Env = mock_env();
    // mock_env.contract.address = Addr::unchecked(TEST_CONTRACT_ADDR);
    // return mock_env;
    Env {
        block: BlockInfo {
            height: 12_345,
            time: Timestamp::from_nanos(1_571_797_419_879_305_533),
            chain_id: "cosmos-testnet-14002".to_string(),
        },
        transaction: Some(TransactionInfo { index: 3 }),
        contract: ContractInfo {
            address: Addr::unchecked(TEST_CONTRACT_ADDR),
        },
    }
}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        market_id: "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6".to_string(),
        base_denom: "inj".to_string(),
        quote_denom: "usdc".to_string(),
    };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // reply(deps, )
    // // it worked, let's query the state
    // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    // let value: GetCountResponse = from_binary(&res).unwrap();
    // assert_eq!(17, value.count);
}

#[test]
fn test_swap() {
    let mut deps = mock_dependencies();
    let contract_addr = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";
    let sender_addr = "inj1x2ck0ql2ngyxqtw8jteyc0tchwnwxv7npaungt";
    let market_id = "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6";

    let msg = InstantiateMsg {
        market_id: market_id.to_string(),
        base_denom: "inj".to_string(),
        quote_denom: "usdc".to_string(),
    };
    let info = mock_info(contract_addr, &coins(1000, "earth"));

    let _ = instantiate(deps.as_mut(), mock_env(), info, msg);

    let env = inj_mock_env();

    let info = mock_info(sender_addr, &coins(9000, "usdt"));
    let msg = ExecuteMsg::SwapSpot {
        quantity: i32_to_dec(8),
        price: i32_to_dec(1000),
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let expected_atomic_order_message = CreateSpotMarketOrder {
        sender: env.contract.address.into_string(),
        order: SpotOrder {
            market_id: String::from(market_id),
            order_info: OrderInfo {
                subaccount_id: "0xade4a5f5803a439835c636395a8d648dee57b2fc000000000000000000000000"
                    .to_string(),
                fee_recipient: contract_addr.to_string(),
                price: i32_to_dec(1000),
                quantity: i32_to_dec(8),
            },
            order_type: 9,
            trigger_price: None,
        },
    };

    match &get_message_data(&res.messages, 0).msg_data {
        InjectiveMsg::Deposit {
            sender,
            subaccount_id,
            amount,
        } => {
            assert_eq!(sender, contract_addr, "sender not correct")
        }
        _ => {}
    }
    let order_message = get_message_data(&res.messages, 1);
    assert_eq!(
        InjectiveRoute::Exchange,
        order_message.route,
        "route was incorrect"
    );
    assert_eq!(
        expected_atomic_order_message, order_message.msg_data,
        "spot create order had incorrect content"
    );

    let binary_response = Binary::from_base64("CkIweGRkNzI5MmY2ODcwMzIwOTc2YTUxYTUwODBiMGQ2NDU5M2NhZjE3OWViM2YxOTNjZWVlZGFiNGVhNWUxNDljZWISQwoTODAwMDAwMDAwMDAwMDAwMDAwMBIWMTAwMDAwMDAwMDAwMDAwMDAwMDAwMBoUMzYwMDAwMDAwMDAwMDAwMDAwMDA=").unwrap();
    let reply_msg = Reply {
        id: ATOMIC_ORDER_REPLY_ID,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(binary_response),
        }),
    };

    let transfers_response = reply(deps.as_mut(), inj_mock_env(), reply_msg);
    let transfers = transfers_response.unwrap().messages;
    assert_eq!(transfers.len(), 2);
    let msg1 = &get_message_data(&transfers, 0).msg_data;
    match msg1 { // base
        InjectiveMsg::ExternalTransfer {
            sender, source_subaccount_id, destination_subaccount_id, amount
        } => {
            assert_eq!(sender, contract_addr, "sender not correct");
            assert_eq!(amount.amount, Uint128::from(8u128));
        }
        _ => panic!("Wrong message type!")
    }
    match &get_message_data(&transfers, 1).msg_data { // leftover quote
        InjectiveMsg::ExternalTransfer {
            sender, source_subaccount_id, destination_subaccount_id, amount
        } => {
            assert_eq!(sender, contract_addr, "sender not correct");
            assert_eq!(amount.amount, Uint128::from((9000u128-8036u128)));
        }
        _ => panic!("Wrong message type!")
    }
}

