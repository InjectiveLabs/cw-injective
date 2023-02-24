use cosmwasm_std::{DepsMut, Env, Response};
use schemars::_serde_json::to_string;

use injective_cosmwasm::{
    create_relay_pyth_prices_msg, Hash, InjectiveMsgWrapper, InjectiveQueryWrapper,
    PriceAttestation, PythStatus,
};

use crate::ContractError;

pub fn execute_trigger_pyth_update(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    price: i64,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    deps.api.debug("Starting trigger update");
    let mut response = Response::new();
    let pa = PriceAttestation {
        product_id: "MOCK_PRODUCT_ID".to_string(),
        price_id: Hash::from_hex(
            "f9c0172ba10dfa4d19088d94f5bf61d3b54d5bd7483a322a982e1373ee8ea31b",
        )?,
        price,
        conf: 500,
        expo: -3,
        ema_price: 1000,
        ema_conf: 2000,
        status: PythStatus::Trading,
        num_publishers: 10,
        max_num_publishers: 20,
        attestation_time: (env.block.time.nanos() - 100) as i64,
        publish_time: env.block.time.nanos() as i64,
    };
    deps.api.debug(&format!("Msg: {}", to_string(&pa).unwrap()));
    let relay_msg = create_relay_pyth_prices_msg(env.contract.address, vec![pa]);
    response = response.add_message(relay_msg);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
    use cosmwasm_std::{Api, CustomQuery, DepsMut, OwnedDeps, Querier, QuerierWrapper, Storage};

    use injective_cosmwasm::{InjectiveQueryWrapper, WasmMockQuerier};

    use crate::contract::execute;
    use crate::msg::ExecuteMsg;

    #[test]
    pub fn test_send_pyth() {
        let sender_addr = "inj1x2ck0ql2ngyxqtw8jteyc0tchwnwxv7npaungt";

        let msg = ExecuteMsg::TriggerPythUpdate { price: 10000 };
        let info = mock_info(sender_addr, &[]);
        let env = mock_env();
        let res = execute(inj_mock_deps().as_mut_deps(), env, info, msg);
        assert!(res.is_ok())
    }

    pub fn inj_mock_deps() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InjectiveQueryWrapper>
    {
        let custom_querier: WasmMockQuerier = WasmMockQuerier::new();
        OwnedDeps {
            api: MockApi::default(),
            storage: MockStorage::default(),
            querier: custom_querier,
            custom_query_type: PhantomData::default(),
        }
    }

    pub trait OwnedDepsExt<S, A, Q, C>
    where
        C: CustomQuery,
    {
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
}
