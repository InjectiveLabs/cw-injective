mod msg;
mod querier;
mod query;
mod route;

pub use msg::{
    InjectiveMsg, InjectiveMsgWrapper,
    create_subaccount_transfer_msg,
};
pub use query::{
    InjectiveQuery, InjectiveQueryWrapper,
    SubaccountDepositResponse, Deposit,
};
pub use querier::InjectiveQuerier;
pub use route::InjectiveRoute;

// TODO: Albert re-enable at some point once we find out how to get around the raw_log: 'failed to execute message; message index: 0: Error calling the VM: Error   during static Wasm validation: Wasm contract requires unsupported features: {"injective"}:   create wasm contract failed'
// This export is added to all contracts that import this package, signifying that they require
// "injective" support on the chain they run on.
// #[no_mangle]
// extern "C" fn requires_injective() {}
