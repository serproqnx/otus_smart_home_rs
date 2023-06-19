mod error;

use tokio::sync::Mutex;

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

// use iced::widget::{button, column, text};
// use iced::{Alignment, Element, Sandbox, Settings};

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
// use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
// use std::net::{TcpListener, TcpStream};

struct Model {
  //button_turn_on: String,
  //button_turn_off: String,
  status: AtomicBool,
  report: String,
  count: i32,
  pub name: String,
  pub about: String,
  pub on_status: AtomicBool,
  pub current_power_consumption: i32,
  pub ip: SocketAddrV4,
  connection_count: Arc<AtomicUsize>,
  counter: Arc<Mutex<i32>>,
}

// impl Model {
//   fn set_status_on(&mut self) -> String {
//     *self.on_status.get_mut() = true;
//     "Turned On".to_string()
//   }

//   fn set_status_off(&mut self) -> String {
//     *self.on_status.get_mut() = false;
//     "Turned Off".to_string()
//   }

//   fn get_report(&mut self) -> String {
//     format!(
//       "Name: {}, About: {}, On_status: {}, current_power_consumption: {}",
//       self.name,
//       self.about,
//       self.on_status.load(std::sync::atomic::Ordering::SeqCst),
//       self.current_power_consumption,
//     )
//   }
// }

#[derive(Debug, Clone)]
enum Message {
  TurnOn,
  TurnOff,
  Tick,
  // ConnectionCount(i32),
}

async fn rtrn42() -> i32 {
  42
}

impl Application for Model {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;

  // type Theme: Default + StyleSheet;
  type Flags = Arc<AtomicUsize>;

  fn new(connection_count: Arc<AtomicUsize>) -> (Model, Command<Message>) {
    // let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(1));
    // println!("NEW {}", counter.try_lock().unwrap());
    // drop(counter);
    // let count = counter.lock().unwrap();
    // let command = Command::perform(net(Arc::clone(&counter)), Message::ConnectionCount);
    // let command = Command::perform(net(Arc::clone(&counter)), Message::ConnectionCount);
    // let command = Command::single(net());
    (
      Model {
        status: AtomicBool::new(false),
        report: "Test_report".to_string(),
        count: 0,
        name: "Test_name".to_string(),
        about: "Test_about".to_string(),
        on_status: AtomicBool::new(false),
        current_power_consumption: 42,
        ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
        connection_count,
        counter: Arc::new(Mutex::new(0)),
      },
      Command::none(),
      // command,
    )
  }

  fn title(&self) -> String {
    "Socket".into()
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::TurnOn => {
        self.report = "Turned On".to_string();
        self.status = AtomicBool::new(true);
      }
      Message::TurnOff => self.report = "Turned Off".to_string(),
      Message::Tick => {
        self.count += 1;
      } // Message::ConnectionCount(count) => {
        //   println!("UPDATE {}", self.connection_count.load(Ordering::SeqCst));
        // }
    }

    Command::none()
  }

  fn view(&self) -> Element<Self::Message> {
    Column::new()
      .push(Text::new(format!("Count: {}", self.count)).size(20))
      .push(Text::new(format!("Status: {}", self.status.load(Ordering::Relaxed))).size(20))
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
    // Subscription::from_recipe(Duration::from_secs(1)).map(Message::Tick)
    time::every(Duration::from_secs(1)).map(|_| Message::Tick)
  }

  fn scale_factor(&self) -> f64 {
    2.0
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
  fn set_status_on(&mut self, status: Arc<AtomicBool>) -> String {
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

async fn handle_client(
  mut stream: TcpStream,
  device: &mut Socket,
  power_status: Arc<AtomicBool>,
) -> SocketErr<()> {
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
    b"turnOn" => device.set_status_on(power_status.load(order)),
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
async fn main() -> iced::Result {
  let counter = Arc::new(AtomicUsize::new(0));
  let counter_clone = counter.clone();

  let power_status = Arc::new(AtomicBool::new(false));
  let power_status_clone = power_status.clone();
  // let power_status = AtomicBool::new(true);

  // let rt = Runtime::new().unwrap();

  let srv =
    tokio::spawn(async move { net(counter_clone.clone(), power_status_clone.clone()).await });

  // rt.spawn(async {
  //   Model::run(Settings::default());
  // });

  // let _ = Model::run(Settings::default());
  // let _ = Model::run(Settings::with_flags(counter));
  let _ = Model::run(Settings::with_flags(counter));

  srv.await.unwrap();
  Ok(())
}

async fn net(connection_count: Arc<AtomicUsize>, power_status: Arc<AtomicBool>) {
  let mut test_socket: Socket = Socket {
    name: "Socket1",
    about: "Real Socket 1",
    on_status: AtomicBool::new(false),
    current_power_consumption: 42,
    ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
  };

  println!("SOCKET: {:?}", test_socket.ip);

  let listener = TcpListener::bind(test_socket.ip).await.unwrap();

  let power_status_net_clone = power_status.clone();

  while let Ok((stream, _addr)) = listener.accept().await {
    let res = handle_client(stream, &mut test_socket, power_status_net_clone.clone())
      .await
      .unwrap();

    connection_count.fetch_add(1, Ordering::SeqCst);

    // *counter += 1;
    // drop(counter);
  }
}
