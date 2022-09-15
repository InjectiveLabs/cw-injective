mod derivative;
mod derivative_market;
mod exchange;
mod msg;
mod oracle;
mod order;
mod querier;
mod query;
mod route;
mod spot;
mod spot_market;
mod subaccount;
mod volatility;

pub use msg::{
    create_batch_update_orders_msg, create_burn_tokens_msg, create_deposit_msg, create_derivative_market_order_msg, create_external_transfer_msg,
    create_increase_position_margin_msg, create_liquidate_position_msg, create_mint_tokens_msg, create_register_as_dmm_msg,
    create_spot_market_order_msg, create_subaccount_transfer_msg, create_withdraw_msg, InjectiveMsg, InjectiveMsgWrapper,
};

#[cfg(not(target_arch = "wasm32"))]
mod exchange_mock_querier;

#[cfg(not(target_arch = "wasm32"))]
pub use exchange_mock_querier::{mock_dependencies, WasmMockQuerier};

pub use querier::InjectiveQuerier;
pub use query::{
    DerivativeMarketMidPriceAndTOBResponse, DerivativeMarketResponse, InjectiveQuery, InjectiveQueryWrapper, MarketVolatilityResponse,
    OracleVolatilityResponse, PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, SpotMarketMidPriceAndTOBResponse, SpotMarketResponse,
    SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse, TraderDerivativeOrdersResponse,
    TraderSpotOrdersResponse,
};
pub use route::InjectiveRoute;
pub use subaccount::{
    addr_to_bech32, address_to_subaccount_id, bech32_to_hex, default_subaccount_id, subaccount_id_to_ethereum_address,
    subaccount_id_to_injective_address,
};

pub use order::{OrderData, OrderInfo};

pub use exchange::Deposit;

pub use spot::{SpotLimitOrder, SpotMarketOrder, SpotOrder, TrimmedSpotLimitOrder};

pub use spot_market::SpotMarket;

pub use derivative::{
    DerivativeLimitOrder, DerivativeMarketOrder, DerivativeOrder, DerivativePosition, EffectivePosition, OrderType, Position,
    TrimmedDerivativeLimitOrder,
};
pub use oracle::OracleInfo;

pub use derivative_market::{
    DerivativeMarket, FullDerivativeMarket, FullDerivativeMarketPerpetualInfo, PerpetualMarketFunding, PerpetualMarketInfo, PerpetualMarketState,
};

pub use volatility::{MetadataStatistics, PriceRecord, TradeHistoryOptions, TradeRecord};

// This export is added to all contracts that import this package, signifying that they require
// "injective" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_injective() {}
