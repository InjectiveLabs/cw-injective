pub use derivative::{
    DerivativeLimitOrder, DerivativeMarketOrder, DerivativeOrder, DerivativePosition, EffectivePosition, Position, TrimmedDerivativeLimitOrder,
};
pub use derivative_market::{
    DerivativeMarket, FullDerivativeMarket, FullDerivativeMarketPerpetualInfo, PerpetualMarketFunding, PerpetualMarketInfo, PerpetualMarketState,
};
pub use exchange::Deposit;
#[cfg(not(target_arch = "wasm32"))]
pub use exchange_mock_querier::handlers::*;
#[cfg(not(target_arch = "wasm32"))]
pub use exchange_mock_querier::*;
pub use msg::{
    cancel_derivative_order_msg, cancel_spot_order_msg, create_activate_contract_msg, create_batch_update_orders_msg, create_burn_tokens_msg,
    create_deactivate_contract_msg, create_deposit_msg, create_derivative_market_order_msg, create_external_transfer_msg,
    create_increase_position_margin_msg, create_liquidate_position_msg, create_mint_tokens_msg, create_new_denom_msg, create_register_as_dmm_msg,
    create_relay_pyth_prices_msg, create_set_token_metadata_msg, create_spot_market_order_msg, create_subaccount_transfer_msg,
    create_update_contract_msg, create_withdraw_msg, InjectiveMsg, InjectiveMsgWrapper,
};
pub use oracle::{OracleInfo, OracleType, PriceAttestation, PythStatus};
pub use order::{GenericOrder, OrderData, OrderInfo, OrderType};
pub use querier::InjectiveQuerier;
pub use query::{
    DenomDecimals, DerivativeMarketResponse, InjectiveQuery, InjectiveQueryWrapper, MarketMidPriceAndTOBResponse, MarketVolatilityResponse,
    OraclePriceResponse, OracleVolatilityResponse, PerpetualMarketFundingResponse, PerpetualMarketInfoResponse, PricePairState, PythPriceResponse,
    QueryAggregateMarketVolumeResponse, QueryAggregateVolumeResponse, QueryDenomDecimalResponse, QueryDenomDecimalsResponse, SpotMarketResponse,
    SubaccountDepositResponse, SubaccountEffectivePositionInMarketResponse, SubaccountPositionInMarketResponse, TokenFactoryDenomSupplyResponse,
    TraderDerivativeOrdersResponse, TraderSpotOrdersResponse, FROM_WORST_TO_BEST_CANCELLATION_STRATEGY, UNSORTED_CANCELLATION_STRATEGY,
};
pub use route::InjectiveRoute;
pub use spot::{MsgCreateSpotMarketOrderResponse, SpotLimitOrder, SpotMarketOrder, SpotOrder, TrimmedSpotLimitOrder};
pub use spot_market::SpotMarket;
pub use subaccount::{
    addr_to_bech32, bech32_to_hex, checked_address_to_subaccount_id, get_default_subaccount_id_for_checked_address, is_default_subaccount,
    subaccount_id_to_ethereum_address, subaccount_id_to_injective_address, subaccount_id_to_unchecked_injective_address,
};
pub use types::{Hash, MarketId, MarketType, SubaccountId};
pub use volatility::{MetadataStatistics, PriceRecord, TradeHistoryOptions, TradeRecord};

mod derivative;
mod derivative_market;
mod exchange;
mod msg;
mod oracle;
mod order;
pub mod privileged_action;
mod querier;
mod query;
mod route;
mod spot;
mod spot_market;
mod subaccount;
mod types;
mod volatility;

#[cfg(not(target_arch = "wasm32"))]
mod exchange_mock_querier;

// This export is added to all contracts that import this package, signifying that they require
// "injective" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_injective() {}
