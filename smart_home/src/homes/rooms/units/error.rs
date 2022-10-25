use thiserror::Error;
use std::io;

pub type ConnectResult<T> = Result<T, NetError>;

#[derive(Error, Debug)]
pub enum NetError {
  #[error("Net Error: {0}")]
  Turnonsocket(#[from] io::Error),
}
