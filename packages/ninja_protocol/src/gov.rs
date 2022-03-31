use cosmwasm_std::{Binary, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::common::OrderBy;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub ninja_token: String,
    pub effective_delay: u64,
    pub default_poll_config: PollConfig,
    pub migration_poll_config: PollConfig,
    pub auth_admin_poll_config: PollConfig,
    pub voter_weight: Decimal,
    pub snapshot_period: u64,
    pub admin_manager: String,
    pub poll_gas_limit: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    UpdateConfig {
        owner: Option<String>,
        effective_delay: Option<u64>,
        default_poll_config: Option<PollConfig>,
        migration_poll_config: Option<PollConfig>,
        auth_admin_poll_config: Option<PollConfig>,
        voter_weight: Option<Decimal>,
        snapshot_period: Option<u64>,
        admin_manager: Option<String>,
        poll_gas_limit: Option<u64>,
    },
    CastVote {
        poll_id: u64,
        vote: VoteOption,
        amount: Uint128,
    },
    WithdrawVotingTokens {
        amount: Option<Uint128>,
    },
    WithdrawVotingRewards {
        poll_id: Option<u64>,
    },
    StakeVotingRewards {
        poll_id: Option<u64>,
    },
    EndPoll {
        poll_id: u64,
    },
    ExecutePoll {
        poll_id: u64,
    },
    SnapshotPoll {
        poll_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum Cw20HookMsg {
    /// StakeVotingTokens a user can stake their Ninja token to receive rewards
    /// or do vote on polls
    StakeVotingTokens {},
    /// CreatePoll need to receive deposit from a proposer
    CreatePoll {
        title: String,
        description: String,
        link: Option<String>,
        execute_msg: Option<PollExecuteMsg>,
        admin_action: Option<PollAdminAction>,
    },
    /// Deposit rewards to be distributed among stakers and voters
    DepositReward {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PollExecuteMsg {
    pub contract: String,
    pub msg: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PollConfig {
    pub proposal_deposit: Uint128,
    pub voting_period: u64,
    pub quorum: Decimal,
    pub threshold: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum PollAdminAction {
    /// Updates migration manager owner
    UpdateOwner { owner: String },
    /// Executes a set of migrations. The poll can be executes as soon as it reaches the quorum and threshold
    ExecuteMigrations {
        migrations: Vec<(String, u64, Binary)>,
    },
    /// Transfer admin privileges over Ninja contracts to the authorized_addr
    AuthorizeClaim { authorized_addr: String },
    /// Updates Governace contract configuration
    UpdateConfig {
        owner: Option<String>,
        effective_delay: Option<u64>,
        default_poll_config: Option<PollConfig>,
        migration_poll_config: Option<PollConfig>,
        auth_admin_poll_config: Option<PollConfig>,
        voter_weight: Option<Decimal>,
        snapshot_period: Option<u64>,
        admin_manager: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    Staker {
        address: String,
    },
    Poll {
        poll_id: u64,
    },
    Polls {
        filter: Option<PollStatus>,
        start_after: Option<u64>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
    Voter {
        poll_id: u64,
        address: String,
    },
    Voters {
        poll_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
    Shares {
        start_after: Option<String>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub ninja_token: String,
    pub effective_delay: u64,
    pub default_poll_config: PollConfig,
    pub migration_poll_config: PollConfig,
    pub auth_admin_poll_config: PollConfig,
    pub voter_weight: Decimal,
    pub snapshot_period: u64,
    pub admin_manager: String,
    pub poll_gas_limit: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub poll_count: u64,
    pub total_share: Uint128,
    pub total_deposit: Uint128,
    pub pending_voting_rewards: Uint128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct PollResponse {
    pub id: u64,
    pub creator: String,
    pub status: PollStatus,
    pub end_time: u64,
    pub title: String,
    pub description: String,
    pub link: Option<String>,
    pub deposit_amount: Uint128,
    pub execute_data: Option<PollExecuteMsg>,
    pub yes_votes: Uint128,     // balance
    pub no_votes: Uint128,      // balance
    pub abstain_votes: Uint128, // balance
    pub total_balance_at_end_poll: Option<Uint128>,
    pub voters_reward: Uint128,
    pub staked_amount: Option<Uint128>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct PollsResponse {
    pub polls: Vec<PollResponse>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct PollCountResponse {
    pub poll_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct StakerResponse {
    pub balance: Uint128,
    pub share: Uint128,
    pub locked_balance: Vec<(u64, VoterInfo)>,
    pub withdrawable_polls: Vec<(u64, Uint128)>,
    pub pending_voting_rewards: Uint128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct SharesResponseItem {
    pub staker: String,
    pub share: Uint128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct SharesResponse {
    pub stakers: Vec<SharesResponseItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct VotersResponseItem {
    pub voter: String,
    pub vote: VoteOption,
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct VotersResponse {
    pub voters: Vec<VotersResponseItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {
    pub migration_poll_config: PollConfig,
    pub auth_admin_poll_config: PollConfig,
    pub admin_manager: String,
    pub poll_gas_limit: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoterInfo {
    pub vote: VoteOption,
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PollStatus {
    InProgress,
    Passed,
    Rejected,
    Executed,
    Expired,
    Failed,
}

impl fmt::Display for PollStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VoteOption {
    Yes,
    No,
    Abstain,
}

impl fmt::Display for VoteOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VoteOption::Yes => write!(f, "yes"),
            VoteOption::No => write!(f, "no"),
            VoteOption::Abstain => write!(f, "abstain"),
        }
    }
}
