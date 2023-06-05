use thiserror::Error;

pub type SHResult<T> = Result<T, SmartHomeError>;

#[derive(Error, Debug)]
pub enum SmartHomeError {
  #[error("Device error: {0}")]
  DeviceError(String),

  #[error("TcpStream error: {0}")]
  TcpStreamError(#[source] std::io::Error),

  #[error("UdpSocketError error: {0}")]
  UdpSocketError(#[source] std::io::Error),
}
