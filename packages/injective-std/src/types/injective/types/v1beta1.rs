// @generated
/// EthAccount implements the authtypes.AccountI interface and embeds an
/// authtypes.BaseAccount type. It is compatible with the auth AccountKeeper.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAccount {
    #[prost(message, optional, tag="1")]
    pub base_account: ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(bytes="vec", tag="2")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
