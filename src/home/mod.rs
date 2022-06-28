pub mod room;

use crate::Room;
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

  pub fn get_rooms_list(&self) {
    for (_key, val) in self.rooms.iter() {
      println!("{}", val.name);
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
    assert_eq!(home1.rooms.is_empty(), true);

    home1.add_room("Room1");
    home1.add_room("Room2");

    assert_eq!(home1.rooms["Room1"].name, "Room1");
    assert_eq!(home1.rooms["Room1"].devices.is_empty(), true);
  }
}
