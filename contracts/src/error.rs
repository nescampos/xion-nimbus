use cosmwasm_std::{StdError};
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Insufficient funds sent for subscription")]
    InsufficientFunds,
    #[error("No funds sent for subscription")]
    NoFundsSent,
    #[error("Unauthorized: Not a subscriber")]
    Unauthorized,
}
