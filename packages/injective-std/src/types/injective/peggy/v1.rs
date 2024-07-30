use injective_std_derive::CosmwasmExt;
/// Attestation is an aggregate of `claims` that eventually becomes `observed` by
/// all orchestrators
/// EVENT_NONCE:
/// EventNonce a nonce provided by the peggy contract that is unique per event
/// fired These event nonces must be relayed in order. This is a correctness
/// issue, if relaying out of order transaction replay attacks become possible
/// OBSERVED:
/// Observed indicates that >67% of validators have attested to the event,
/// and that the event should be executed by the peggy state machine
///
/// The actual content of the claims is passed in with the transaction making the
/// claim and then passed through the call stack alongside the attestation while
/// it is processed the key in which the attestation is stored is keyed on the
/// exact details of the claim but there is no reason to store those exact
/// details becuause the next message sender will kindly provide you with them.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.Attestation")]
pub struct Attestation {
    #[prost(bool, tag = "1")]
    pub observed: bool,
    #[prost(string, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(message, optional, tag = "4")]
    pub claim: ::core::option::Option<crate::shim::Any>,
}
/// ERC20Token unique identifier for an Ethereum ERC20 token.
/// CONTRACT:
/// The contract address on ETH of the token, this could be a Cosmos
/// originated token, if so it will be the ERC20 address of the representation
/// (note: developers should look up the token symbol using the address on ETH to
/// display for UI)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.ERC20Token")]
pub struct Erc20Token {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// ClaimType is the cosmos type of an event from the counterpart chain that can
/// be handled
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ClaimType {
    Unknown = 0,
    Deposit = 1,
    Withdraw = 2,
    Erc20Deployed = 3,
    ValsetUpdated = 4,
}
impl ClaimType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClaimType::Unknown => "CLAIM_TYPE_UNKNOWN",
            ClaimType::Deposit => "CLAIM_TYPE_DEPOSIT",
            ClaimType::Withdraw => "CLAIM_TYPE_WITHDRAW",
            ClaimType::Erc20Deployed => "CLAIM_TYPE_ERC20_DEPLOYED",
            ClaimType::ValsetUpdated => "CLAIM_TYPE_VALSET_UPDATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLAIM_TYPE_UNKNOWN" => Some(Self::Unknown),
            "CLAIM_TYPE_DEPOSIT" => Some(Self::Deposit),
            "CLAIM_TYPE_WITHDRAW" => Some(Self::Withdraw),
            "CLAIM_TYPE_ERC20_DEPLOYED" => Some(Self::Erc20Deployed),
            "CLAIM_TYPE_VALSET_UPDATED" => Some(Self::ValsetUpdated),
            _ => None,
        }
    }
}
/// OutgoingTxBatch represents a batch of transactions going from Peggy to ETH
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.OutgoingTxBatch")]
pub struct OutgoingTxBatch {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_timeout: u64,
    #[prost(message, repeated, tag = "3")]
    pub transactions: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
    #[prost(string, tag = "4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block: u64,
}
/// OutgoingTransferTx represents an individual send from Peggy to ETH
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.OutgoingTransferTx")]
pub struct OutgoingTransferTx {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub dest_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub erc20_token: ::core::option::Option<Erc20Token>,
    #[prost(message, optional, tag = "5")]
    pub erc20_fee: ::core::option::Option<Erc20Token>,
}
/// SignType defines messages that have been signed by an orchestrator
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum SignType {
    Unknown = 0,
    OrchestratorSignedMultiSigUpdate = 1,
    OrchestratorSignedWithdrawBatch = 2,
}
impl SignType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignType::Unknown => "SIGN_TYPE_UNKNOWN",
            SignType::OrchestratorSignedMultiSigUpdate => "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE",
            SignType::OrchestratorSignedWithdrawBatch => "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGN_TYPE_UNKNOWN" => Some(Self::Unknown),
            "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE" => Some(Self::OrchestratorSignedMultiSigUpdate),
            "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH" => Some(Self::OrchestratorSignedWithdrawBatch),
            _ => None,
        }
    }
}
/// BridgeValidator represents a validator's ETH address and its power
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.BridgeValidator")]
pub struct BridgeValidator {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: u64,
    #[prost(string, tag = "2")]
    pub ethereum_address: ::prost::alloc::string::String,
}
/// Valset is the Ethereum Bridge Multsig Set, each peggy validator also
/// maintains an ETH key to sign messages, these are used to check signatures on
/// ETH because of the significant gas savings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.Valset")]
pub struct Valset {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(string, tag = "4")]
    pub reward_amount: ::prost::alloc::string::String,
    /// the reward token in it's Ethereum hex address representation
    #[prost(string, tag = "5")]
    pub reward_token: ::prost::alloc::string::String,
}
/// LastObservedEthereumBlockHeight stores the last observed
/// Ethereum block height along with the Cosmos block height that
/// it was observed at. These two numbers can be used to project
/// outward and always produce batches with timeouts in the future
/// even if no Ethereum block height has been relayed for a long time
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.LastObservedEthereumBlockHeight")]
pub struct LastObservedEthereumBlockHeight {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub cosmos_block_height: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ethereum_block_height: u64,
}
/// LastClaimEvent stores last claim event details of validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.LastClaimEvent")]
pub struct LastClaimEvent {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ethereum_event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ethereum_event_height: u64,
}
/// This records the relationship between an ERC20 token and the denom
/// of the corresponding Cosmos originated asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.ERC20ToDenom")]
pub struct Erc20ToDenom {
    #[prost(string, tag = "1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventAttestationObserved")]
pub struct EventAttestationObserved {
    #[prost(enumeration = "ClaimType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attestation_type: i32,
    #[prost(string, tag = "2")]
    pub bridge_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "bridge_chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub bridge_chain_id: u64,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(alias = "attestationID")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventBridgeWithdrawCanceled")]
