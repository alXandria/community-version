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

    #[error("Only One Link Allowed")]
    OnlyOneLink {},

    #[error("Insufficient funds. Needed: {needed} Sent: {received}")]
    NotEnoughFunds { needed: String, received: String },
}
