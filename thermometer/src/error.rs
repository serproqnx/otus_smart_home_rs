use thiserror::Error;

pub type ThermoErr<T> = Result<T, ThermoError>;

#[derive(Error, Debug)]
pub enum ThermoError {
  // #[error("Device error: {0}")]
  // DeviceError(String),

  // #[error("TcpStream error: {0}")]
  // TcpStreamError(#[source] std::io::Error),
  #[error("Connection fail: {0}")]
  UdpConnectionFailError(#[source] std::io::Error),

  #[error("Didn't recieve data: {0}")]
  UdpRecieveDataError(#[source] std::io::Error),

  #[error("Couldn't send data: {0}")]
  UdpSendDataError(#[source] std::io::Error),

  #[error("Couldn't bind to adress: {0}")]
  UdpAdressBindError(#[source] std::io::Error),
  // #[error("UdpSocketError error: {0}")]
  // UdpSocketError(#[source] std::io::Error),
}
