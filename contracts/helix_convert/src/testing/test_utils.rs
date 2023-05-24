use std::collections::HashMap;
use std::str::FromStr;

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{Addr, Coin, OwnedDeps};
use injective_std::types::injective::exchange::v1beta1::{
    MsgCreateSpotLimitOrder, MsgInstantSpotMarketLaunch, OrderInfo, OrderType,
    QuerySpotMarketsRequest, SpotOrder,
};
use injective_test_tube::{Account, Bank, Exchange, InjectiveTestApp, Module, SigningAccount, Wasm};
use injective_std::types::cosmos::bank::v1beta1::{QueryAllBalancesRequest, QueryBalanceRequest};

use injective_cosmwasm::{
    create_mock_spot_market, create_orderbook_response_handler, create_spot_multi_market_handler,
    get_default_subaccount_id_for_checked_address, inj_mock_deps, InjectiveQueryWrapper, MarketId,
    PriceLevel, WasmMockQuerier, TEST_MARKET_ID_1, TEST_MARKET_ID_2,
};
use injective_math::FPDecimal;

use crate::msg::{ExecuteMsg, FeeRecipient, InstantiateMsg};

pub const TEST_CONTRACT_ADDR: &str = "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz";
pub const TEST_USER_ADDR: &str = "inj1p7z8p649xspcey7wp5e4leqf7wa39kjjj6wja8";

// Helper function to create a PriceLevel
pub fn create_price_level(p: u128, q: u128) -> PriceLevel {
    PriceLevel {
        p: FPDecimal::from(p),
        q: FPDecimal::from(q),
    }
}

pub fn mock_deps_eth_inj() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, InjectiveQueryWrapper>
{
    inj_mock_deps(|querier| {
        let mut markets = HashMap::new();
        markets.insert(
            MarketId::new(TEST_MARKET_ID_1).unwrap(),
            create_mock_spot_market("eth", 0),
        );
        markets.insert(
            MarketId::new(TEST_MARKET_ID_2).unwrap(),
            create_mock_spot_market("inj", 1),
        );
        querier.spot_market_response_handler = create_spot_multi_market_handler(markets);

        let mut orderbooks = HashMap::new();
        let eth_buy_orderbook = vec![
            PriceLevel {
                p: 201000u128.into(),
                q: FPDecimal::from_str("5").unwrap(),
            },
            PriceLevel {
                p: 195000u128.into(),
                q: FPDecimal::from_str("4").unwrap(),
            },
            PriceLevel {
                p: 192000u128.into(),
                q: FPDecimal::from_str("3").unwrap(),
            },
        ];
        orderbooks.insert(MarketId::new(TEST_MARKET_ID_1).unwrap(), eth_buy_orderbook);

        let inj_sell_orderbook = vec![
            PriceLevel {
                p: 800u128.into(),
                q: 800u128.into(),
            },
            PriceLevel {
                p: 810u128.into(),
                q: 800u128.into(),
            },
            PriceLevel {
                p: 820u128.into(),
                q: 800u128.into(),
            },
            PriceLevel {
                p: 830u128.into(),
                q: 800u128.into(),
            },
        ];
        orderbooks.insert(MarketId::new(TEST_MARKET_ID_2).unwrap(), inj_sell_orderbook);

        querier.spot_market_orderbook_response_handler =
            create_orderbook_response_handler(orderbooks);
    })
}

pub fn wasm_file(contract_name: String) -> String {
    let arch = std::env::consts::ARCH;
    let artifacts_dir =
        std::env::var("ARTIFACTS_DIR_PATH").unwrap_or_else(|_| "artifacts".to_string());
    let snaked_name = contract_name.replace('-', "_");
    format!("../../{artifacts_dir}/{snaked_name}-{arch}.wasm")
}

pub fn store_code(
    wasm: &Wasm<InjectiveTestApp>,
    owner: &SigningAccount,
    contract_name: String,
) -> u64 {
    let wasm_byte_code = std::fs::read(wasm_file(contract_name)).unwrap();
    wasm.store_code(&wasm_byte_code, None, owner)
        .unwrap()
        .data
        .code_id
}

