use std::borrow::Cow;
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
    
}

fn handle_client(mut stream: TcpStream) {

  // Request

  let mut request = [0; 4];
  stream.read_exact(&mut request).unwrap();
  let req_len = u32::from_be_bytes(request);

  let mut request = vec![0; req_len as _];
  stream.read_exact(&mut request).unwrap();
  
  // Response 

  //stream.read_exact(&mut response);

  let mut data = "empty";
   
  match &request[..] {
    b"turnOn" => { 
      data = "turnOn";
      turn_on();
    },
    b"turnOff" => data = "turnOff",
    b"report" => data = "report",
    _ => data = "ERR",
  };

  let bytes = data.as_bytes(); 
  let len = bytes.len() as u32;
  let len_bytes = len.to_be_bytes();
  stream.write_all(&len_bytes);
  stream.write_all(bytes);

  println!("Request: {}", String::from_utf8_lossy(&request[..]));
}

fn turn_on() { 
  b"turnOn";
}
//fn turn_off(stream: TcpStream) {}
//fn get_report(stream: TcpStream) {}

fn main() -> Result<()> {

    let test_socket: Socket = Socket {
      name: "Socket1",
      about: "Real Socket 1",
      on_status: false,
      current_power_consumption: 42,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };

    let listener = TcpListener::bind(test_socket.ip)?;
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
