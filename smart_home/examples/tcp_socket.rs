use async_trait::async_trait;
use smart_home_lib::homes::home::Home;

#[async_trait]
async fn main() {
  let mut home_1: Home = Home::new("Home1");
  home_1.add_room("kitchen1");

  home_1
    .rooms
    .get_mut("kitchen1")
        .unwrap()
    .add_device_socket("s1");

  println!("TurnOn");
  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .send_cmd("turnO2n");
//        .unwrap();
  
  println!("TurnOn 2");
  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .turn_on();
//        .unwrap();

  println!("get report");
  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .get_report();
 //       .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .turn_off();
  //      .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .get_report();
   //     .unwrap();
}
