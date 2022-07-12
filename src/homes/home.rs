// pub mod room;

use crate::homes::rooms::{room::Room, units::unit::SmartHomeUnit};

use std::collections::HashMap;

pub struct Home {
  pub name: &'static str,
  pub rooms: HashMap<&'static str, Room>,
}

impl Home {
  pub fn new(name: &'static str) -> Home {
    Home {
      name,
      rooms: HashMap::new(),
    }
  }

  pub fn add_room(&mut self, name: &'static str) {
    self.rooms.insert(name, Room::new(name));
  }

  pub fn get_rooms_list(&self) -> &HashMap<&str, Room> {
    for (_key, val) in self.rooms.iter() {
      println!("{}", val.name);
    }
    &self.rooms
  }

  pub fn get_report(&self, device: &dyn SmartHomeUnit) -> String {
    match device.get_device_report() {
      None => {
        println!("None");
        "None".to_string()
      }
      Some(x) => x,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_home() {
    let mut home1 = Home {
      name: "Home1",
      rooms: HashMap::new(),
    };

    assert_eq!(home1.name, "Home1");
    assert!(home1.rooms.is_empty());

    home1.add_room("Room1");
    home1.add_room("Room2");

    assert_eq!(home1.rooms["Room1"].name, "Room1");
    assert!(home1.rooms["Room1"].devices.is_empty());

    let hashmap = home1.get_rooms_list();

    for (key, device) in hashmap.iter() {
      match *key {
        "Room1" => assert_eq!(device.name, "Room1"),
        "Room2" => assert_eq!(device.name, "Room2"),
        _ => assert_eq!(true, false),
      }
    }
  }
}
