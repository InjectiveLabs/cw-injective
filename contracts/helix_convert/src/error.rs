use cosmwasm_std::StdError;
use thiserror::Error;
use injective_math::FPDecimal;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
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
//
// impl From<StdError> for ContractError {
//     fn from(err: StdError) -> Self {
//         ContractError::Std(err)
//     }
// }
//
// trait ContractErrorMapper<R> {
//     fn mapToContractError(&self) -> Result<R, ContractError>;
// }
//
// impl ContractErrorMapper<R> for Result<R, StdError> {
//
//     fn mapToContractError(&self) -> Result<R, ContractError> {
//          self.map_err(|e| ContractError::Std(e))
//     }
// }

