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

    #[error("This wallet address has already registered a profile name.")]
    CanOnlyRegisterOneName {},

    #[error("Denom not accepted: {denom}")]
    InvalidDenom { denom: String},
}