pub fn launch_spot_market(
    exchange: &Exchange<InjectiveTestApp>,
    signer: &SigningAccount,
    base: &str,
    quote: &str,
) -> String {
    let ticker = format!("{}/{}", base, quote);
    exchange
        .instant_spot_market_launch(
            MsgInstantSpotMarketLaunch {
                sender: signer.address(),
                ticker: ticker.clone(),
                base_denom: base.to_string(),
                quote_denom: quote.to_string(),
                min_price_tick_size: "1_000_000_000_000_000".to_owned(),
                min_quantity_tick_size: "1_000_000_000_000_000".to_owned(),
            },
            signer,
        )
        .unwrap();

    get_spot_market_id(exchange, ticker)
}

pub fn get_spot_market_id(exchange: &Exchange<InjectiveTestApp>, ticker: String) -> String {
    let spot_markets = exchange
        .query_spot_markets(&QuerySpotMarketsRequest {
            status: "Active".to_string(),
        })
        .unwrap()
        .markets;

    let market = spot_markets.iter().find(|m| m.ticker == ticker).unwrap();

    market.market_id.to_string()
}

#[derive(PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

pub fn create_limit_order(
    app: &InjectiveTestApp,
    trader: &SigningAccount,
    market_id: &String,
    order_side: OrderSide,
    price: u128,
    quantity: u32,
) {
    let exchange = Exchange::new(app);
    exchange
        .create_spot_limit_order(
            MsgCreateSpotLimitOrder {
                sender: trader.address(),
                order: Some(SpotOrder {
                    market_id: market_id.clone(),
                    order_info: Some(OrderInfo {
                        subaccount_id: get_default_subaccount_id_for_checked_address(
                            &Addr::unchecked(trader.address()),
                        )
                        .to_string(),
                        fee_recipient: trader.address(),
                        price: format!("{}000000000000000000", price),
                        quantity: format!("{}000000000000000000", quantity),
                    }),
                    order_type: if order_side == OrderSide::Buy {
                        OrderType::BuyAtomic.into()
                    } else {
                        OrderType::SellAtomic.into()
                    },
                    trigger_price: "".to_string(),
                }),
            },
            trader,
        )
        .unwrap();
}

pub fn init_contract_and_get_address(wasm: &Wasm<InjectiveTestApp>, owner: &SigningAccount, initial_balance: &[Coin]) -> String {
    let code_id = store_code(wasm, owner, "helix_converter".to_string());
    wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                fee_recipient: FeeRecipient::SwapContract,
                admin: Addr::unchecked(owner.address()),
            },
            Some(&owner.address()),
            Some("Swap"),
            initial_balance,
            owner,
        )
        .unwrap()
        .data
        .address
}

pub fn set_route_and_assert_success(wasm: &Wasm<InjectiveTestApp>, signer: &SigningAccount, contr_addr: &str, from_denom: &str, to_denom: &str, route: Vec<MarketId>) {
    wasm.execute(
        contr_addr,
        &ExecuteMsg::SetRoute {
            denom_1: from_denom.to_string(),
            denom_2: to_denom.to_string(),
            route,
        },
        &[],
        signer,
    ).unwrap();
}

pub fn must_init_account_with_funds(app: &InjectiveTestApp, initial_funds: &[Coin]) -> SigningAccount {
    app
        .init_account(initial_funds)
        .unwrap()
}

pub fn query_all_bank_balances(bank: &Bank<InjectiveTestApp>, address: &str) -> Vec<injective_std::types::cosmos::base::v1beta1::Coin> {
    bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: address.to_string(),
            pagination: None,
        })
        .unwrap()
        .balances
}


pub fn query_bank_balance(bank: &Bank<InjectiveTestApp>, denom: &str, address: &str) -> FPDecimal {
    FPDecimal::from_str(bank
        .query_balance(&QueryBalanceRequest {
            address: address.to_string(),
            denom: denom.to_string(),
        })
        .unwrap()
        .balance
        .unwrap()
        .amount.as_str())
        .unwrap()
}
