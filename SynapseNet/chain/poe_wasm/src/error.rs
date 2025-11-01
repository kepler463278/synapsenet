//! PoE contract errors

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Epoch {epoch} already posted")]
    EpochAlreadyPosted { epoch: u64 },

    #[error("Insufficient signers: required {required}, provided {provided}")]
    InsufficientSigners { required: u32, provided: u32 },

    #[error("No rewards to claim")]
    NoRewardsToClaim {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid signature")]
    InvalidSignature {},
}
