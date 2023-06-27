mod error;

use iced::executor;
use iced::widget::Button;
use iced::widget::Column;
use iced::Application;
use iced::Command;
use iced::Element;
use iced::Settings;
use iced::Theme;

use crate::error::{SocketErr, SocketError};
use iced::time;
use iced::widget::Text;
use iced::Subscription;

use std::net::{Ipv4Addr, SocketAddrV4};

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct Model {
  status: AtomicBool,
  report: String,
  count: i32,
  name: String,
  about: String,
  ip: SocketAddrV4,
  current_power_consumption: Arc<AtomicUsize>,
  connection_count: Arc<AtomicUsize>,
  power_status: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
enum Message {
  TurnOn,
  TurnOff,
  Tick,
}

impl Application for Model {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;

  type Flags = (Arc<AtomicUsize>, Arc<AtomicBool>, Arc<AtomicUsize>);

  fn new(
    flags: (Arc<AtomicUsize>, Arc<AtomicBool>, Arc<AtomicUsize>),
  ) -> (Model, Command<Message>) {
    let (connection_count, power_status, current_power_consumption) = flags;
    (
      Model {
        status: AtomicBool::new(false),
        report: "Test_report".to_string(),
        count: 0,
        name: "Test_name".to_string(),
        about: "Test_about".to_string(),
        ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
        current_power_consumption,
        power_status,
        connection_count,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    "Socket".into()
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::TurnOn => {
        self.report = "On".to_string();
        self.current_power_consumption.store(42, Ordering::SeqCst);
        self.status.store(true, Ordering::SeqCst);
        self.power_status.store(true, Ordering::SeqCst);
        println!("{}", self.status.load(Ordering::SeqCst));
      }
      Message::TurnOff => {
        self.report = "Off".to_string();
        self.current_power_consumption.store(0, Ordering::SeqCst);
        self.status.store(false, Ordering::SeqCst);
        self.power_status.store(false, Ordering::SeqCst);
        println!("{}", self.status.load(Ordering::SeqCst));
      }
      Message::Tick => {
        self.count += 1;
      }
    }

    Command::none()
  }

  fn view(&self) -> Element<Self::Message> {
    Column::new()
      .push(Text::new(format!("Name: {}", self.name)).size(20))
      .push(Text::new(format!("About: {}", self.about)).size(20))
      .push(Text::new(format!("IP: {}", self.ip)).size(20))
      .push(
        Text::new(format!(
          "Power: {}",
          self.current_power_consumption.load(Ordering::SeqCst)
        ))
        .size(20),
      )
      .push(Text::new(format!("Uptime: {}", self.count)).size(20))
      .push(
        Text::new(format!(
          "IsOn?: {}",
          self.power_status.load(Ordering::SeqCst)
        ))
        .size(20),
      )
      .push(
        Text::new(format!(
          "ConnectionCount: {}",
          self.connection_count.load(Ordering::SeqCst)
        ))
        .size(20),
      )
      .push(Button::new("On").on_press(Message::TurnOn))
      .push(Button::new("Off").on_press(Message::TurnOff))
      .into()
  }

  fn theme(&self) -> Self::Theme {
    Self::Theme::default()
  }

  fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
    <Self::Theme as iced::application::StyleSheet>::Style::default()
  }

  fn subscription(&self) -> Subscription<Message> {
    time::every(Duration::from_secs(1)).map(|_| Message::Tick)
  }

  fn scale_factor(&self) -> f64 {
    2.0
  }
}

struct Socket {
  pub name: &'static str,
  pub about: &'static str,
  pub power_status: Arc<AtomicBool>,
  pub current_power_consumption: Arc<AtomicUsize>,
  pub ip: SocketAddrV4,
}

impl Socket {
  fn set_status_on(&mut self) -> String {
    self.power_status.store(true, Ordering::SeqCst);
    self.current_power_consumption.store(42, Ordering::SeqCst);

    "Turned On".to_string()
  }

  fn set_status_off(&mut self) -> String {
    self.power_status.store(false, Ordering::SeqCst);
    self.current_power_consumption.store(0, Ordering::SeqCst);
    // status.store(false, Ordering::SeqCst);
    "Turned Off".to_string()
  }

  fn get_report(&mut self) -> String {
    format!(
      "Name: {}, About: {}, On_status: {}, current_power_consumption: {}",
      self.name,
      self.about,
      self.power_status.load(Ordering::SeqCst),
      self.current_power_consumption.load(Ordering::SeqCst),
    )
  }
}

#[tokio::main]
async fn main() -> iced::Result {
  let counter = Arc::new(AtomicUsize::new(0));
  let counter_clone = counter.clone();

  let power_consumtion = Arc::new(AtomicUsize::new(0));
  let power_consumtion_clone = power_consumtion.clone();

  let power_status = Arc::new(AtomicBool::new(false));
  let power_status_clone = power_status.clone();
  let power_status_gui_clone = power_status.clone();

  let srv = tokio::spawn(async move {
    net(
      counter_clone.clone(),
      power_status_clone.clone(),
      power_consumtion_clone.clone(),
    )
    .await
  });

  let _ = Model::run(Settings::with_flags((
    counter,
    power_status_gui_clone,
    power_consumtion,
  )));

  srv.await.unwrap();
  Ok(())
}

async fn net(
  connection_count: Arc<AtomicUsize>,
  power_status: Arc<AtomicBool>,
  current_power_consumption: Arc<AtomicUsize>,
) {
  let mut test_socket: Socket = Socket {
    name: "Socket1",
    about: "Real Socket 1",
    power_status,
    current_power_consumption,
    ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
  };

  println!("SOCKET: {:?}", test_socket.ip);

  let listener = TcpListener::bind(test_socket.ip).await.unwrap();

  while let Ok((stream, _addr)) = listener.accept().await {
    handle_client(stream, &mut test_socket).await.unwrap();

    connection_count.fetch_add(1, Ordering::SeqCst);
  }
}

async fn handle_client(
  mut stream: TcpStream,
  device: &mut Socket,
  // power_status: Arc<AtomicBool>,
  // current_power_consumption: Arc<AtomicUsize>
) -> SocketErr<()> {
  // Request

  let mut request = [0; 4];

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
