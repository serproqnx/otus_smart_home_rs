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
    let mut socket1: Socket = SmartHomeUnit::new("Socket1");
    socket1.get_about();
    socket1.turn_on_off();
    socket1.get_current_power_consumption();

    let mut thermometer1: Thermometer = SmartHomeUnit::new("Thermomenter1");
    thermometer1.get_about();
    thermometer1.turn_on_off();
    thermometer1.get_current_temperature();
}
