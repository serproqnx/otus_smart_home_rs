use std::fmt;
use std::collections::HashMap;

struct Home {
    name: &'static str,
    rooms: HashMap<&'static str, Room>,
}

#[derive(Debug)]
struct Room {
    name: &'static str,
    devices: HashMap<&'static str, SmartHomeUnitType>,
}

#[derive(Debug)]
struct Socket {
    name: &'static str,
    about: &'static str,
    on_status: bool,
    current_power_consumption: i32,
}

#[derive(Debug)]
struct Thermometer {
    name: &'static str,
    about: &'static str,
    on_status: bool,
    current_temperature: i32,
}

#[derive(Debug)]
enum SmartHomeUnitType {
    Socket(Socket),
    Thermometer(Thermometer),
}

trait SmartHomeUnit {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn on_status(&self) -> bool;
    fn turn_on_off(&mut self);
    fn get_about(&self) -> &'static str;
    fn is_on(&self) -> &'static str;
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
        self.devices
            .insert(name, SmartHomeUnitType::Socket(SmartHomeUnit::new(name)));
    }

    fn add_device_thermometer(&mut self, name: &'static str) {
        self.devices.insert(
            name,
            SmartHomeUnitType::Thermometer(SmartHomeUnit::new(name)),
        );
    }

    fn get_devices_list(&self) {
        // dbg!(self.devices)
        for (_key, device) in self.devices.iter() {
            match &device {
                SmartHomeUnitType::Socket(Socket { name, .. }) => {
                    println!("{:?}", name)
                }

                SmartHomeUnitType::Thermometer(Thermometer { name, .. }) => {
                    println!("{:?}", name)
                }
            }
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

// fn get_report(device: &SmartHomeUnitType) {
fn get_report<T: fmt::Debug>(device: T) {
    println!("\nREPORT: \n");
		println!("{:?}", device);

    // match &device { 
		// 	Some(Socket { 
		// 		name, 
		// 		about, 
		// 		on_status, 
		// 		current_power_consumption 
		// 	}) => {
    //         println!(
    //             "Name: {}\nAbout: {}\nIs on: {}\nConsumption: {}",
    //             name, about, on_status, current_power_consumption,
    //         )
		// 	}
		// }

    match &device {
			
        SmartHomeUnitType::Socket(Socket {
            name,
            about,
            on_status,
            current_power_consumption,
        }) => {
            println!(
                "Name: {}\nAbout: {}\nIs on: {}\nConsumption: {}",
                name, about, on_status, current_power_consumption,
            )
        }

        SmartHomeUnitType::Thermometer(Thermometer {
            name,
            about,
            on_status,
            current_temperature,
        }) => {
            println!(
                "Name: {}\nAbout: {}\nIs on: {}\nTemperature: {}",
                name, about, on_status, current_temperature,
            )
        }
    }
}

// impl Thermometer {}

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
    println!("\nСписок устройств в помещении");
    home_1.rooms["kitchen1"].get_devices_list();

    // - Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
    //     Эта функция принимает в качестве аргумента обобщённый тип, позволяющий
    //     получить текстовую информацию о состоянии устройства, для включения в отчёт.
    //     Эта информация должна предоставляться для каждого устройства на основе
    //     данных о положении устройства в доме: имени комнаты и имени устройства.
    //     Если устройство не найдено в источнике информации, то вместо текста о
    //     состоянии вернуть сообщение об ошибке.

    get_report(&home_1.rooms["kitchen1"].devices["Socket2"]);
    get_report(&home_1.rooms["kitchen1"].devices["Socket3"]);
    get_report(&home_1.rooms["kitchen1"].devices["Thermometer3"]);
    // get_report(&home_1.rooms["kitchen1"]);

    // - Привести пример типа, предоставляющего текстовую информацию об устройствах
    //     в доме для составления отчёта. Шаблон для описания сущностей библиотеки:
    //     https://gist.github.com/76dff7177f19ff7e802b1e121865afe4

    // let test = Home1.rooms["bedroom1"].name;
    // println!("{}", test);
    // Home1.get_room_list();

    // let mut socket1: Socket = SmartHomeUnit::new("Socket1");
    // dbg!(socket1);
    // socket1.get_about();
    // socket1.turn_on_off();
    // socket1.get_current_power_consumption();

    // let mut thermometer1: Thermometer = SmartHomeUnit::new("Thermometer1");
    // thermometer1.get_about();
    // thermometer1.turn_on_off();
    // thermometer1.get_current_temperature();
}
