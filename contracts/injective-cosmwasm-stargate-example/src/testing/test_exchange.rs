use base64::{encode, Engine};
use base64::prelude::BASE64_STANDARD;
use crate::{
    msg::{QueryStargateResponse, QueryMsg},
    utils::{
        add_spot_initial_liquidity, add_spot_order_as, add_spot_orders, dec_to_proto, execute_all_authorizations,
        get_initial_liquidity_orders_vector, get_spot_market_id, human_to_dec, human_to_proto, scale_price_quantity_for_spot_market,
        scale_price_quantity_for_spot_market_dec, str_coin, ExchangeType, HumanOrder, Setup, BASE_DECIMALS, BASE_DENOM, QUOTE_DECIMALS, QUOTE_DENOM,
    },
    testing::type_helpers::{ParamResponse, ExchangeParams},
};

use injective_std::types::injective::exchange::v1beta1::{Deposit, MsgDeposit, MsgInstantSpotMarketLaunch, OrderType, QueryAggregateMarketVolumeResponse, QuerySubaccountDepositRequest, QuerySubaccountDepositResponse, QuerySubaccountDepositsRequest};

use cosmwasm_std::{Addr, from_json, to_json_string};
use injective_test_tube::{injective_cosmwasm::get_default_subaccount_id_for_checked_address, Account, Exchange, Module, RunnerResult, Wasm};
use injective_cosmwasm::{checked_address_to_subaccount_id, SubaccountDepositResponse, SubaccountId};
use prost::Message;
use serde_json::to_vec;
use injective_std::types::injective::oracle::v1beta1::GrantPriceFeederPrivilegeProposal;
use crate::order_management::encode_bytes_message;
use crate::testing::type_helpers::AuthParams;
use crate::utils::encode_proto_message;

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_exchange_param() {
    let env = Setup::new(ExchangeType::None);

    let wasm = Wasm::new(&env.app);
    let user = &env.users[0];

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(user.account.address()), 1u32);
    // Execute contract

    let query_msg = QueryMsg::QueryStargate {
        path: "/injective.exchange.v1beta1.Query/QueryExchangeParams".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response =  contract_response.value;
    println!("{:?}", contract_response);
    let response: ParamResponse<ExchangeParams> = from_json(&contract_response).unwrap();
    println!("{:?}", response);
    let listing_fee_coin = str_coin("1000", BASE_DENOM, BASE_DECIMALS);
    assert_eq!(response.params.spot_market_instant_listing_fee, listing_fee_coin);
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_subaccount_deposit() {
    let env = Setup::new(ExchangeType::None);
    let exchange = Exchange::new(&env.app);
    let wasm = Wasm::new(&env.app);

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(env.users[0].account.address()), 1u32);

    let make_deposit = |amount: &str, denom_key: &str| {
        exchange
            .deposit(
                MsgDeposit {
                    sender: env.users[0].account.address(),
                    subaccount_id: subaccount_id.to_string(),
                    amount: Some(injective_std::types::cosmos::base::v1beta1::Coin {
                        amount: amount.to_string(),
                        denom: env.denoms[denom_key].clone(),
                    }),
                },
                &env.users[0].account,
            )
            .unwrap();
    };

    make_deposit("10000000000000000000", "base");
    make_deposit("100000000", "quote");

    let response = exchange
        .query_subaccount_deposits(&QuerySubaccountDepositsRequest {
            subaccount_id: subaccount_id.to_string(),
            subaccount: None,
        })
        .unwrap();

    assert_eq!(
        response.deposits[&env.denoms["base"].clone()],
        Deposit {
            available_balance: human_to_proto("10.0", BASE_DECIMALS),
            total_balance: human_to_proto("10.0", BASE_DECIMALS),
        }
    );
    assert_eq!(
        response.deposits[&env.denoms["quote"].clone()],
        Deposit {
            available_balance: human_to_proto("100.0", QUOTE_DECIMALS),
            total_balance: human_to_proto("100.0", QUOTE_DECIMALS),
        }
    );


    let query_msg = QueryMsg::QueryStargate {
        path: "/injective.exchange.v1beta1.Query/SubaccountDeposit".to_string(),
        query_request: encode_proto_message(QuerySubaccountDepositRequest {
            subaccount_id: subaccount_id.to_string(),
            denom: env.denoms["base"].clone(),
        }),
    };
    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response =  contract_response.value;
    let contract_response: SubaccountDepositResponse = serde_json::from_str(&contract_response).unwrap();
    println!("{:?}", contract_response);
    let deposit = contract_response.deposits;
    assert_eq!(deposit.total_balance, human_to_dec("10.0", BASE_DECIMALS));
}