pub struct EventBridgeWithdrawCanceled {
    #[prost(string, tag = "1")]
    pub bridge_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "bridge_chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub bridge_chain_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventOutgoingBatch")]
pub struct EventOutgoingBatch {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_nonce: u64,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_timeout: u64,
    #[prost(uint64, repeated, tag = "5")]
    #[serde(alias = "batch_txIDs")]
    pub batch_tx_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventOutgoingBatchCanceled")]
pub struct EventOutgoingBatchCanceled {
    #[prost(string, tag = "1")]
    pub bridge_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "bridge_chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub bridge_chain_id: u64,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "batchID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_id: u64,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventValsetUpdateRequest")]
pub struct EventValsetUpdateRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_height: u64,
    #[prost(message, repeated, tag = "3")]
    pub valset_members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag = "4")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub reward_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventSetOrchestratorAddresses")]
pub struct EventSetOrchestratorAddresses {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub operator_eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventValsetConfirm")]
pub struct EventValsetConfirm {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_nonce: u64,
    #[prost(string, tag = "2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventSendToEth")]
pub struct EventSendToEth {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "outgoing_txID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub outgoing_tx_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub bridge_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventConfirmBatch")]
pub struct EventConfirmBatch {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_nonce: u64,
    #[prost(string, tag = "2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventAttestationVote")]
pub struct EventAttestationVote {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "attestationID")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub voter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventDepositClaim")]
pub struct EventDepositClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_height: u64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "attestationID")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub ethereum_sender: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cosmos_receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventWithdrawClaim")]
pub struct EventWithdrawClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_height: u64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "attestationID")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_nonce: u64,
    #[prost(string, tag = "5")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventERC20DeployedClaim")]
pub struct EventErc20DeployedClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_height: u64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "attestationID")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub cosmos_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: u64,
    #[prost(string, tag = "9")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventValsetUpdateClaim")]
pub struct EventValsetUpdateClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_height: u64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "attestationID")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_nonce: u64,
    #[prost(message, repeated, tag = "5")]
    pub valset_members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag = "6")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventCancelSendToEth")]
