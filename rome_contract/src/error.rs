use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("No Text Allowed")]
    TooMuchText {},

    #[error("Deleted post content must be empty.")]
    DeletedPost {},

    #[error("Deleted post content must be empty.")]
    NotEnoughFunds { needed: String, received: String },
}
