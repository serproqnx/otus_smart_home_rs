use thiserror::Error;

pub type SocketErr<T> = Result<T, SocketError>;

#[derive(Error, Debug)]
pub enum SocketError {
  #[error("Read exact error: {0}")]
  TcpReadError(#[source] std::io::Error),

  #[error("Write all error: {0}")]
  TcpWriteError(#[source] std::io::Error),

  #[error("TCP error: {0}")]
  TcpError(#[from] std::io::Error),
}
