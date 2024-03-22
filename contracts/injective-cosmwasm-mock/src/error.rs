use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unrecognized reply id: {0}")]
    UnrecognizedReply(u64),
    #[error("Invalid reply from sub-message {id}, {err}")]
    ReplyParseFailure { id: u64, err: String },
    #[error("Failure response from submsg: {0}")]
    SubMsgFailure(String),
}
