use crate::msg::QueryMsg;
use crate::utils::{str_coin, ExchangeType, Setup, BASE_DECIMALS, BASE_DENOM};
use cosmwasm_std::Uint128;
use injective_cosmwasm::tokenfactory::response::{TokenFactoryCreateDenomFeeResponse, TokenFactoryDenomSupplyResponse};
use injective_std::types::injective::tokenfactory::v1beta1::MsgCreateDenom;
use injective_test_tube::{Account, Module, TokenFactory, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_token_factory_denom_total_supply() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let factory = TokenFactory::new(&env.app);

    let test_denom = format!("factory/{}/test", env.owner.address());
    let msg_create_denom = MsgCreateDenom {
        sender: env.owner.address(),
        subdenom: "test".to_string(),
        name: "Test".to_string(),
        symbol: "TST".to_string(),
    };

    factory.create_denom(msg_create_denom, &env.owner).unwrap();
    let query_msg = QueryMsg::TestQueryTokenFactoryDenomTotalSupply { denom: test_denom };

    let response: TokenFactoryDenomSupplyResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(response.total_supply, Uint128::zero())
}

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_token_factory_creation_fee() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    let response: TokenFactoryCreateDenomFeeResponse = wasm.query(&env.contract_address, &QueryMsg::TestQueryTokenFactoryCreationFee {}).unwrap();
    assert_eq!(response.fee, vec![str_coin("10", BASE_DENOM, BASE_DECIMALS)])
}