pub struct EventCancelSendToEth {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "outgoing_txID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub outgoing_tx_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventSubmitBadSignatureEvidence")]
pub struct EventSubmitBadSignatureEvidence {
    #[prost(string, tag = "1")]
    pub bad_eth_signature: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bad_eth_signature_subject: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.EventValidatorSlash")]
pub struct EventValidatorSlash {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub consensus_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub moniker: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.Params")]
pub struct Params {
    #[prost(string, tag = "1")]
    #[serde(alias = "peggyID")]
    pub peggy_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_source_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bridge_ethereum_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    #[serde(alias = "bridge_chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub bridge_chain_id: u64,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub signed_valsets_window: u64,
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub signed_batches_window: u64,
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub signed_claims_window: u64,
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub target_batch_timeout: u64,
    #[prost(uint64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub average_block_time: u64,
    #[prost(uint64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub average_ethereum_block_time: u64,
    #[prost(bytes = "vec", tag = "11")]
    pub slash_fraction_valset: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "12")]
    pub slash_fraction_batch: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "13")]
    pub slash_fraction_claim: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "14")]
    pub slash_fraction_conflicting_claim: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "15")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbond_slashing_valsets_window: u64,
    #[prost(bytes = "vec", tag = "16")]
    pub slash_fraction_bad_eth_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "17")]
    pub cosmos_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub cosmos_coin_erc20_contract: ::prost::alloc::string::String,
    #[prost(bool, tag = "19")]
    pub claim_slashing_enabled: bool,
    #[prost(uint64, tag = "20")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub bridge_contract_start_height: u64,
    #[prost(message, optional, tag = "21")]
    pub valset_reward: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, repeated, tag = "22")]
    pub admins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSetOrchestratorAddresses
