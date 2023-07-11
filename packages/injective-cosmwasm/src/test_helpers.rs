#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod testing_helpers {

    use std::marker::PhantomData;
    use std::str::FromStr;
    use std::string::ToString;

    use cosmwasm_std::testing::{MockApi, MockStorage};
    use cosmwasm_std::{
        Addr, Api, BlockInfo, ContractInfo, CustomQuery, DepsMut, Env, OwnedDeps, Querier, QuerierWrapper, Storage, Timestamp, TransactionInfo,
    };

    use injective_math::FPDecimal;

    use crate::MarketStatus;
    use crate::{
        exchange::{spot_market::SpotMarket, types::MarketId},
        InjectiveQueryWrapper, WasmMockQuerier,
    };

    pub const TEST_CONTRACT_ADDR: &str = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";

    pub const TEST_MARKET_ID_1: &str = "0xb0f0cd5dc3d18e0407b88a683871399d52483f06c757858a3a9f388877232b11";
    pub const TEST_MARKET_ID_2: &str = "0xa815458b073ea303494e0c87f532483834f85622e1db1ad08e4ece2d360b248d";
    pub const TEST_MARKET_ID_3: &str = "0x0fb00a5b353c58e92ec15e054473b04ecd80c6aec0294cd6702d45e53cdee791";
    pub const TEST_MARKET_ID_4: &str = "0xcde25cd74f38858ac4514412da4363e162bf76f6ae8b462b02e0450d8c8ce78a";
    pub const TEST_MARKET_ID_5: &str = "0x631000a7505094f18446cb6e29a47d1c2a10f44ddb57c021bc0f782adc5ae181";
    pub const TEST_MARKET_ID_6: &str = "0xae28d026f7df038b4a91513181446b9985bdb532485c4d57c8d130d91ba9ce91";
    pub const TEST_MARKET_ID_7: &str = "0x071e1baf54139efc4f4e7897412fde9d06836e12ecbe8b5736c954c0609514d7";
    pub const TEST_MARKET_ID_8: &str = "0x2b0524a1b95942c28de4463d67bb24e0104fd853582a327b3af136d32a84f15d";
    pub const TEST_MARKET_ID_9: &str = "0x92194a85e26a47057c0180465229803c41603b5e9db0dcb54ae4300b023369a4";
    pub const TEST_MARKET_ID_10: &str = "0xc95810e76d34530ce1dc6ca25eb82123e49033f1e5e36db9a0639003267fac32";

    pub fn test_market_ids() -> Vec<MarketId> {
        vec![
            MarketId::new(TEST_MARKET_ID_1.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_2.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_3.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_4.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_5.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_6.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_7.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_8.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_9.to_string()).unwrap(),
            MarketId::new(TEST_MARKET_ID_10.to_string()).unwrap(),
        ]
    }

    pub fn inj_mock_env() -> Env {
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

    pub fn inj_mock_deps<F>(handlers_callback: F) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InjectiveQueryWrapper>
    where
        F: FnOnce(&mut WasmMockQuerier),
    {
        let mut custom_querier: WasmMockQuerier = WasmMockQuerier::new();
        handlers_callback(&mut custom_querier);
        OwnedDeps {
            api: MockApi::default(),
            storage: MockStorage::default(),
            querier: custom_querier,
            custom_query_type: PhantomData::default(),
        }
    }

    pub fn create_mock_spot_market(base: &str, idx: u32) -> SpotMarket {
        SpotMarket {
            ticker: format!("{base}usdt"),
            base_denom: base.to_string(),
            quote_denom: "usdt".to_string(),
            maker_fee_rate: FPDecimal::from_str("0.001").unwrap(),
            taker_fee_rate: FPDecimal::from_str("0.002").unwrap(),
            relayer_fee_share_rate: FPDecimal::from_str("0.4").unwrap(),
            market_id: test_market_ids()[idx as usize].clone(),
            status: MarketStatus::Active,
            min_price_tick_size: FPDecimal::from_str("0.01").unwrap(),
            min_quantity_tick_size: FPDecimal::from_str("0.01").unwrap(),
        }
    }
}
