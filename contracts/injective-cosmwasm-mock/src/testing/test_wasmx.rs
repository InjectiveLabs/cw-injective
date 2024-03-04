use crate::msg::QueryMsg;
use crate::utils::{ExchangeType, Setup};
use injective_cosmwasm::wasmx::response::QueryContractRegistrationInfoResponse;
use injective_test_tube::{Module, Wasm};

#[test]
#[cfg_attr(not(feature = "integration"), ignore)]
fn test_query_contract_registration_info() {
    let env = Setup::new(ExchangeType::None);
    let wasm = Wasm::new(&env.app);

    let query_msg = QueryMsg::TestQueryContractRegistrationInfo {
        contract_address: env.contract_address.to_owned(),
    };

    let response: QueryContractRegistrationInfoResponse = wasm.query(&env.contract_address, &query_msg).unwrap();
    assert!(response.contract.is_none())
}
