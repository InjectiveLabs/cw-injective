use osmosis_std_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.ContractRegistrationRequestProposal")]
pub struct ContractRegistrationRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub contract_registration_request: ::core::option::Option<ContractRegistrationRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.BatchContractRegistrationRequestProposal")]
pub struct BatchContractRegistrationRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub contract_registration_requests: ::prost::alloc::vec::Vec<ContractRegistrationRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.BatchContractDeregistrationProposal")]
pub struct BatchContractDeregistrationProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.ContractRegistrationRequest")]
pub struct ContractRegistrationRequest {
    /// Unique Identifier for contract instance to be registered.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// Maximum gas to be used for the smart contract execution.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_limit: u64,
    /// gas price to be used for the smart contract execution.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_price: u64,
    #[prost(bool, tag = "4")]
    pub should_pin_contract: bool,
    /// if true contract owner can update it, if false only current code_id will be
    /// allowed to be executed
    #[prost(bool, tag = "5")]
    pub is_migration_allowed: bool,
    /// code_id of the contract being registered - will be verified upon every
    /// execution but only if is_migration_allowed is false
    #[prost(uint64, tag = "6")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Optional address of admin account (that will be allowed to pause or update
    /// contract params)
    #[prost(string, tag = "7")]
    pub admin_address: ::prost::alloc::string::String,
    /// Optional address of the contract that grants fees. Must be set if
    /// funding_mode is other than SelfFunded
    #[prost(string, tag = "8")]
    pub granter_address: ::prost::alloc::string::String,
    /// Specifies how the contract will fund its execution
    #[prost(enumeration = "FundingMode", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub funding_mode: i32,
}
// #[allow(clippy::derive_partial_eq_without_eq)]
// #[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
// #[proto_message(type_url = "/injective.wasmx.v1.BatchStoreCodeProposal")]
// pub struct BatchStoreCodeProposal {
//     #[prost(string, tag = "1")]
//     pub title: ::prost::alloc::string::String,
//     #[prost(string, tag = "2")]
//     pub description: ::prost::alloc::string::String,
//     #[prost(message, repeated, tag = "3")]
//     pub proposals: ::prost::alloc::vec::Vec<super::super::super::cosmwasm::wasm::v1::StoreCodeProposal>,
// }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum FundingMode {
    Unspecified = 0,
    SelfFunded = 1,
    GrantOnly = 2,
    Dual = 3,
}
impl FundingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FundingMode::Unspecified => "Unspecified",
            FundingMode::SelfFunded => "SelfFunded",
            FundingMode::GrantOnly => "GrantOnly",
            FundingMode::Dual => "Dual",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "SelfFunded" => Some(Self::SelfFunded),
            "GrantOnly" => Some(Self::GrantOnly),
            "Dual" => Some(Self::Dual),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.Params")]
