mod error;

use crate::error::{SocketErr, SocketError};

use iced::{executor, Application, Command, Element, Settings, Text};
use iced::subscription::Interval;
use std::time::Duration;

// use iced::widget::{button, column, text};
// use iced::{Alignment, Element, Sandbox, Settings};

use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::atomic::AtomicBool;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct Model {
  //button_turn_on: String,
  //button_turn_off: String,
  report: String,
  count: usize,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  TurnOn,
  TurnOff,
  Tick,
}

impl Application for Model {
  type Executor = Executor;
  type Message = Debug + Send;
  type Theme: Default + StyleSheet;
  type Flags = ();

  fn new() -> Self {
    Self {
      //   button_turn_on: Default::default(),
      //  button_turn_off: Default::default(),
      report: "Test_report".to_string(),
      count: 0,
    }
  }

  fn title(&self) -> String {
    "Socket".into()
  }

  fn update(&mut self, message: Self::Message) {
    match message {
      Message::TurnOn => self.report = "Turned On".to_string(),
      Message::TurnOff => self.report = "Turned Off".to_string(),
      Message::Tick => {
        self.count = self.count += 1;
      }
    }
  }

  fn view(&self) -> Element<'_, Self::Message> {
    column![
      text(self.report.to_string()).size(20),
      button("On").on_press(Message::TurnOn),
      button("Off").on_press(Message::TurnOff)
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

async fn handle_client(mut stream: TcpStream, device: &mut Socket) -> SocketErr<()> {
  // Request

  let mut request = [0; 4];
  // stream.read_exact(&mut request).await.unwrap();
  stream
    .read_exact(&mut request)
    .await
    .map_err(SocketError::TcpReadError)?;

  let req_len = u32::from_be_bytes(request);

  let mut request = vec![0; req_len as _];
  stream
    .read_exact(&mut request)
    .await
    .map_err(SocketError::TcpReadError)?;

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
  stream
    .write_all(&len_bytes)
    .await
    .map_err(SocketError::TcpWriteError)?;

  stream
    .write_all(bytes)
    .await
    .map_err(SocketError::TcpWriteError)?;

  println!("Request: {}", String::from_utf8_lossy(&request[..]));

  Ok(())
}

#[tokio::main]
//async fn main() -> io::Result<()> {
async fn main() -> iced::Result {
  let power_status = AtomicBool::new(true);

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

async fn net(pwr_stat: AtomicBool) -> SocketErr<()> {
  let mut test_socket: Socket = Socket {
    name: "Socket1",
    about: "Real Socket 1",
    on_status: pwr_stat,
    current_power_consumption: 42,
    ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
  };

  println!("SOCKET: {:?}", test_socket.ip);

  let listener = TcpListener::bind(test_socket.ip)
    .await
    .map_err(SocketError::TcpError)?;

  loop {
    let (socket, _) = listener.accept().await.map_err(SocketError::TcpError)?;

    handle_client(socket, &mut test_socket).await?
  }
  // Ok(())
}
