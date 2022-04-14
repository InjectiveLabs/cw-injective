mod msg;
mod querier;
mod query;
mod route;

pub use msg::{
    create_batch_update_orders_msg, create_derivative_market_order_msg, create_subaccount_transfer_msg, DerivativeOrder, InjectiveMsg,
    InjectiveMsgWrapper, OrderData, OrderInfo, SpotOrder,
};

pub use querier::InjectiveQuerier;
pub use query::{
    Deposit, DerivativeLimitOrder, DerivativeMarket, EffectivePosition, InjectiveQuery, InjectiveQueryWrapper, PerpetualMarketFunding,
    PerpetualMarketInfo, SubaccountDepositResponse,
};
pub use route::InjectiveRoute;

// This export is added to all contracts that import this package, signifying that they require
// "injective" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_injective() {}
