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

    #[error("The IPFS link must be with alxandria's dedicated gateway: https://alxandria.infura-ipfs.io/ipfs/")]
    MustUseAlxandriaGateway {},

    #[error("The profile name {taken_profile_name} is already taken. Please choose another")]
    ProfileNameTaken { taken_profile_name: String },

    #[error("To prevent misattribution, profile names are immutably tied to wallet addresses.")]
    ProfileNameImmutable {},

    #[error("This post already exists. Please edit the existing post or change the title.")]
    PostAlreadyExists {},

    #[error("Denom not accepted: {denom}")]
    InvalidDenom { denom: String },
}
