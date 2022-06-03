use std::collections::HashMap;

struct Home {
  name: &'static str,
  rooms: HashMap<&'static str, Room>,
}

struct Room {
  name: &'static str,
  pub devices: HashMap<&'static str, Socket>, 
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

impl Home {
    fn new (name: &'static str) -> Home {
        Home {
            name,
            rooms: HashMap::new(),
        }
    }
    
    fn add_room(&mut self, name: &'static str) {
        self.rooms.insert(
            name,
            Room { 
                name, 
                devices: HashMap::new() 
            } 
        );
    }

    fn get_rooms_list(&self) {
        for (_key, val) in self.rooms.iter()  {
            println!("{}", val.name);
        }
    }
}

impl Room {
    fn new (name: &'static str) -> Room {
        Room { 
            name, 
            devices: HashMap::new(),
        }
    }
    
    fn add_device(&mut self, name: &'static str) {
        self.devices.insert(
            name,
            Socket { 
                name, 
                about: "about ",
                on_status: false,
                current_power_consumption: 0, 
            } 
        );
    }
    
    fn get_devices_list(&self) {
        for (_key, val) in self.devices.iter() {
            println!("{}", val.name);
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
    home_1.rooms["kitchen1"].add_device("Socket2");

    home_1.rooms["bedroom1"].get_devices_list();

    // - Устройство имеет уникальное в рамках помещения имя.

    // - Библиотека позволяет получать список устройств в помещении.
    // - Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома. 
    //     Эта функция принимает в качестве аргумента обобщённый тип, позволяющий 
    //     получить текстовую информацию о состоянии устройства, для включения в отчёт. 
    //     Эта информация должна предоставляться для каждого устройства на основе 
    //     данных о положении устройства в доме: имени комнаты и имени устройства. 
    //     Если устройство не найдено в источнике информации, то вместо текста о 
    //     состоянии вернуть сообщение об ошибке.
    // - Привести пример типа, предоставляющего текстовую информацию об устройствах 
    //     в доме для составления отчёта. Шаблон для описания сущностей библиотеки: 
    //     https://gist.github.com/76dff7177f19ff7e802b1e121865afe4

    


    // let test = Home1.rooms["bedroom1"].name;
    // println!("{}", test);
    // Home1.get_room_list();

    // let mut socket1: Socket = SmartHomeUnit::new("Socket1");
    // socket1.get_about();
    // socket1.turn_on_off();
    // socket1.get_current_power_consumption();

    // let mut thermometer1: Thermometer = SmartHomeUnit::new("Thermomenter1");
    // thermometer1.get_about();
    // thermometer1.turn_on_off();
    // thermometer1.get_current_temperature();
}
