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
