use crate::{
    encode_helper::encode_proto_message,
    msg::{ExecuteMsg, QueryMsg, QueryStargateResponse},
    testing::type_helpers::{ExchangeParams, ParamResponse},
    utils::{add_spot_initial_liquidity, execute_all_authorizations, ExchangeType, Setup, BASE_DECIMALS, BASE_DENOM, QUOTE_DECIMALS},
};
use cosmwasm_std::{coin, from_json, Addr};
use injective_cosmwasm::{checked_address_to_subaccount_id, MarketId, SubaccountDepositResponse};
use injective_test_tube::{
    injective_std::types::{
        cosmos::base::v1beta1::Coin as BaseCoin,
        injective::exchange::v1beta1::{Deposit, MsgDeposit, QuerySubaccountDepositRequest, QuerySubaccountDepositsRequest},
    },
    Account, Exchange, Module, Wasm,
};
use injective_testing::utils::{human_to_dec, human_to_proto, scale_price_quantity_spot_market, str_coin};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_exchange_param() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/injective.exchange.v1beta1.Query/QueryExchangeParams".to_string(),
        query_request: "".to_string(),
    };

    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;

    let response: ParamResponse<ExchangeParams> = from_json(contract_response).unwrap();

    let listing_fee_coin = str_coin("20", BASE_DENOM, BASE_DECIMALS);

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
                    amount: Some(BaseCoin {
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

    let query_msg = QueryMsg::QueryStargateRaw {
        path: "/injective.exchange.v1beta1.Query/SubaccountDeposit".to_string(),
        query_request: encode_proto_message(QuerySubaccountDepositRequest {
            subaccount_id: subaccount_id.to_string(),
            denom: env.denoms["base"].clone(),
        }),
    };
    let contract_response: QueryStargateResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    let contract_response = contract_response.value;
    let contract_response: SubaccountDepositResponse = serde_json::from_str(&contract_response).unwrap();
    let deposit = contract_response.deposits;
    assert_eq!(deposit.total_balance, human_to_dec("10.0", BASE_DECIMALS));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_transient_spot_orders() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(env.users[0].account.address()), 0u32);

    execute_all_authorizations(&env.app, &env.users[0].account, env.contract_address.clone());

    add_spot_initial_liquidity(&env.app, market_id.clone());

    let (scale_price, scale_quantity) = scale_price_quantity_spot_market("9.8", "1", &BASE_DECIMALS, &QUOTE_DECIMALS);

    let res = wasm
        .execute(
            &env.contract_address,
            &ExecuteMsg::TestTraderTransientSpotOrders {
                market_id: MarketId::new(market_id).unwrap(),
                subaccount_id: subaccount_id.clone(),
                price: scale_price.to_string(),
                quantity: scale_quantity.to_string(),
            },
            &[],
            &env.users[0].account,
        )
        .unwrap();

    let transient_query = res
        .events
        .iter()
        .find(|e| e.ty == "wasm-transient_order")
        .and_then(|event| event.attributes.iter().find(|a| a.key == "query_str"));

    assert!(transient_query.is_some());
    let expected_order_info = "{\"value\":\"{\\\"orders\\\":[{\\\"price\\\":\\\"0.000000000009800000\\\",\\\"quantity\\\":\\\"1000000000000000000.000000000000000000\\\",\\\"fillable\\\":\\\"1000000000000000000.000000000000000000\\\",\\\"isBuy\\\":false,";
    assert!(transient_query.unwrap().value.contains(expected_order_info));
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_trader_spot_market_order() {
    let env = Setup::new(ExchangeType::Spot);
    let wasm = Wasm::new(&env.app);
    let market_id = env.market_id.unwrap();

    let subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(env.contract_address.to_owned()), 0u32);

    execute_all_authorizations(&env.app, &env.users[0].account, env.contract_address.clone());
    add_spot_initial_liquidity(&env.app, market_id.clone());

    let (scale_price, scale_quantity) = scale_price_quantity_spot_market("9.8", "1", &BASE_DECIMALS, &QUOTE_DECIMALS);

    wasm.execute(
        &env.contract_address,
        &ExecuteMsg::TestMarketOrderStargate {
            market_id: MarketId::new(market_id).unwrap(),
            subaccount_id: subaccount_id.clone(),
            price: scale_price.to_string(),
            quantity: scale_quantity.to_string(),
        },
        &[coin(1000000000000000000000u128, BASE_DENOM)],
        &env.users[0].account,
    )
    .unwrap();
}
