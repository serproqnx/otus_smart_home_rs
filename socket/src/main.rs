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

    let mut buffer = [0; 4];

    stream.read_exact(&mut buffer).unwrap();
    let len = u32::from_be_bytes(buffer);

    let mut buffer = vec![0; len as _];

    stream.read_exact(&mut buffer).unwrap();

    match &buffer[..] {
       b"turnOn" => stream.write_all(b"turnOn"),
       b"turnOff" => stream.write_all(b"turnOff"),
       b"report" => stream.write_all(b"report"),
       _ => stream.write_all(b"ERR"),
    };

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // stream.write(response);

}

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
