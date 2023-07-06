use std::net::{Ipv4Addr, SocketAddrV4};

use super::{socket::Socket, thermometer::Thermometer, unit::SmartHomeUnit};

pub struct UnitBuilder {
  pub unit_type: String,
  pub name: String,
  pub about: &'static str,
  pub on_status: bool,
  pub ip: SocketAddrV4,
}

impl Default for UnitBuilder {
  fn default() -> Self {
    UnitBuilder {
      unit_type: "BUILDER_TYPE".to_string(),
      name: "BUILDER_NAME".to_string(),
      about: "BUILDER_ABOUT",
      on_status: false,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    }
  }
}

impl UnitBuilder {
  pub fn new() -> UnitBuilder {
    UnitBuilder {
      unit_type: "socket".to_string(),
      name: "BUILDER_NAME".to_string(),
      about: "BUILDER_ABOUT",
      on_status: false,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    }
  }

  pub fn unit_type(mut self, unit_type: String) -> UnitBuilder {
    self.unit_type = unit_type;
    self
  }

  pub fn name(mut self, name: String) -> UnitBuilder {
    self.name = name;
    self
  }

  pub fn about(mut self, about: &'static str) -> UnitBuilder {
    self.about = about;
    self
  }

  pub fn on_status(mut self, on_status: bool) -> UnitBuilder {
    self.on_status = on_status;
    self
  }

  pub fn ip(mut self, ip: SocketAddrV4) -> UnitBuilder {
    self.ip = ip;
    self
  }

  pub fn build(self) -> Box<dyn SmartHomeUnit + Send + Sync> {
    match self.unit_type.as_str() {
      "socket" => Box::new(Socket {
        name: self.name,
        about: self.about.to_string(),
        on_status: self.on_status,
        current_power_consumption: 0,
        ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
      }),
      "thermometer" => Box::new(Thermometer {
        name: self.name,
        about: "1".to_string(),
        on_status: true,
        current_temperature: 1,
        ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
      }),
      _ => panic!("wrong unit type"),
    }
  }
}
