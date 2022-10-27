use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;
//use std::io::prelude::*;
use std::net::{SocketAddrV4, Ipv4Addr};

use iced:: {  button, Button, Column, Element, Row, Sandbox, Settings, Text };

//Model
//View
//Message
//Update

#[derive(Debug, Clone)]
enum Message {
  Increment,
  Decrement,
}

struct Model {
  counter: i64, 
  button_inc: button::State,
  button_dec: button::State,
}

impl Sandbox for Model {
  type Message = Message;

  fn new() -> Self {
    Self {
      counter: 42,
      button_inc: Default::default(),
      button_dec: Default::default(),
    }
  }

  fn title(&self) -> String {
    "Socket".into()
  }

  fn update(&mut self, message: Self::Message) {
    match message {
      Message::Increment => self.counter += 1,
      Message::Decrement => self.counter -= 1,
    }
  }

  fn view(&mut self) -> Element<'_, Self::Message> {
    let text = Text::new(self.counter.to_string()).size(60);
      
    let button_inc = Button::new(&mut self.button_inc, Text::new("Increment")).on_press(Message::Increment);
    let button_dec = Button::new(&mut self.button_dec, Text::new("Decrement")).on_press(Message::Decrement);

    Column::new().padding(20).push(button_inc).push(text).push(button_dec).into()
  } 

}

struct Socket {
  pub name: &'static str,
  pub about: &'static str,
  pub on_status: bool,
  pub current_power_consumption: i32,
  pub ip: SocketAddrV4,
}

impl Socket {

  fn set_status_on(&mut self) -> String {
    self.on_status = true;
    "Turned On".to_string()
  }

  fn set_status_off(&mut self) -> String {
    self.on_status = false;
    "Turned Off".to_string()
  }

  fn get_report(&mut self) -> String {
    format!("Name: {}, About: {}, On_status: {}, current_power_consumption: {}", 
      self.name,
      self.about,
      self.on_status,
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
async fn main() -> io::Result<()> {
//async fn main() -> iced::Result {

    let mut test_socket: Socket = Socket {
      name: "Socket1",
      about: "Real Socket 1",
      on_status: false,
      current_power_consumption: 42,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };
    
    println!("SOCKET: {:?}", test_socket.ip);

    let listener = TcpListener::bind(test_socket.ip).await?;

  let t_gui = tokio::spawn( async move { gui().await });
    //
    // accept connections and process them serially for stream in listener. {
    //    handle_client(stream?, &mut test_socket);
    //}

  t_gui.await?;

    loop {
      let (socket, _) = listener.accept().await?;
      handle_client(socket, &mut test_socket).await;
    }

    //Ok(())
}

async fn gui() -> iced::Result {
  Model::run(
    Settings {
      window: iced::window::Settings {
        size: (300, 200),
        ..Default::default()
      },
      ..Default::default()
    }
  )
}
