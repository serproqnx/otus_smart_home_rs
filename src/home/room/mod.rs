use std::collections::HashMap;

pub mod unit;

use crate::home::room::unit::{socket::Socket, thermometer::Thermometer, SmartHomeUnit};

pub struct Room {
  pub name: &'static str,
  pub devices: HashMap<&'static str, Box<dyn SmartHomeUnit>>,
}

impl Room {
  pub fn new(name: &'static str) -> Room {
    Room {
      name,
      devices: HashMap::new(),
    }
  }

  pub fn add_device_socket(&mut self, name: &'static str) {
    let new_socket = Thermometer {
      name,
      on_status: false,
      about: "about Thermometer",
      current_temperature: 20,
    };
    self.devices.insert(name, Box::new(new_socket));
  }

  pub fn add_device_thermometer(&mut self, name: &'static str) {
    let new_therm = Socket {
      name,
      on_status: false,
      about: "about Socket",
      current_power_consumption: 0,
    };

    self.devices.insert(name, Box::new(new_therm));
  }

  pub fn get_devices_list(&self) {
    println!("\nСписок устройств в помещении {} :", self.name);
    for (_key, device) in self.devices.iter() {
      println!("{}", device.get_name());
      // device.get_device_report();
    }
  }
}
