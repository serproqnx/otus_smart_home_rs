use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
//use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4};
//use std::sync::{Arc, Mutex};

use std::sync::atomic::AtomicBool;

use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

struct Model {
  //button_turn_on: String,
  //button_turn_off: String,
  report: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  TurnOn,
  TurnOff,
}
impl Sandbox for Model {
  type Message = Message;

  fn new() -> Self {
    Self {
   //   button_turn_on: Default::default(),
    //  button_turn_off: Default::default(),
      report: "Test_report".to_string(),
    }
  }

  fn title(&self) -> String {
    "Socket".into()
  }

  fn update(&mut self, message: Self::Message) {
    match message {
      Message::TurnOn => self.report = "Turned On".to_string(),
      Message::TurnOff => self.report = "Turned Off".to_string(),
    }
  }

  fn view(&self) -> Element<'_, Self::Message> {
    column![
      text(self.report.to_string()).size(20),
      button("TurnOn").on_press(Message::TurnOn),
      button("Decrement").on_press(Message::TurnOff)
    ]
    .padding(20)
    .align_items(Alignment::Center)
    .into()
  }
}

struct Socket {
  pub name: &'static str,
  pub about: &'static str,
  pub on_status: AtomicBool,
  pub current_power_consumption: i32,
  pub ip: SocketAddrV4,
}

impl Socket {
  fn set_status_on(&mut self) -> String {
    *self.on_status.get_mut() = true;
    "Turned On".to_string()
  }

  fn set_status_off(&mut self) -> String {
    *self.on_status.get_mut() = false;
    "Turned Off".to_string()
  }

  fn get_report(&mut self) -> String {
    format!(
      "Name: {}, About: {}, On_status: {}, current_power_consumption: {}",
      self.name, 
      self.about, 
      self.on_status.load(std::sync::atomic::Ordering::SeqCst), 
      self.current_power_consumption,
    )
  }
}

async fn handle_client(mut stream: TcpStream, device: &mut Socket) {
  // Request

  let mut request = [0; 4];
  stream.read_exact(&mut request).await.unwrap();
  let req_len = u32::from_be_bytes(request);

  let mut request = vec![0; req_len as _];
  stream.read_exact(&mut request).await.unwrap();

  // Response

  let data = match &request[..] {
    b"turnOn" => device.set_status_on(),
    b"turnOff" => device.set_status_off(),
    b"report" => device.get_report(),
    _ => "ERR".to_string(),
  };

  let bytes = data.as_bytes();
  let len = bytes.len() as u32;
  let len_bytes = len.to_be_bytes();
  stream.write_all(&len_bytes).await.unwrap();
  stream.write_all(bytes).await.unwrap();

  println!("Request: {}", String::from_utf8_lossy(&request[..]));
}

#[tokio::main]
//async fn main() -> io::Result<()> {
async fn main() -> iced::Result {
  let mut power_status = AtomicBool::new(true);

  let t_net = tokio::spawn(async move { net(power_status).await });

  Model::run(Settings {
    window: iced::window::Settings {
      size: (300, 300),
      ..Default::default()
    },
    ..Default::default()
  })
  .unwrap();

  t_net.await.unwrap().unwrap();
  Ok(())
}

async fn net(pwr_stat: AtomicBool) -> io::Result<()> {
  let mut test_socket: Socket = Socket {
    name: "Socket1",
    about: "Real Socket 1",
    on_status: pwr_stat,
    current_power_consumption: 42,
    ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
  };

  println!("SOCKET: {:?}", test_socket.ip);

  let listener = TcpListener::bind(test_socket.ip).await?;

  loop {
    let (socket, _) = listener.accept().await?;
    handle_client(socket, &mut test_socket).await;
  }
}
