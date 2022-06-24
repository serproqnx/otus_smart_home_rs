mod home;
use home::{room::unit::SmartHomeUnit, room::Room, Home};

fn get_report(device: &dyn SmartHomeUnit) {
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

  get_report(home_1.rooms["kitchen1"].devices["Socket2"].as_ref());
  get_report(home_1.rooms["kitchen1"].devices["Thermometer3"].as_ref());

  // - Привести пример типа, предоставляющего текстовую информацию об устройствах
  //     в доме для составления отчёта. Шаблон для описания сущностей библиотеки:
  //     https://gist.github.com/76dff7177f19ff7e802b1e121865afe4
}
