use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};

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

fn handle_client(mut stream: TcpStream, device: &mut Socket) {

  // Request

  let mut request = [0; 4];
  stream.read_exact(&mut request).unwrap();
  let req_len = u32::from_be_bytes(request);

  let mut request = vec![0; req_len as _];
  stream.read_exact(&mut request).unwrap();
  
  // Response 

  let mut data = "empty".to_string();
   
  match &request[..] {
    b"turnOn" => data = device.set_status_on(),
    b"turnOff" => data = device.set_status_off(),
    b"report" => data = device.get_report(),
    _ => data = "ERR".to_string(),
  };

  let bytes = data.as_bytes();
  let len = bytes.len() as u32;
  let len_bytes = len.to_be_bytes();
  stream.write_all(&len_bytes);
  stream.write_all(bytes);

  println!("Request: {}", String::from_utf8_lossy(&request[..]));
}

fn main() -> Result<()> {

    let mut test_socket: Socket = Socket {
      name: "Socket1",
      about: "Real Socket 1",
      on_status: false,
      current_power_consumption: 42,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };


    let listener = TcpListener::bind(test_socket.ip)?;
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?, &mut test_socket);
    }

    Ok(())
}
