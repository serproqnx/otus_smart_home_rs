use smart_home_lib::homes::home::Home;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
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
    .send_cmd("turnOn").await?;
//        .unwrap();
  
  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .turn_on().await?;
//        .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .get_report().await?;
 //       .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .turn_off().await?;
  //      .unwrap();

  home_1
    .rooms["kitchen1"]
    .devices["s1"]
    .get_report().await?;

  home_1
    .rooms
    .get_mut("kitchen1")
        .unwrap()
    .add_device_thermometer("t1");

  home_1
    .rooms["kitchen1"]
    .devices["t1"]
    .send_cmd("TEST").await?;
        //.unwrap();
 Ok(())
}
