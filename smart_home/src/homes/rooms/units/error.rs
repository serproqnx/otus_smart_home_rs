use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum NetError {
  #[error("Turn on socket!")]
  Turnonsocket(#[from] io::Error),
}
