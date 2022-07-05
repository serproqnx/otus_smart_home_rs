use std::collections::HashMap;

use crate::home::room::unit::{socket::Socket, thermometer::Thermometer, unit::SmartHomeUnit};

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

  pub fn add_device_thermometer(&mut self, name: &'static str) {
    let new_thermometer = Thermometer {
      name,
      on_status: false,
      about: "about Thermometer",
      current_temperature: 20,
    };
    self.devices.insert(name, Box::new(new_thermometer));
  }

  pub fn add_device_socket(&mut self, name: &'static str) {
    let new_socket = Socket {
      name,
      on_status: false,
      about: "about Socket",
      current_power_consumption: 0,
    };

    self.devices.insert(name, Box::new(new_socket));
  }

  pub fn get_devices_list(&self) -> &HashMap<&str, Box<dyn SmartHomeUnit>> {
    println!("\nСписок устройств в помещении {} :", self.name);

    for (_key, device) in self.devices.iter() {
      println!("{}", device.get_name());
    }

    &self.devices
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_room() {
    let mut new_room = Room {
      name: "Room1",
      devices: HashMap::new(),
    };

    assert_eq!(new_room.name, "Room1");
    assert_eq!(new_room.devices.is_empty(), true);

    new_room.add_device_socket("Socket1");
    let socket1 = &new_room.devices["Socket1"];

    assert_eq!(socket1.get_name(), "Socket1");
    assert_eq!(socket1.get_about(), "about Socket");
    assert_eq!(socket1.get_bool_on_status(), false);
    assert_eq!(socket1.get_on_status(), "OFF");

    assert_eq!(
      socket1.get_device_report(),
      "\nName: Socket1\nAbout: about Socket\nPower: OFF\nCurrent power consumption: 0\n",
    );

    new_room.add_device_thermometer("Thermometer1");
    let thermometer1 = &new_room.devices["Thermometer1"];

    assert_eq!(thermometer1.get_name(), "Thermometer1");
    assert_eq!(thermometer1.get_about(), "about Thermometer");
    assert_eq!(thermometer1.get_bool_on_status(), false);
    assert_eq!(thermometer1.get_on_status(), "OFF");

    assert_eq!(
      thermometer1.get_device_report(),
      "\nName: Thermometer1\nAbout: about Thermometer\nPower: OFF\nTemperature: 20\n",
    );

    let hashmap = new_room.get_devices_list();

    for (key, device) in hashmap.iter() {
      match key {
        &"Socket1" => assert_eq!(device.get_name(), "Socket1"),
        &"Thermometer1" => assert_eq!(device.get_name(), "Thermometer1"),
        _ => assert!(false),
      }
    }
  }
}