/// this message allows validators to delegate their voting responsibilities
/// to a given key. This key is then used as an optional authentication method
/// for sigining oracle claims
/// VALIDATOR
/// The validator field is a cosmosvaloper1... string (i.e. sdk.ValAddress)
/// that references a validator in the active set
/// ORCHESTRATOR
/// The orchestrator field is a cosmos1... string  (i.e. sdk.AccAddress) that
/// references the key that is being delegated to
/// ETH_ADDRESS
/// This is a hex encoded 0x Ethereum public key that will be used by this
/// validator on Ethereum
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgSetOrchestratorAddresses")]
pub struct MsgSetOrchestratorAddresses {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgSetOrchestratorAddressesResponse")]
pub struct MsgSetOrchestratorAddressesResponse {}
/// MsgValsetConfirm
/// this is the message sent by the validators when they wish to submit their
/// signatures over the validator set at a given block height. A validator must
/// first call MsgSetEthAddress to set their Ethereum address to be used for
/// signing. Then someone (anyone) must make a ValsetRequest the request is
/// essentially a messaging mechanism to determine which block all validators
/// should submit signatures over. Finally validators sign the validator set,
/// powers, and Ethereum addresses of the entire validator set at the height of a
/// ValsetRequest and submit that signature with this message.
///
/// If a sufficient number of validators (66% of voting power) (A) have set
/// Ethereum addresses and (B) submit ValsetConfirm messages with their
/// signatures it is then possible for anyone to view these signatures in the
/// chain store and submit them to Ethereum to update the validator set
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgValsetConfirm")]
pub struct MsgValsetConfirm {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub eth_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgValsetConfirmResponse")]
pub struct MsgValsetConfirmResponse {}
/// MsgSendToEth
/// This is the message that a user calls when they want to bridge an asset
/// it will later be removed when it is included in a batch and successfully
/// submitted tokens are removed from the users balance immediately
/// -------------
/// AMOUNT:
/// the coin to send across the bridge, note the restriction that this is a
/// single coin not a set of coins that is normal in other Cosmos messages
/// FEE:
/// the fee paid for the bridge, distinct from the fee paid to the chain to
/// actually send this message in the first place. So a successful send has
/// two layers of fees for the user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgSendToEth")]
pub struct MsgSendToEth {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub eth_dest: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub bridge_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgSendToEthResponse")]
pub struct MsgSendToEthResponse {}
/// MsgRequestBatch
/// this is a message anyone can send that requests a batch of transactions to
/// send across the bridge be created for whatever block height this message is
/// included in. This acts as a coordination point, the handler for this message
/// looks at the AddToOutgoingPool tx's in the store and generates a batch, also
/// available in the store tied to this message. The validators then grab this
/// batch, sign it, submit the signatures with a MsgConfirmBatch before a relayer
/// can finally submit the batch
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgRequestBatch")]
pub struct MsgRequestBatch {
    #[prost(string, tag = "1")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgRequestBatchResponse")]
pub struct MsgRequestBatchResponse {}
/// MsgConfirmBatch
/// When validators observe a MsgRequestBatch they form a batch by ordering
/// transactions currently in the txqueue in order of highest to lowest fee,
/// cutting off when the batch either reaches a hardcoded maximum size (to be
/// decided, probably around 100) or when transactions stop being profitable
/// (TODO determine this without nondeterminism) This message includes the batch
/// as well as an Ethereum signature over this batch by the validator
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgConfirmBatch")]
pub struct MsgConfirmBatch {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub eth_signer: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgConfirmBatchResponse")]
pub struct MsgConfirmBatchResponse {}
/// EthereumBridgeDepositClaim
/// When more than 66% of the active validator set has
/// claimed to have seen the deposit enter the ethereum blockchain coins are
/// issued to the Cosmos address in question
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgDepositClaim")]
pub struct MsgDepositClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub ethereum_sender: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub cosmos_receiver: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgDepositClaimResponse")]
pub struct MsgDepositClaimResponse {}
/// WithdrawClaim claims that a batch of withdrawal
/// operations on the bridge contract was executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgWithdrawClaim")]
pub struct MsgWithdrawClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub batch_nonce: u64,
    #[prost(string, tag = "4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgWithdrawClaimResponse")]
pub struct MsgWithdrawClaimResponse {}
/// ERC20DeployedClaim allows the Cosmos module
/// to learn about an ERC20 that someone deployed
/// to represent a Cosmos asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgERC20DeployedClaim")]
pub struct MsgErc20DeployedClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub cosmos_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: u64,
    #[prost(string, tag = "8")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgERC20DeployedClaimResponse")]
pub struct MsgErc20DeployedClaimResponse {}
/// This call allows the sender (and only the sender)
/// to cancel a given MsgSendToEth and recieve a refund
/// of the tokens
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgCancelSendToEth")]
pub struct MsgCancelSendToEth {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "transactionID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub transaction_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgCancelSendToEthResponse")]
pub struct MsgCancelSendToEthResponse {}
/// This call allows anyone to submit evidence that a
/// validator has signed a valset, batch, or logic call that never
/// existed. Subject contains the batch, valset, or logic call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgSubmitBadSignatureEvidence")]
pub struct MsgSubmitBadSignatureEvidence {
    #[prost(message, optional, tag = "1")]
    pub subject: ::core::option::Option<crate::shim::Any>,
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgSubmitBadSignatureEvidenceResponse")]
pub struct MsgSubmitBadSignatureEvidenceResponse {}
/// This informs the Cosmos module that a validator
/// set has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgValsetUpdatedClaim")]
pub struct MsgValsetUpdatedClaim {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub event_nonce: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_nonce: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
    #[prost(message, repeated, tag = "4")]
    pub members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag = "5")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgValsetUpdatedClaimResponse")]
pub struct MsgValsetUpdatedClaimResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the peggy parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgBlacklistEthereumAddresses defines the message used to add Ethereum
/// addresses to peggy blacklist.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgBlacklistEthereumAddresses")]
pub struct MsgBlacklistEthereumAddresses {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// Ethereum addresses to include in the blacklist
    #[prost(string, repeated, tag = "2")]
    pub blacklist_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgBlacklistEthereumAddressesResponse defines the
/// MsgBlacklistEthereumAddresses response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgBlacklistEthereumAddressesResponse")]
pub struct MsgBlacklistEthereumAddressesResponse {}
/// MsgRevokeEthereumBlacklist defines the message used to remove Ethereum
/// addresses from peggy blacklist.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgRevokeEthereumBlacklist")]
pub struct MsgRevokeEthereumBlacklist {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// Ethereum addresses to include in the blacklist
    #[prost(string, repeated, tag = "2")]
    pub blacklist_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgRevokeEthereumBlacklistResponse defines the MsgRevokeEthereumBlacklist
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MsgRevokeEthereumBlacklistResponse")]
pub struct MsgRevokeEthereumBlacklistResponse {}
/// GenesisState struct
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_observed_nonce: u64,
    #[prost(message, repeated, tag = "3")]
    pub valsets: ::prost::alloc::vec::Vec<Valset>,
    #[prost(message, repeated, tag = "4")]
    pub valset_confirms: ::prost::alloc::vec::Vec<MsgValsetConfirm>,
    #[prost(message, repeated, tag = "5")]
    pub batches: ::prost::alloc::vec::Vec<OutgoingTxBatch>,
    #[prost(message, repeated, tag = "6")]
    pub batch_confirms: ::prost::alloc::vec::Vec<MsgConfirmBatch>,
    #[prost(message, repeated, tag = "7")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag = "8")]
    pub orchestrator_addresses: ::prost::alloc::vec::Vec<MsgSetOrchestratorAddresses>,
    #[prost(message, repeated, tag = "9")]
    pub erc20_to_denoms: ::prost::alloc::vec::Vec<Erc20ToDenom>,
    #[prost(message, repeated, tag = "10")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
    #[prost(uint64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_observed_ethereum_height: u64,
    #[prost(uint64, tag = "12")]
    #[serde(alias = "last_outgoing_batchID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_outgoing_batch_id: u64,
    #[prost(uint64, tag = "13")]
    #[serde(alias = "last_outgoing_poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_outgoing_pool_id: u64,
    #[prost(message, optional, tag = "14")]
    pub last_observed_valset: ::core::option::Option<Valset>,
    #[prost(string, repeated, tag = "15")]
    pub ethereum_blacklist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// IDSet represents a set of IDs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.IDSet")]
pub struct IdSet {
    #[prost(uint64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.BatchFees")]
pub struct BatchFees {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub total_fees: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryParamsRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryCurrentValsetRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/CurrentValset",
    response_type = QueryCurrentValsetResponse
)]
pub struct QueryCurrentValsetRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryCurrentValsetResponse")]
pub struct QueryCurrentValsetResponse {
    #[prost(message, optional, tag = "1")]
    pub valset: ::core::option::Option<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryValsetRequestRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/ValsetRequest",
    response_type = QueryValsetRequestResponse
)]
pub struct QueryValsetRequestRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryValsetRequestResponse")]
pub struct QueryValsetRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub valset: ::core::option::Option<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryValsetConfirmRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/ValsetConfirm",
    response_type = QueryValsetConfirmResponse
)]
pub struct QueryValsetConfirmRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryValsetConfirmResponse")]
pub struct QueryValsetConfirmResponse {
    #[prost(message, optional, tag = "1")]
    pub confirm: ::core::option::Option<MsgValsetConfirm>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryValsetConfirmsByNonceRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/ValsetConfirmsByNonce",
    response_type = QueryValsetConfirmsByNonceResponse
)]
pub struct QueryValsetConfirmsByNonceRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryValsetConfirmsByNonceResponse")]
pub struct QueryValsetConfirmsByNonceResponse {
    #[prost(message, repeated, tag = "1")]
    pub confirms: ::prost::alloc::vec::Vec<MsgValsetConfirm>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastValsetRequestsRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/LastValsetRequests",
    response_type = QueryLastValsetRequestsResponse
)]
pub struct QueryLastValsetRequestsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastValsetRequestsResponse")]
pub struct QueryLastValsetRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub valsets: ::prost::alloc::vec::Vec<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastPendingValsetRequestByAddrRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/LastPendingValsetRequestByAddr",
    response_type = QueryLastPendingValsetRequestByAddrResponse
)]
pub struct QueryLastPendingValsetRequestByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastPendingValsetRequestByAddrResponse")]
pub struct QueryLastPendingValsetRequestByAddrResponse {
    #[prost(message, repeated, tag = "1")]
    pub valsets: ::prost::alloc::vec::Vec<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryBatchFeeRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/BatchFees",
    response_type = QueryBatchFeeResponse
)]
pub struct QueryBatchFeeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryBatchFeeResponse")]
pub struct QueryBatchFeeResponse {
    #[prost(message, repeated, tag = "1")]
    pub batchFees: ::prost::alloc::vec::Vec<BatchFees>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastPendingBatchRequestByAddrRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/LastPendingBatchRequestByAddr",
    response_type = QueryLastPendingBatchRequestByAddrResponse
)]
pub struct QueryLastPendingBatchRequestByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastPendingBatchRequestByAddrResponse")]
pub struct QueryLastPendingBatchRequestByAddrResponse {
    #[prost(message, optional, tag = "1")]
    pub batch: ::core::option::Option<OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryOutgoingTxBatchesRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/OutgoingTxBatches",
    response_type = QueryOutgoingTxBatchesResponse
)]
pub struct QueryOutgoingTxBatchesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryOutgoingTxBatchesResponse")]
pub struct QueryOutgoingTxBatchesResponse {
    #[prost(message, repeated, tag = "1")]
    pub batches: ::prost::alloc::vec::Vec<OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryBatchRequestByNonceRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/BatchRequestByNonce",
    response_type = QueryBatchRequestByNonceResponse
)]
pub struct QueryBatchRequestByNonceRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryBatchRequestByNonceResponse")]
pub struct QueryBatchRequestByNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub batch: ::core::option::Option<OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryBatchConfirmsRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/BatchConfirms",
    response_type = QueryBatchConfirmsResponse
)]
pub struct QueryBatchConfirmsRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryBatchConfirmsResponse")]
pub struct QueryBatchConfirmsResponse {
    #[prost(message, repeated, tag = "1")]
    pub confirms: ::prost::alloc::vec::Vec<MsgConfirmBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastEventByAddrRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/LastEventByAddr",
    response_type = QueryLastEventByAddrResponse
)]
pub struct QueryLastEventByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryLastEventByAddrResponse")]
pub struct QueryLastEventByAddrResponse {
    #[prost(message, optional, tag = "1")]
    pub last_claim_event: ::core::option::Option<LastClaimEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryERC20ToDenomRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/ERC20ToDenom",
    response_type = QueryErc20ToDenomResponse
)]
pub struct QueryErc20ToDenomRequest {
    #[prost(string, tag = "1")]
    pub erc20: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryERC20ToDenomResponse")]
pub struct QueryErc20ToDenomResponse {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub cosmos_originated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDenomToERC20Request")]
#[proto_query(
    path = "/injective.peggy.v1.Query/DenomToERC20",
    response_type = QueryDenomToErc20Response
)]
pub struct QueryDenomToErc20Request {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDenomToERC20Response")]
pub struct QueryDenomToErc20Response {
    #[prost(string, tag = "1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub cosmos_originated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDelegateKeysByValidatorAddress")]
#[proto_query(
    path = "/injective.peggy.v1.Query/GetDelegateKeyByValidator",
    response_type = QueryDelegateKeysByValidatorAddressResponse
)]
pub struct QueryDelegateKeysByValidatorAddress {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDelegateKeysByValidatorAddressResponse")]
pub struct QueryDelegateKeysByValidatorAddressResponse {
    #[prost(string, tag = "1")]
    pub eth_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDelegateKeysByEthAddress")]
#[proto_query(
    path = "/injective.peggy.v1.Query/GetDelegateKeyByEth",
    response_type = QueryDelegateKeysByEthAddressResponse
)]
pub struct QueryDelegateKeysByEthAddress {
    #[prost(string, tag = "1")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDelegateKeysByEthAddressResponse")]
pub struct QueryDelegateKeysByEthAddressResponse {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDelegateKeysByOrchestratorAddress")]
#[proto_query(
    path = "/injective.peggy.v1.Query/GetDelegateKeyByOrchestrator",
    response_type = QueryDelegateKeysByOrchestratorAddressResponse
)]
pub struct QueryDelegateKeysByOrchestratorAddress {
    #[prost(string, tag = "1")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryDelegateKeysByOrchestratorAddressResponse")]
pub struct QueryDelegateKeysByOrchestratorAddressResponse {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryPendingSendToEth")]
#[proto_query(
    path = "/injective.peggy.v1.Query/GetPendingSendToEth",
    response_type = QueryPendingSendToEthResponse
)]
pub struct QueryPendingSendToEth {
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryPendingSendToEthResponse")]
pub struct QueryPendingSendToEthResponse {
    #[prost(message, repeated, tag = "1")]
    pub transfers_in_batches: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
    #[prost(message, repeated, tag = "2")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
}
/// QueryModuleStateRequest is the request type for the Query/PeggyModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryModuleStateRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/PeggyModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateResponse is the response type for the Query/PeggyModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MissingNoncesRequest")]
#[proto_query(
    path = "/injective.peggy.v1.Query/MissingPeggoNonces",
    response_type = MissingNoncesResponse
)]
pub struct MissingNoncesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.peggy.v1.MissingNoncesResponse")]
pub struct MissingNoncesResponse {
    #[prost(string, repeated, tag = "1")]
    pub operator_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
pub struct PeggyQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PeggyQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn current_valset(&self) -> Result<QueryCurrentValsetResponse, cosmwasm_std::StdError> {
        QueryCurrentValsetRequest {}.query(self.querier)
    }
    pub fn valset_request(&self, nonce: u64) -> Result<QueryValsetRequestResponse, cosmwasm_std::StdError> {
        QueryValsetRequestRequest { nonce }.query(self.querier)
    }
    pub fn valset_confirm(&self, nonce: u64, address: ::prost::alloc::string::String) -> Result<QueryValsetConfirmResponse, cosmwasm_std::StdError> {
        QueryValsetConfirmRequest { nonce, address }.query(self.querier)
    }
    pub fn valset_confirms_by_nonce(&self, nonce: u64) -> Result<QueryValsetConfirmsByNonceResponse, cosmwasm_std::StdError> {
        QueryValsetConfirmsByNonceRequest { nonce }.query(self.querier)
    }
    pub fn last_valset_requests(&self) -> Result<QueryLastValsetRequestsResponse, cosmwasm_std::StdError> {
        QueryLastValsetRequestsRequest {}.query(self.querier)
    }
    pub fn last_pending_valset_request_by_addr(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryLastPendingValsetRequestByAddrResponse, cosmwasm_std::StdError> {
        QueryLastPendingValsetRequestByAddrRequest { address }.query(self.querier)
    }
    pub fn last_event_by_addr(&self, address: ::prost::alloc::string::String) -> Result<QueryLastEventByAddrResponse, cosmwasm_std::StdError> {
        QueryLastEventByAddrRequest { address }.query(self.querier)
    }
    pub fn get_pending_send_to_eth(
        &self,
        sender_address: ::prost::alloc::string::String,
    ) -> Result<QueryPendingSendToEthResponse, cosmwasm_std::StdError> {
        QueryPendingSendToEth { sender_address }.query(self.querier)
    }
    pub fn batch_fees(&self) -> Result<QueryBatchFeeResponse, cosmwasm_std::StdError> {
        QueryBatchFeeRequest {}.query(self.querier)
    }
    pub fn outgoing_tx_batches(&self) -> Result<QueryOutgoingTxBatchesResponse, cosmwasm_std::StdError> {
        QueryOutgoingTxBatchesRequest {}.query(self.querier)
    }
    pub fn last_pending_batch_request_by_addr(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryLastPendingBatchRequestByAddrResponse, cosmwasm_std::StdError> {
        QueryLastPendingBatchRequestByAddrRequest { address }.query(self.querier)
    }
    pub fn batch_request_by_nonce(
        &self,
        nonce: u64,
        contract_address: ::prost::alloc::string::String,
    ) -> Result<QueryBatchRequestByNonceResponse, cosmwasm_std::StdError> {
        QueryBatchRequestByNonceRequest { nonce, contract_address }.query(self.querier)
    }
    pub fn batch_confirms(
        &self,
        nonce: u64,
        contract_address: ::prost::alloc::string::String,
    ) -> Result<QueryBatchConfirmsResponse, cosmwasm_std::StdError> {
        QueryBatchConfirmsRequest { nonce, contract_address }.query(self.querier)
    }
    pub fn erc20_to_denom(&self, erc20: ::prost::alloc::string::String) -> Result<QueryErc20ToDenomResponse, cosmwasm_std::StdError> {
        QueryErc20ToDenomRequest { erc20 }.query(self.querier)
    }
    pub fn denom_to_erc20(&self, denom: ::prost::alloc::string::String) -> Result<QueryDenomToErc20Response, cosmwasm_std::StdError> {
        QueryDenomToErc20Request { denom }.query(self.querier)
    }
    pub fn get_delegate_key_by_validator(
        &self,
        validator_address: ::prost::alloc::string::String,
    ) -> Result<QueryDelegateKeysByValidatorAddressResponse, cosmwasm_std::StdError> {
        QueryDelegateKeysByValidatorAddress { validator_address }.query(self.querier)
    }
    pub fn get_delegate_key_by_eth(
        &self,
        eth_address: ::prost::alloc::string::String,
    ) -> Result<QueryDelegateKeysByEthAddressResponse, cosmwasm_std::StdError> {
        QueryDelegateKeysByEthAddress { eth_address }.query(self.querier)
    }
    pub fn get_delegate_key_by_orchestrator(
        &self,
        orchestrator_address: ::prost::alloc::string::String,
    ) -> Result<QueryDelegateKeysByOrchestratorAddressResponse, cosmwasm_std::StdError> {
        QueryDelegateKeysByOrchestratorAddress { orchestrator_address }.query(self.querier)
    }
    pub fn peggy_module_state(&self) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        QueryModuleStateRequest {}.query(self.querier)
    }
    pub fn missing_peggo_nonces(&self) -> Result<MissingNoncesResponse, cosmwasm_std::StdError> {
        MissingNoncesRequest {}.query(self.querier)
    }
}
