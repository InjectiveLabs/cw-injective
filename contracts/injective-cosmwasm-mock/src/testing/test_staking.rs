use crate::msg::QueryMsg;
use crate::utils::{ExchangeType, Setup, BASE_DENOM};
use cosmwasm_std::Uint128;
use injective_cosmwasm::exchange::response::StakedAmountResponse;
use injective_std::types::cosmos::base::v1beta1::Coin;
use injective_std::types::cosmos::staking::v1beta1::MsgDelegate;
use injective_test_tube::{Account, Module, Staking, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_staked_amount() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);
    let staking = Staking::new(&env.app);
    let validator_address = env.app.get_first_validator_address().unwrap();

    staking
        .delegate(
            MsgDelegate {
                delegator_address: env.owner.address(),
                validator_address,
                amount: Some(Coin {
                    amount: "10".to_string(),
                    denom: BASE_DENOM.to_string(),
                }),
            },
            &env.owner,
        )
        .unwrap();

    let query_msg = QueryMsg::TestQueryStakedAmount {
        delegator_address: env.owner.address(),
        max_delegations: 100,
    };

    let response: StakedAmountResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert_eq!(response.staked_amount, Uint128::new(10));
}
