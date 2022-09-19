use std::net::{SocketAddrV4, Ipv4Addr};

use super::{unit::SmartHomeUnit, socket::Socket, thermometer::Thermometer};



pub struct UnitBuilder {
  pub unit_type: &'static str,
  pub name: &'static str,
  pub about: &'static str,
  pub on_status: bool,
  pub ip: SocketAddrV4,
}

impl Default for UnitBuilder {
    fn default() -> Self {
      UnitBuilder {
        unit_type: "BUILDER_TYPE",
        name: "BUILDER_NAME",
        about: "BUILDER_ABOUT",
        on_status: false,
        ip: SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 8181) 
        }
    }
}

impl UnitBuilder {
    pub fn new() -> UnitBuilder {
      UnitBuilder {
        unit_type: "socket",
        name: "BUILDER_NAME",
        about: "BUILDER_ABOUT",
        on_status: false,
        ip: SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 8181) 
      }
    }


    pub fn unit_type(mut self, unit_type: &'static str) -> UnitBuilder {
      self.unit_type = unit_type;
      self
    }

    pub fn name(mut self, name: &'static str) -> UnitBuilder {
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

    pub fn build(self) -> Box<dyn SmartHomeUnit> {
        match self.unit_type {
          "socket" => Box::new(Socket {
            name: self.name,
            about: self.about,
            on_status: self.on_status,
            current_power_consumption: 0,
            ip: SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 8181),
          }),
          "thermometer" => Box::new(Thermometer {
            name: self.name,
            about: "1",
            on_status: true,
            current_temperature: 1,
            ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
          }),
          _ => panic!("wrong unit type"),
        }
    }
}
