use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;
//use std::io::prelude::*;
use std::net::{SocketAddrV4, Ipv4Addr};

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

    let mut test_socket: Socket = Socket {
      name: "Socket1",
      about: "Real Socket 1",
      on_status: false,
      current_power_consumption: 42,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };


    let listener = TcpListener::bind(test_socket.ip).await?;
    //
    // accept connections and process them serially for stream in listener. {
    //    handle_client(stream?, &mut test_socket);
    //}
    loop {
        let (socket, _) = listener.accept().await?;
        handle_client(socket, &mut test_socket).await;
    }


    //Ok(())
}
