use osmosis_std_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.EventSetVoucher")]
pub struct EventSetVoucher {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Params defines the parameters for the permissions module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.Params")]
pub struct Params {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub wasm_hook_query_max_gas: u64,
}
/// Namespace defines a permissions namespace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.Namespace")]
pub struct Namespace {
    /// tokenfactory denom to which this namespace applies to
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// address of smart contract to apply code-based restrictions
    #[prost(string, tag = "2")]
    pub wasm_hook: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub mints_paused: bool,
    #[prost(bool, tag = "4")]
    pub sends_paused: bool,
    #[prost(bool, tag = "5")]
    pub burns_paused: bool,
    /// permissions for each role
    #[prost(message, repeated, tag = "6")]
    pub role_permissions: ::prost::alloc::vec::Vec<Role>,
    #[prost(message, repeated, tag = "7")]
    pub address_roles: ::prost::alloc::vec::Vec<AddressRoles>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.AddressRoles")]
pub struct AddressRoles {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Role is only used for storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.Role")]
pub struct Role {
    #[prost(string, tag = "1")]
    pub role: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub permissions: u32,
}
/// used in storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.RoleIDs")]
pub struct RoleIDs {
    #[prost(uint32, repeated, tag = "1")]
    #[serde(alias = "roleIDs")]
    pub role_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.Voucher")]
pub struct Voucher {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.AddressVoucher")]
pub struct AddressVoucher {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<Voucher>,
}
/// each Action enum value should be a power of two
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Action {
    Unspecified = 0,
    Mint = 1,
    Receive = 2,
    Burn = 4,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unspecified => "UNSPECIFIED",
            Action::Mint => "MINT",
            Action::Receive => "RECEIVE",
            Action::Burn => "BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "MINT" => Some(Self::Mint),
            "RECEIVE" => Some(Self::Receive),
            "BURN" => Some(Self::Burn),
            _ => None,
        }
    }
}
/// GenesisState defines the permissions module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/injective.permissions.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllNamespacesRequest is the request type for the Query/AllNamespaces RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryAllNamespacesRequest")]
#[proto_query(
    path = "/injective.permissions.v1beta1.Query/AllNamespaces",
    response_type = QueryAllNamespacesResponse
)]
pub struct QueryAllNamespacesRequest {}
/// QueryAllNamespacesResponse is the response type for the Query/AllNamespaces
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryAllNamespacesResponse")]
pub struct QueryAllNamespacesResponse {
    #[prost(message, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
}
/// QueryNamespaceByDenomRequest is the request type for the
/// Query/NamespaceByDenom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryNamespaceByDenomRequest")]
#[proto_query(
    path = "/injective.permissions.v1beta1.Query/NamespaceByDenom",
    response_type = QueryNamespaceByDenomResponse
)]
pub struct QueryNamespaceByDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub include_roles: bool,
}
/// QueryNamespaceByDenomResponse is the response type for the
/// Query/NamespaceByDenom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryNamespaceByDenomResponse")]
pub struct QueryNamespaceByDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub namespace: ::core::option::Option<Namespace>,
}
/// QueryAddressesByRoleRequest is the request type for the Query/AddressesByRole
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryAddressesByRoleRequest")]
#[proto_query(
    path = "/injective.permissions.v1beta1.Query/AddressesByRole",
    response_type = QueryAddressesByRoleResponse
)]
pub struct QueryAddressesByRoleRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
/// QueryAddressesByRoleResponse is the response type for the
/// Query/AddressesByRole RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryAddressesByRoleResponse")]
pub struct QueryAddressesByRoleResponse {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryAddressRolesRequest")]
#[proto_query(
    path = "/injective.permissions.v1beta1.Query/AddressRoles",
    response_type = QueryAddressRolesResponse
)]
pub struct QueryAddressRolesRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryAddressRolesResponse")]
pub struct QueryAddressRolesResponse {
    #[prost(string, repeated, tag = "1")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryVouchersForAddressRequest")]
#[proto_query(
    path = "/injective.permissions.v1beta1.Query/VouchersForAddress",
    response_type = QueryVouchersForAddressResponse
)]
pub struct QueryVouchersForAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.QueryVouchersForAddressResponse")]
pub struct QueryVouchersForAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub vouchers: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the permissions parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgCreateNamespace")]
pub struct MsgCreateNamespace {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub namespace: ::core::option::Option<Namespace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgCreateNamespaceResponse")]
pub struct MsgCreateNamespaceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgDeleteNamespace")]
pub struct MsgDeleteNamespace {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgDeleteNamespaceResponse")]
pub struct MsgDeleteNamespaceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespace")]
pub struct MsgUpdateNamespace {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
    /// address of smart contract to apply code-based restrictions
    #[prost(message, optional, tag = "3")]
    pub wasm_hook: ::core::option::Option<msg_update_namespace::MsgSetWasmHook>,
    #[prost(message, optional, tag = "4")]
    pub mints_paused: ::core::option::Option<msg_update_namespace::MsgSetMintsPaused>,
    #[prost(message, optional, tag = "5")]
    pub sends_paused: ::core::option::Option<msg_update_namespace::MsgSetSendsPaused>,
    #[prost(message, optional, tag = "6")]
    pub burns_paused: ::core::option::Option<msg_update_namespace::MsgSetBurnsPaused>,
}
/// Nested message and enum types in `MsgUpdateNamespace`.
pub mod msg_update_namespace {
    use osmosis_std_derive::CosmwasmExt;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
    #[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespace.MsgSetWasmHook")]
    pub struct MsgSetWasmHook {
        #[prost(string, tag = "1")]
        pub new_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
    #[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespace.MsgSetMintsPaused")]
    pub struct MsgSetMintsPaused {
        #[prost(bool, tag = "1")]
        pub new_value: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
    #[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespace.MsgSetSendsPaused")]
    pub struct MsgSetSendsPaused {
        #[prost(bool, tag = "1")]
        pub new_value: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
    #[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespace.MsgSetBurnsPaused")]
    pub struct MsgSetBurnsPaused {
        #[prost(bool, tag = "1")]
        pub new_value: bool,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespaceResponse")]
pub struct MsgUpdateNamespaceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespaceRoles")]
pub struct MsgUpdateNamespaceRoles {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
    /// new role definitions or updated permissions for existing roles
    #[prost(message, repeated, tag = "3")]
    pub role_permissions: ::prost::alloc::vec::Vec<Role>,
    /// new addresses to add or new roles for existing addresses to
    #[prost(message, repeated, tag = "4")]
    pub address_roles: ::prost::alloc::vec::Vec<AddressRoles>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgUpdateNamespaceRolesResponse")]
pub struct MsgUpdateNamespaceRolesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgRevokeNamespaceRoles")]
pub struct MsgRevokeNamespaceRoles {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
    /// {"address" => array of roles to revoke from this address}
    #[prost(message, repeated, tag = "3")]
    pub address_roles_to_revoke: ::prost::alloc::vec::Vec<AddressRoles>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgRevokeNamespaceRolesResponse")]
pub struct MsgRevokeNamespaceRolesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgClaimVoucher")]
pub struct MsgClaimVoucher {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.permissions.v1beta1.MsgClaimVoucherResponse")]
pub struct MsgClaimVoucherResponse {}
pub struct PermissionsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PermissionsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn all_namespaces(&self) -> Result<QueryAllNamespacesResponse, cosmwasm_std::StdError> {
        QueryAllNamespacesRequest {}.query(self.querier)
    }
    pub fn namespace_by_denom(
        &self,
        denom: ::prost::alloc::string::String,
        include_roles: bool,
    ) -> Result<QueryNamespaceByDenomResponse, cosmwasm_std::StdError> {
        QueryNamespaceByDenomRequest { denom, include_roles }.query(self.querier)
    }
    pub fn address_roles(
        &self,
        denom: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryAddressRolesResponse, cosmwasm_std::StdError> {
        QueryAddressRolesRequest { denom, address }.query(self.querier)
    }
    pub fn addresses_by_role(
        &self,
        denom: ::prost::alloc::string::String,
        role: ::prost::alloc::string::String,
    ) -> Result<QueryAddressesByRoleResponse, cosmwasm_std::StdError> {
        QueryAddressesByRoleRequest { denom, role }.query(self.querier)
    }
    pub fn vouchers_for_address(&self, address: ::prost::alloc::string::String) -> Result<QueryVouchersForAddressResponse, cosmwasm_std::StdError> {
        QueryVouchersForAddressRequest { address }.query(self.querier)
    }
}
