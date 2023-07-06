// use core::time;

use std::sync::Arc;
use tokio::sync::Mutex;

use smart_home_lib::homes::{
  home::Home,
  rooms::units::{error::SmartHomeError, unit_builder::UnitBuilder},
};

use smart_home_lib::homes::rooms::units::unit_visitor::{
  GetAboutVisitor, GetReportVisitor, TurnOnVisitor,
};

use axum::{
  extract::{Path, State},
  routing::{delete, get, post},
  Router,
};
use axum_macros::debug_handler;

#[tokio::main]
async fn main() -> Result<(), SmartHomeError> {
  // let mut home_1: Home = Home::new("Home1");

  // let mut shared_home: Arc<Mutex<Home>> = Arc::new(Mutex::new(Home::new("home_1")));
  let shared_home: Arc<Mutex<Home>> = Arc::new(Mutex::new(Home::new("Shared Home")));

  shared_home.lock().await.add_room("test1".to_string());

  shared_home.lock().await.add_room("kitchen1".to_string());

  shared_home
    .lock()
    .await
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .add_device(
      UnitBuilder::new()
        .unit_type("socket".to_string())
        .name("Socket_builder".to_string())
        .about("about_socket_builder")
        .build(),
    );

  shared_home
    .lock()
    .await
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .add_device(
      UnitBuilder::new()
        .unit_type("thermometer".to_string())
        .name("Thermometer_builder".to_string())
        .about("about_thermometer_builder")
        .build(),
    );

  shared_home
    .lock()
    .await
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Socket_builder")
    .unwrap()
    .accept(&GetReportVisitor);

  shared_home
    .lock()
    .await
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Thermometer_builder")
    .unwrap()
    .accept(&GetAboutVisitor);

  shared_home
    .lock()
    .await
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Thermometer_builder")
    .unwrap()
    .accept(&TurnOnVisitor);

  // shared_home.lock().await.rooms["kitchen1"].devices["Socket_builder"]
  //   .send_cmd("turnOn")
  //   .await
  //   .map_err(|e| SmartHomeError::DeviceError(format!("Failed to send command: {}", e)))?;

  // let state = AppState {
  //   my_value: String::from("Shared STATE"),
  // };

  // let shared_state = Arc::new(state);

  let app = Router::new()
    .route("/", get(get_report))
    .route("/turn_on", get(turn_on))
    .route("/list_rooms", get(list_rooms))
    .route("/create_room/:room_name", post(create_room))
    .route("/delete_room/:room_name", delete(delete_room))
    .route("/list_devices/:room_name", get(list_devices))
    .route(
      "/create_device/:room_name/:device_type/:device_name",
      post(create_device),
    )
    .route(
      "/delete_device/:room_name/:device_name",
      delete(delete_device),
    )
    .with_state(shared_home);

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

  Ok(())
}

async fn delete_device(
  Path((room_name, device_name)): Path<(String, String)>,
  // Path(device_type): Path<String>,
  // Path(device_name): Path<String>,
  State(state): State<Arc<Mutex<Home>>>,
) -> String {
  state
    .lock()
    .await
    .rooms
    .get_mut(&room_name)
    .unwrap()
    .del_device(device_name);

  "device deleted".to_string()
}

async fn create_device(
  Path((room_name, device_type, device_name)): Path<(String, String, String)>,
  // Path(device_type): Path<String>,
  // Path(device_name): Path<String>,
  State(state): State<Arc<Mutex<Home>>>,
) -> String {
  println!("device created");
  state
    .lock()
    .await
    .rooms
    .get_mut(&room_name)
    .unwrap()
    .add_device(
      UnitBuilder::new()
        .unit_type(device_type)
        .name(device_name)
        .about("about_socket_builder")
        .build(),
    );

  "device created".to_string()
}

async fn list_devices(
  Path(room_name): Path<String>,
  State(state): State<Arc<Mutex<Home>>>,
) -> String {
  format!("{:?}", state.lock().await.rooms[&room_name])
  // "room list".to_string()
}

async fn delete_room(
  Path(room_name): Path<String>,
  State(state): State<Arc<Mutex<Home>>>,
) -> String {
  println!("room created");
  state.lock().await.del_room(room_name)
}

async fn create_room(
  Path(room_name): Path<String>,
  State(state): State<Arc<Mutex<Home>>>,
) -> String {
  println!("room created");
  state.lock().await.add_room(room_name)
}

async fn list_rooms(State(state): State<Arc<Mutex<Home>>>) -> String {
  format!("{:?}", state.lock().await.get_rooms_list())
  // "room list".to_string()
}

async fn get_report(State(state): State<Arc<Mutex<Home>>>) -> String {
  state
    .lock()
    .await
    .rooms
    .get_mut("kitchen1")
    .unwrap()
    .devices
    .get_mut("Socket_builder")
    .unwrap()
    .get_report()
    .await
    .unwrap()
}

#[debug_handler]
async fn turn_on(State(state): State<Arc<Mutex<Home>>>) -> String {
  state.lock().await.rooms["kitchen1"].devices["Socket_builder"]
    .send_cmd("turnOn")
    .await
    .map_err(|e| SmartHomeError::DeviceError(format!("Failed to send command: {}", e)))
    .unwrap();

  "On".to_string()
}

// struct AppState {
//   my_value: String,
// }
