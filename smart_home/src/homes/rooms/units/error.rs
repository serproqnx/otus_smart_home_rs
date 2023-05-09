use thiserror::Error;
use std::io;

pub type ConnectResult<T> = Result<T, NetError>;

#[derive(Error, Debug)]
pub enum NetError {
  #[error("Net Error: {0}")]
  Turnonsocket(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum SmartHomeError {
    #[error("Failed to perform an operation: {0}")]
    OperationError(String),
    // Add other error variants as needed
    #[error("Custom error: {0}")]
    CustomError(String),
}


