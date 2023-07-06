use crate::homes::rooms::{room::Room, units::unit::SmartHomeUnit};

use std::collections::HashMap;

pub struct Home {
  pub name: &'static str,
  pub rooms: HashMap<String, Room>,
}

impl Home {
  pub fn new(name: &'static str) -> Home {
    Home {
      name,
      rooms: HashMap::new(),
    }
  }

  pub fn add_room(&mut self, name: String) -> String {
    let room_name = name.clone();
    let return_name = name.clone();
    let key_name = name.clone();

    self.rooms.insert(key_name, Room::new(room_name));
    return_name
  }

  pub fn del_room(&mut self, name: String) -> String {
    self.rooms.remove(&name);
    name
  }

  pub fn get_room(&self, name: &'static str) -> Option<&Room> {
    Some(&self.rooms[name])
  }

  pub fn get_rooms_list(&self) -> &HashMap<String, Room> {
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

    home1.add_room("Room1".to_string());
    home1.add_room("Room2".to_string());

    home1.add_room("delete".to_string());
    assert_eq!(home1.rooms["delete"].name, "delete");
    home1.del_room("delete".to_string());
    assert!(!home1.rooms.contains_key("delete"));

    assert_eq!(home1.rooms["Room1"].name, "Room1");
    assert!(home1.rooms["Room1"].devices.is_empty());

    let _hashmap = home1.get_rooms_list();

    // for (key, device) in hashmap.iter() {
    //   match *key {
    //     "Room1" => assert_eq!(device.name, "Room1"),
    //     "Room2" => assert_eq!(device.name, "Room2"),
    //     _ => panic!("Этого тут быть не должно: {}", device.name),
    //   }
    // }
  }
}
