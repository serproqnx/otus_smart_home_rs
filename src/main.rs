// use std::fmt;
use std::collections::HashMap;
// use std::fmt::Display;

// #[derive(Debug, Clone)]
struct Home {
    name: &'static str,
    rooms: HashMap<&'static str, Room>,
}

// #[derive(Debug)]
// #[derive(Debug, Clone)]
struct Room {
    name: &'static str,
    devices: HashMap<&'static str, Box<dyn SmartHomeUnit>>,
}

// #[derive(Debug)]
#[derive(Debug, Clone)]
struct Socket {
    name: &'static str,
    about: &'static str,
    on_status: bool,
    current_power_consumption: i32,
}

#[derive(Debug, Clone)]
// #[derive(Debug)]
struct Thermometer {
    name: &'static str,
    about: &'static str,
    on_status: bool,
    current_temperature: i32,
}

trait SmartHomeUnit {
    // fn new(name: &'static str) -> Self;
    fn get_name(&self) -> &'static str;
    fn on_status(&self) -> bool;
    fn turn_on_off(&mut self);
    fn get_about(&self) -> &'static str;
    fn is_on(&self) -> &'static str;
    fn get_device_report(&self);
}

impl Home {
    fn new(name: &'static str) -> Home {
        Home {
            name,
            rooms: HashMap::new(),
        }
    }

    fn add_room(&mut self, name: &'static str) {
        self.rooms.insert(name, Room::new(name));
    }

    fn get_rooms_list(&self) {
        for (_key, val) in self.rooms.iter() {
            println!("{}", val.name);
        }
    }
}

impl Room {
    fn new(name: &'static str) -> Room {
        Room {
            name,
            devices: HashMap::new(),
        }
    }

    fn add_device_socket(&mut self, name: &'static str) {
        let new_socket = Thermometer {
            name,
            on_status: false,
            about: "about Thermometer",
            current_temperature: 20,
        };
        self.devices.insert(name, Box::new(new_socket));
    }

    fn add_device_thermometer(&mut self, name: &'static str) {
        let new_therm = Socket {
            name,
            on_status: false,
            about: "about Socket",
            current_power_consumption: 0,
        };

        self.devices.insert(name, Box::new(new_therm));
    }

    fn get_devices_list(&self) {
        println!("\nСписок устройств в помещении {} :", self.name);
        for (_key, device) in self.devices.iter() {
            println!("{}", device.get_name());
            // device.get_device_report();
        }
    }
}

impl Socket {
    fn get_current_power_consumption(&self) -> i32 {
        self.current_power_consumption
    }
}

impl Thermometer {
    fn get_current_temperature(&self) -> i32 {
        self.current_temperature
    }
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

fn get_report(device: &Box<dyn SmartHomeUnit>) {
    device.get_device_report();
}

fn main() {
    // Библиотека предоставляет структуру дома в комнатах которого расположены устройства.
    // - Дом имеет название и содержит несколько помещений.
    println!("Дом имеет название и содержит несколько помещений.");
    let mut home_1: Home = Home::new("Home1");

    home_1.add_room("bedroom1");
    home_1.add_room("kitchen1");

    println!("{}", home_1.name);

    // - Библиотека позволяет запросить список помещений в доме.
    println!("\nСписок помещений: ");
    home_1.get_rooms_list();

    // - Помещение имеет уникальное название
    println!("\nУникальное название помещения: ");
    println!("{}", home_1.rooms["bedroom1"].name);

    //и содержит названия нескольких устройств.
    println!("\nНазвания нескольких устройств: ");
    home_1
        .rooms
        .get_mut("kitchen1")
        .unwrap()
        .add_device_socket("Socket2");
    home_1
        .rooms
        .get_mut("kitchen1")
        .unwrap()
        .add_device_socket("Socket3");

    home_1
        .rooms
        .get_mut("kitchen1")
        .unwrap()
        .add_device_thermometer("Thermometer2");
    home_1
        .rooms
        .get_mut("kitchen1")
        .unwrap()
        .add_device_thermometer("Thermometer3");

    // - Устройство имеет уникальное в рамках помещения имя.
    println!("\nУникальное имя устройства:");
    // println!("{}", home_1.rooms["kitchen1"].devices["Socket2"].);

    // - Библиотека позволяет получать список устройств в помещении.
    // println!("\nСписок устройств в помещении");
    home_1.rooms["kitchen1"].get_devices_list();
    home_1.rooms["bedroom1"].get_devices_list();

    // - Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
    //     Эта функция принимает в качестве аргумента обобщённый тип, позволяющий
    //     получить текстовую информацию о состоянии устройства, для включения в отчёт.
    //     Эта информация должна предоставляться для каждого устройства на основе
    //     данных о положении устройства в доме: имени комнаты и имени устройства.
    //     Если устройство не найдено в источнике информации, то вместо текста о
    //     состоянии вернуть сообщение об ошибке.

    println!("\nREPORT: ");

    get_report(&home_1.rooms["kitchen1"].devices["Socket2"]);
    get_report(&home_1.rooms["kitchen1"].devices["Thermometer3"]);

    // - Привести пример типа, предоставляющего текстовую информацию об устройствах
    //     в доме для составления отчёта. Шаблон для описания сущностей библиотеки:
    //     https://gist.github.com/76dff7177f19ff7e802b1e121865afe4
}
