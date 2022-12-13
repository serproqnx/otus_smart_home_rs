use smart_home_lib::homes::{home::Home, rooms::units::unit_builder::UnitBuilder};

use smart_home_lib::homes::rooms::units::unit_visitor::{
  GetReportVisitor, GetAboutVisitor, TurnOnVisitor 
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
  let mut home_1: Home = Home::new("Home1");
  home_1.add_room("kitchen1");

  home_1
    .rooms
    .get_mut("kitchen1")
      .unwrap()
    .add_device( 
      UnitBuilder::new()
        .unit_type("socket")
        .name("Socket_builder")
        .about("about_socket_builder")
        .build()
    );

  home_1
    .rooms
    .get_mut("kitchen1")
      .unwrap()
    .add_device( 
      UnitBuilder::new()
        .unit_type("thermometer")
        .name("Thermometer_builder")
        .about("about_thermometer_builder")
        .build()
    );

  home_1
    .rooms
    .get_mut("kitchen1").unwrap()
    .devices
    .get_mut("Socket_builder").unwrap()
    .accept(&GetReportVisitor);

  home_1
    .rooms
    .get_mut("kitchen1").unwrap()
    .devices
    .get_mut("Thermometer_builder").unwrap()
    .accept(&GetAboutVisitor);

  home_1
    .rooms
    .get_mut("kitchen1").unwrap()
    .devices
    .get_mut("Thermometer_builder").unwrap()
    .accept(&TurnOnVisitor);

home_1
  .rooms["kitchen1"]
  .devices["Socket_builder"]
  .send_cmd("turnOff").await?;
////        .unwrap();
//  
//  home_1
//    .rooms["kitchen1"]
//    .devices["Socket_builder"]
//    .turn_on().await?;
////        .unwrap();
//
//  home_1
//    .rooms["kitchen1"]
//    .devices["Socket_builder"]
//    .get_report().await?;
// //       .unwrap();
//
//  home_1
//    .rooms["kitchen1"]
//    .devices["Socket_builder"]
//    .turn_off().await?;
//  //      .unwrap();
//
//  home_1
//    .rooms["kitchen1"]
//    .devices["Socket_builder"]
//    .get_report().await?;
//
//
//  home_1
//    .rooms["kitchen1"]
//    .devices["Thermometer_builder"]
//    .send_cmd("TEST").await?;
        //.unwrap();
 Ok(())
}
