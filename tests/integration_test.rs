use smart_home_lib::homes::{
  home::Home,
  rooms::room::Room,
  // room::unit::unit::SmartHomeUnit
};

#[test]

fn it_works() {
  let mut home_1: Home = Home::new("Home1");

  assert_eq!(home_1.name, "Home1");

  home_1.add_room("bedroom1");
  home_1.add_room("kitchen1");

  assert_eq!(home_1.rooms["bedroom1"].name, "bedroom1");
  assert_eq!(home_1.rooms["kitchen1"].name, "kitchen1");

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

  // - Библиотека позволяет запросить список помещений в доме.
  // - Помещение имеет уникальное название
  //	и содержит названия нескольких устройств.
  // - Устройство имеет уникальное в рамках помещения имя.
  // - Библиотека позволяет получать список устройств в помещении.
  // println!("\nСписок устройств в помещении");

  for (_key, room) in home_1.get_rooms_list().iter() {
    match room.name {
      "bedroom1" => {
        assert_eq!(room.name, "bedroom1");
        check_units_in_the_room(room);
      }
      "kitchen1" => {
        assert_eq!(room.name, "kitchen1");
        check_units_in_the_room(room);
      }
      _ => panic!(),
    }
  }

  fn check_units_in_the_room(room: &Room) {
    for (_key, unit) in room.devices.iter() {
      let u = unit.get_name();

      match u {
        "Socket2" => {
          assert_eq!(u, "Socket2");
        }
        "Socket3" => {
          assert_eq!(u, "Socket3");
        }
        "Thermometer2" => {
          assert_eq!(u, "Thermometer2");
        }
        "Thermometer3" => {
          assert_eq!(u, "Thermometer3");
        }
        _ => panic!(),
      }
    }
  }

  // Библиотека позволяет добавлять и удалять устройства
    
  home_1
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .add_device_thermometer("Delition_test");
 
  let deletion_result = home_1
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .del_device("Deletion_test")
    .unwrap()
    .get_name();
    
  assert_eq!(deletion_result, "Deletion_test");
  

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

  let report1 = home_1.get_report(home_1.rooms["kitchen1"].devices["Socket2"].as_ref());

  assert_eq!(
    report1,
    "\nName: Socket2\nAbout: about Socket\nPower: OFF\nCurrent power consumption: 0\n",
  );

  let report2 = home_1.get_report(home_1.rooms["kitchen1"].devices["Thermometer3"].as_ref());

  assert_eq!(
    report2,
    "\nName: Thermometer3\nAbout: about Thermometer\nPower: OFF\nTemperature: 20\n",
  );
}
