use cosmwasm_std::StdError;
use injective_math::FPDecimal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error: {val:?}")]
    CustomError { val: String },

    #[error("Failure response from submsg: {0}")]
    SubMsgFailure(String),

    #[error("Unrecognised reply id: {0}")]
    UnrecognisedReply(u64),

    #[error("Invalid reply from sub-message {id}, {err}")]
    ReplyParseFailure { id: u64, err: String },

    #[error("Min expected swap amount ({0}) not reached")]
    MinExpectedSwapAmountNotReached(FPDecimal),
}
