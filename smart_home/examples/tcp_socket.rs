use smart_home_lib::homes::home::Home;

fn main() {
  let mut home_1: Home = Home::new("Home1");
  home_1.add_room("kitchen1");

  home_1
    .rooms
    .get_mut("kitchen1")
        .unwrap()
    .add_device_socket("s1");

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .turn_on()
        .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .get_report()
        .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .turn_off()
        .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .get_report()
        .unwrap();
}
