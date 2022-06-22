pub mod socket;
pub mod thermometer;

use crate::home::room::smarthomeunit::{socket::Socket, thermometer::Thermometer};

pub trait SmartHomeUnit {
    // fn new(name: &'static str) -> Self;
    fn get_name(&self) -> &'static str;
    fn on_status(&self) -> bool;
    fn turn_on_off(&mut self);
    fn get_about(&self) -> &'static str;
    fn is_on(&self) -> &'static str;
    fn get_device_report(&self);
}

impl SmartHomeUnit for Socket {
    fn get_device_report(&self) {
        println!(
            "\nName: {}\nAbout: {}\nPower: {}\nCurrent power consumption: {}\n",
            self.get_name(),
            self.get_about(),
            self.is_on(),
            self.get_current_power_consumption(),
        );
    }

    fn get_about(&self) -> &'static str {
        self.about
    }

    fn is_on(&self) -> &'static str {
        (if self.on_status() { "ON" } else { "OFF" }) as _
    }

    fn get_name(&self) -> &'static str {
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
    fn get_device_report(&self) {
        println!(
            "\nName: {}\nAbout: {}\nPower: {}\nTemperature: {}\n",
            self.get_name(),
            self.get_about(),
            self.is_on(),
            self.get_current_temperature(),
        );
    }

    fn get_about(&self) -> &'static str {
        // println!("{}", self.about);
        self.about
    }

    fn is_on(&self) -> &'static str {
        (if self.on_status() { "ON" } else { "OFF" }) as _
    }

    fn get_name(&self) -> &'static str {
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
