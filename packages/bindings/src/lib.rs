mod msg;
mod querier;
mod query;
mod route;

pub use msg::{create_subaccount_transfer_msg, InjectiveMsg, InjectiveMsgWrapper};
pub use querier::InjectiveQuerier;
pub use query::{Deposit, InjectiveQuery, InjectiveQueryWrapper, SubaccountDepositResponse};
pub use route::InjectiveRoute;

// This export is added to all contracts that import this package, signifying that they require
// "injective" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_injective() {}
