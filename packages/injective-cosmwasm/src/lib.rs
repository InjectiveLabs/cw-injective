mod derivative;
mod derivative_market;
mod exchange;
mod msg;
mod order;
mod querier;
mod query;
mod route;
mod spot;
mod spot_market;
mod subaccount;
mod oracle;
mod volatility;

pub use msg::{
    create_batch_update_orders_msg, create_deposit_msg, create_derivative_market_order_msg, create_external_transfer_msg,
    create_increase_position_margin_msg, create_liquidate_position_msg, create_register_as_dmm_msg, create_spot_market_order_msg,
    create_subaccount_transfer_msg, create_withdraw_msg, InjectiveMsg, InjectiveMsgWrapper,
};

pub use querier::InjectiveQuerier;
pub use query::{
    MarketVolatilityResponse, OracleVolatilityResponse, InjectiveQuery, InjectiveQueryWrapper,
    SubaccountDepositResponse,
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
    DerivativeLimitOrder, DerivativeMarketOrder, DerivativeOrder, DerivativePosition, EffectivePosition, Position, TrimmedDerivativeLimitOrder,
};

pub use derivative_market::{
    DerivativeMarket, FullDerivativeMarket, FullDerivativeMarketPerpetualInfo, PerpetualMarketFunding, PerpetualMarketInfo, PerpetualMarketState,
};

// This export is added to all contracts that import this package, signifying that they require
// "injective" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_injective() {}
