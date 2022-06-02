use std::collections::HashMap;

struct Home<'a, T> {
  name: &'static str,
  rooms: HashMap<&'a str, Room<'a, T>>,
}

struct Room<'a, T> {
  name: &'a str,
  devices: HashMap<&'a str, T>, 
}

struct Socket {
    name: &'static str,
    about: &'static str,
    on_status: bool,
    current_power_consumption: i32,
}

struct Thermometer {
    name: &'static str,
    about: &'static str,
    on_status: bool,
    current_temperature: i32,
}

trait SmartHomeUnit {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn on_status(&self) -> bool;
    fn turn_on_off(&mut self);
    fn get_about(&self) -> &'static str;
    fn is_on(&self) -> &'static str;
}

impl<'a, T> Home<'a, T> {
    fn new (name: &'static str) -> Home<'a, T> {
        Home {
            name,
            rooms: HashMap::new(),
        }
    }
    
    fn add_room(&mut self, name: &'static str) {
        self.rooms.insert(name, Room { name, HashMap::new() } );
    }

    fn get_rooms_list(&self) {
        for (_key, val) in self.rooms.iter()  {
            println!("{}", val.name);
        }
    }
}

impl<'a, T> Room<'a, T> {
    fn new (name: &'static str) -> Room<'a, T> {
        Room { 
            name, 
            devices: HashMap::new(),
        }
    }
}

impl Socket {
    fn get_current_power_consumption(&self) -> i32 {
        println!(
            "Current power consumption of {} is {}",
            self.name, self.current_power_consumption,
        );

        self.current_power_consumption
    }
}

impl Thermometer {
    fn get_current_temperature(&self) -> i32 {
        println!(
            "Current temperature of {} is {}",
            self.name, self.current_temperature,
        );

        self.current_temperature
    }
}

impl SmartHomeUnit for Socket {
    fn new(name: &'static str) -> Socket {
        Socket {
            name,
            on_status: false,
            about: "about Socket",
            current_power_consumption: 0,
        }
    }

    fn get_about(&self) -> &'static str {
        println!("{}", self.about);
        self.about
    }

    fn is_on(&self) -> &'static str {
        (if self.on_status() { "ON" } else { "OFF" }) as _
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn on_status(&self) -> bool {
        self.on_status
    }

    fn turn_on_off(&mut self) {
        self.on_status = !&self.on_status;
        println!("{} turned {}", self.name, self.is_on());
    }
}

impl SmartHomeUnit for Thermometer {
    fn new(name: &'static str) -> Thermometer {
        Thermometer {
            name,
            on_status: false,
            about: "about Thermometer",
            current_temperature: 20,
        }
    }

    fn get_about(&self) -> &'static str {
        println!("{}", self.about);
        self.about
    }

    fn is_on(&self) -> &'static str {
        (if self.on_status() { "ON" } else { "OFF" }) as _
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn on_status(&self) -> bool {
        self.on_status
    }

    fn turn_on_off(&mut self) {
        self.on_status = !&self.on_status;
        println!("{} turned {}", self.name, self.is_on());
    }
}

// impl Thermometer {}

fn main() {
    let mut home_1: Home = Home::new("Home1");
    println!("{}", home_1.name);
    
    home_1.add_room("bedroom1");
    home_1.add_room("kitchen1");

    home_1.get_rooms_list();

    // let test = Home1.rooms["bedroom1"].name;
    // println!("{}", test);
    // Home1.get_room_list();

    let mut socket1: Socket = SmartHomeUnit::new("Socket1");
    socket1.get_about();
    socket1.turn_on_off();
    socket1.get_current_power_consumption();

    let mut thermometer1: Thermometer = SmartHomeUnit::new("Thermomenter1");
    thermometer1.get_about();
    thermometer1.turn_on_off();
    thermometer1.get_current_temperature();
}
