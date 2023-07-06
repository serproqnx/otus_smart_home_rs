use std::net::SocketAddrV4;

#[derive(Debug, Clone)]

pub struct Thermometer {
  pub name: String,
  pub about: String,
  pub on_status: bool,
  pub current_temperature: i32,
  pub ip: SocketAddrV4,
}

impl Thermometer {
  pub fn get_current_temperature(&self) -> i32 {
    self.current_temperature
  }
}

#[cfg(test)]
mod tests {

  use super::Thermometer;
  use std::net::{Ipv4Addr, SocketAddrV4};

  #[test]
  fn create_thermometer() {
    let thermometer1: Thermometer = Thermometer {
      name: "1".to_string(),
      about: "1".to_string(),
      on_status: true,
      current_temperature: 21,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };

    assert_eq!(thermometer1.name, "1");
    assert_eq!(thermometer1.about, "1");
    assert!(thermometer1.on_status);
    assert_eq!(thermometer1.current_temperature, 21);
    assert_eq!(thermometer1.get_current_temperature(), 21);
  }
}
