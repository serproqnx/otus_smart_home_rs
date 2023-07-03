// use core::time;

use std::sync::{Arc, Mutex};

use smart_home_lib::homes::{
  home::Home,
  rooms::units::{error::SmartHomeError, unit_builder::UnitBuilder},
};

use smart_home_lib::homes::rooms::units::unit_visitor::{
  GetAboutVisitor, GetReportVisitor, TurnOnVisitor,
};

use axum::{extract::State, routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), SmartHomeError> {
  // let mut home_1: Home = Home::new("Home1");

  // let mut shared_home: Arc<Mutex<Home>> = Arc::new(Mutex::new(Home::new("home_1")));
  let shared_home: Arc<Mutex<Home>> = Arc::new(Mutex::new(Home::new("home_1")));

  shared_home.lock().unwrap().add_room("test1");

  shared_home.lock().unwrap().add_room("kitchen1");

  shared_home
    .lock()
    .unwrap()
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .add_device(
      UnitBuilder::new()
        .unit_type("socket")
        .name("Socket_builder")
        .about("about_socket_builder")
        .build(),
    );

  shared_home
    .lock()
    .unwrap()
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .add_device(
      UnitBuilder::new()
        .unit_type("thermometer")
        .name("Thermometer_builder")
        .about("about_thermometer_builder")
        .build(),
    );

  shared_home
    .lock()
    .unwrap()
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Socket_builder")
    .unwrap()
    .accept(&GetReportVisitor);

  shared_home
    .lock()
    .unwrap()
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Thermometer_builder")
    .unwrap()
    .accept(&GetAboutVisitor);

  shared_home
    .lock()
    .unwrap()
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Thermometer_builder")
    .unwrap()
    .accept(&TurnOnVisitor);

  // home_1.rooms["kitchen1"].devices["Socket_builder"]
  //   .send_cmd("turnOn")
  //   .await
  //   .map_err(|e| SmartHomeError::DeviceError(format!("Failed to send command: {}", e)))?;

  // let state = AppState {
  //   my_value: String::from("Shared STATE"),
  // };

  // let shared_state = Arc::new(state);

  let app = Router::new()
    .route("/", get(handler))
    .with_state(shared_home);

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}

async fn handler(State(state): State<Arc<Mutex<Home>>>) -> String {
  println!("{}", state.lock().unwrap().name);
  // state.my_value.clone()
  state.lock().unwrap().name.to_string()
  // "test".to_string()
}

// struct AppState {
//   my_value: String,
// }
