use std::marker::PhantomData;
use std::str::FromStr;

use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, coins, ContractInfo, ContractResult, CustomQuery, Deps,
    DepsMut, Env, from_binary, MemoryStorage, OwnedDeps, Querier, QuerierResult, QuerierWrapper,
    Reply, Storage, SubMsgResponse, SubMsgResult, SystemResult, Timestamp, to_binary,
    TransactionInfo, Uint128,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockStorage};

use injective_cosmwasm::{
    Deposit, InjectiveMsg, InjectiveQueryWrapper, InjectiveRoute, OrderInfo, SpotMarket,
    SpotMarketResponse, SpotOrder, WasmMockQuerier,
};
use injective_cosmwasm::InjectiveMsg::CreateSpotMarketOrder;
use injective_math::FPDecimal;

use crate::contract::{ATOMIC_ORDER_REPLY_ID, execute, instantiate, reply};
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


pub trait OwnedDepsExt<S, A, Q, C> where C: CustomQuery {
    fn as_mut_deps(&mut self) -> DepsMut<C>;
}

impl<S, A, Q, C> OwnedDepsExt<S, A, Q, C> for OwnedDeps<S, A, Q, C>
    where
        S: Storage,
        A: Api,
        Q: Querier,
        C: CustomQuery,
{
    fn as_mut_deps(&mut self) -> DepsMut<C> {
        return DepsMut {
            storage: &mut self.storage,
            api: &self.api,
            querier: QuerierWrapper::new(&self.querier),
        };
    }
}

pub fn inj_mock_deps() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InjectiveQueryWrapper> {
    let mut custom_querier: WasmMockQuerier = WasmMockQuerier::new();
    custom_querier.spot_market_response_handler = Some(handle_spot_market_response);
    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
        custom_query_type: PhantomData::default(),
    }
}

#[test]
fn proper_initialization() {
    let msg = InstantiateMsg {
        market_id: "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6".to_string(),
    };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(inj_mock_deps().as_mut_deps(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_swap() {
    let contract_addr = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";
    let sender_addr = "inj1x2ck0ql2ngyxqtw8jteyc0tchwnwxv7npaungt";
    let market_id = "0x78c2d3af98c517b164070a739681d4bd4d293101e7ffc3a30968945329b47ec6";

    let msg = InstantiateMsg {
        market_id: market_id.to_string(),
        // base_denom: "inj".to_string(),
        // quote_denom: "usdc".to_string(),
    };
    let info = mock_info(contract_addr, &coins(1000, "earth"));
    let mut deps = inj_mock_deps();
    let _ = instantiate(deps.as_mut_deps(), mock_env(), info, msg);

    let env = inj_mock_env();

    let info = mock_info(sender_addr, &coins(9000, "usdt"));
    let msg = ExecuteMsg::SwapSpot {
        quantity: i32_to_dec(8),
        price: i32_to_dec(1000),
    };
    let res = execute(deps.as_mut_deps(), env.clone(), info, msg).unwrap();

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
            subaccount_id: _subaccount_id,
            amount: _amount,
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

    let transfers_response = reply(deps.as_mut_deps(), inj_mock_env(), reply_msg);
    let transfers = transfers_response.unwrap().messages;
    assert_eq!(transfers.len(), 2);
    let msg1 = &get_message_data(&transfers, 0).msg_data;
    match msg1 {
        // base
        InjectiveMsg::ExternalTransfer {
            sender,
            source_subaccount_id: _source_subaccount_id,
            destination_subaccount_id: _destination_subaccount_id,
            amount,
        } => {
            assert_eq!(sender, contract_addr, "sender not correct");
            assert_eq!(amount.amount, Uint128::from(8u128));
        }
        _ => panic!("Wrong message type!"),
    }
    match &get_message_data(&transfers, 1).msg_data {
        // leftover quote
        InjectiveMsg::ExternalTransfer {
            sender,
            source_subaccount_id: _source_subaccount_id,
            destination_subaccount_id: _destination_subaccount_id,
            amount,
        } => {
            assert_eq!(sender, contract_addr, "sender not correct");
            assert_eq!(amount.amount, Uint128::from((9000u128 - 8036u128)));
        }
        _ => panic!("Wrong message type!"),
    }
}

fn handle_spot_market_response(market_id: String) -> QuerierResult {
    let response = SpotMarketResponse {
        market: Some(SpotMarket {
            ticker: "INJ/USDT".to_string(),
            base_denom: "INJ".to_string(),
            quote_denom: "USDT".to_string(),
            maker_fee_rate: FPDecimal::from_str("0.01").unwrap(),
            taker_fee_rate: FPDecimal::from_str("0.1").unwrap(),
            relayer_fee_share_rate: FPDecimal::from_str("0.4").unwrap(),
            market_id,
            status: 0,
            min_price_tick_size: FPDecimal::from_str("0.000000000000001").unwrap(),
            min_quantity_tick_size: FPDecimal::from_str("1000000000000000").unwrap(),
        }),
    };
    SystemResult::Ok(ContractResult::from(to_binary(&response)))
}
