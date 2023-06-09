mod error;



use iced::Settings;
use iced::Theme;
use iced::Element;
use iced::Command;
use iced::Application;
use iced::executor;

use iced::time;
use iced::widget::Text;
use iced::Subscription;
use crate::error::{SocketErr, SocketError};

use std::net::{Ipv4Addr, SocketAddrV4};

// use iced::widget::{button, column, text};
// use iced::{Alignment, Element, Sandbox, Settings};

use std::sync::atomic::AtomicBool;
use std::time::Duration;
// use std::time::Duration;
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
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;

  // type Theme: Default + StyleSheet;
  type Flags = ();

  fn new(_flags: ()) -> (Model, Command<Message>) {
    (
      Model {
        report: "Test_report".to_string(),
        count: 0,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    "Socket".into()
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::TurnOn => self.report = "Turned On".to_string(),
      Message::TurnOff => self.report = "Turned Off".to_string(),
      Message::Tick => {
        self.count += 1;
      }
    }

    Command::none()
  }

  fn view(&self) -> Element<Message> {
    // column![
    //   text(self.report.to_string()).size(20),
    //   button("On").on_press(Message::TurnOn),
    //   button("Off").on_press(Message::TurnOff)
    // ]
    // .padding(20)
    // .align_items(Alignment::Center)
    // .into()

    Text::new(format!("Count: {}", self.count)).into()
  }

  // fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
  //   <Self::Theme as iced::application::StyleSheet>::Style::default()
  // }

  fn theme(&self) -> Self::Theme {
    Self::Theme::default()
  }

  fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
    <Self::Theme as iced::application::StyleSheet>::Style::default()
  }

  fn subscription(&self) -> iced::Subscription<Message> {
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

  let _ = Model::run(Settings::default());

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