pub struct Params {
    /// Set the status to active to indicate that contracts can be executed in
    /// begin blocker.
    #[prost(bool, tag = "1")]
    pub is_execution_enabled: bool,
    /// Maximum aggregate total gas to be used for the contract executions in the
    /// BeginBlocker.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_begin_block_total_gas: u64,
    /// the maximum gas limit each individual contract can consume in the
    /// BeginBlocker.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_contract_gas_limit: u64,
    /// min_gas_price defines the minimum gas price the contracts must pay to be
    /// executed in the BeginBlocker.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_gas_price: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.RegisteredContract")]
pub struct RegisteredContract {
    /// limit of gas per BB execution
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_limit: u64,
    /// gas price that contract is willing to pay for execution in BeginBlocker
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_price: u64,
    /// is contract currently active
    #[prost(bool, tag = "3")]
    pub is_executable: bool,
    /// code_id that is allowed to be executed (to prevent malicious updates) - if
    /// nil/0 any code_id can be executed
    #[prost(uint64, tag = "4")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// optional - admin addr that is allowed to update contract data
    #[prost(string, tag = "5")]
    pub admin_address: ::prost::alloc::string::String,
    /// Optional: address of the contract granting fee
    /// Must be set if fund_mode is GrantOnly
    #[prost(string, tag = "6")]
    pub granter_address: ::prost::alloc::string::String,
    /// funding mode
    #[prost(enumeration = "FundingMode", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub fund_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.EventContractExecution")]
pub struct EventContractExecution {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub response: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.RegisteredContractWithAddress")]
pub struct RegisteredContractWithAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub registered_contract: ::core::option::Option<RegisteredContract>,
}
/// GenesisState defines the wasmx module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to wasmx.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// registered_contracts is an array containing the genesis registered
    /// contracts
    #[prost(message, repeated, tag = "2")]
    pub registered_contracts: ::prost::alloc::vec::Vec<RegisteredContractWithAddress>,
}
/// QueryWasmxParamsRequest is the request type for the Query/WasmxParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.QueryWasmxParamsRequest")]
#[proto_query(
    path = "/injective.wasmx.v1.Query/WasmxParams",
    response_type = QueryWasmxParamsResponse
)]
pub struct QueryWasmxParamsRequest {}
/// QueryWasmxParamsRequest is the response type for the Query/WasmxParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.QueryWasmxParamsResponse")]
pub struct QueryWasmxParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryModuleStateRequest is the request type for the Query/WasmxModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.QueryModuleStateRequest")]
#[proto_query(
    path = "/injective.wasmx.v1.Query/WasmxModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateResponse is the response type for the Query/WasmxModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
/// Contract registration info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.QueryContractRegistrationInfoRequest")]
#[proto_query(
    path = "/injective.wasmx.v1.Query/ContractRegistrationInfo",
    response_type = QueryContractRegistrationInfoResponse
)]
pub struct QueryContractRegistrationInfoRequest {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.QueryContractRegistrationInfoResponse")]
pub struct QueryContractRegistrationInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<RegisteredContract>,
}
/// MsgExecuteContractCompat submits the given message data to a smart contract,
/// compatible with EIP712
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgExecuteContractCompat")]
pub struct MsgExecuteContractCompat {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
    /// Funds coins that are transferred to the contract on execution
    #[prost(string, tag = "4")]
    pub funds: ::prost::alloc::string::String,
}
/// MsgExecuteContractCompatResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgExecuteContractCompatResponse")]
pub struct MsgExecuteContractCompatResponse {
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgUpdateContract")]
pub struct MsgUpdateContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Unique Identifier for contract instance to be registered.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    /// Maximum gas to be used for the smart contract execution.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_limit: u64,
    /// gas price to be used for the smart contract execution.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_price: u64,
    /// optional - admin account that will be allowed to perform any changes
    #[prost(string, tag = "5")]
    pub admin_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgUpdateContractResponse")]
pub struct MsgUpdateContractResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgActivateContract")]
pub struct MsgActivateContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Unique Identifier for contract instance to be activated.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgActivateContractResponse")]
pub struct MsgActivateContractResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgDeactivateContract")]
pub struct MsgDeactivateContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Unique Identifier for contract instance to be deactivated.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgDeactivateContractResponse")]
pub struct MsgDeactivateContractResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the wasmx parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgRegisterContract")]
pub struct MsgRegisterContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_registration_request: ::core::option::Option<ContractRegistrationRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.wasmx.v1.MsgRegisterContractResponse")]
pub struct MsgRegisterContractResponse {}
pub struct WasmxQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> WasmxQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn wasmx_params(&self) -> Result<QueryWasmxParamsResponse, cosmwasm_std::StdError> {
        QueryWasmxParamsRequest {}.query(self.querier)
    }
    pub fn contract_registration_info(
        &self,
        contract_address: ::prost::alloc::string::String,
    ) -> Result<QueryContractRegistrationInfoResponse, cosmwasm_std::StdError> {
        QueryContractRegistrationInfoRequest { contract_address }.query(self.querier)
    }
    pub fn wasmx_module_state(&self) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        QueryModuleStateRequest {}.query(self.querier)
    }
}
