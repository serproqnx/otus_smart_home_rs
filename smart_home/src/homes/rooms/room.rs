use std::{
  collections::HashMap,
  net::{Ipv4Addr, SocketAddrV4},
};

use crate::homes::rooms::units::{
  socket::Socket,
  thermometer::Thermometer,
  unit::SmartHomeUnit,
  //unit::UnitBuilder,
};
#[derive(Debug)]
pub struct Room {
  pub name: String,
  pub devices: HashMap<String, Box<dyn SmartHomeUnit + Send + Sync>>,
}

impl Room {
  pub fn new(name: String) -> Room {
    Room {
      name,
      devices: HashMap::new(),
    }
  }

  pub fn del_device(&mut self, name: String) -> Option<Box<dyn SmartHomeUnit + Send + Sync>> {
    self.devices.remove(&name)
  }

  pub fn add_device(&mut self, device: Box<dyn SmartHomeUnit + Send + Sync>) {
    self.devices.insert(device.get_name(), device);
  }

  pub fn add_device_thermometer(&mut self, name: String) {
    let new_thermometer = Thermometer {
      name: name.clone(),
      on_status: false,
      about: "about Thermometer".to_string(),
      current_temperature: 20,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8182),
    };
    self.devices.insert(name, Box::new(new_thermometer));
  }

  pub fn add_device_socket(&mut self, name: String) {
    let new_socket = Socket {
      name: name.clone(),
      on_status: false,
      about: "about Socket".to_string(),
      current_power_consumption: 0,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };

    self.devices.insert(name, Box::new(new_socket));
  }

  pub fn get_devices_list(&self) -> &HashMap<String, Box<dyn SmartHomeUnit + Send + Sync>> {
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
      name: "Room1".to_string(),
      devices: HashMap::new(),
    };

    assert_eq!(new_room.name, "Room1");
    assert!(new_room.devices.is_empty());

    new_room.add_device_socket("Socket1".to_string());
    let socket1 = &new_room.devices["Socket1"];

    assert_eq!(socket1.get_name(), "Socket1");
    assert_eq!(socket1.get_about(), "about Socket");
    assert!(!socket1.get_bool_on_status());
    assert_eq!(socket1.get_on_status(), "OFF");

    assert_eq!(
      socket1.get_device_report().unwrap(),
      "\nName: Socket1\nAbout: about Socket\nPower: OFF\nCurrent power consumption: 0\n",
    );

    new_room.add_device_thermometer("Thermometer1".to_string());
    let thermometer1 = &new_room.devices["Thermometer1"];

    assert_eq!(thermometer1.get_name(), "Thermometer1");
    assert_eq!(thermometer1.get_about(), "about Thermometer");
    assert!(!thermometer1.get_bool_on_status());
    assert_eq!(thermometer1.get_on_status(), "OFF");

    assert_eq!(
      thermometer1.get_device_report().unwrap(),
      "\nName: Thermometer1\nAbout: about Thermometer\nPower: OFF\nTemperature: 20\n",
    );

    let _hashmap = new_room.get_devices_list();

    // for (key, device) in hashmap.iter() {
    //   match *key {
    //     "Socket1" => assert_eq!(device.get_name(), "Socket1".to_string()),
    //     "Thermometer1" => assert_eq!(device.get_name(), "Thermometer1"),
    //     _ => panic!(),
    //   }
    // }
  }
}
