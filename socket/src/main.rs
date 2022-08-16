use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    //println!("{:?}", stream);
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{:?}", &buffer.len());
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8181")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
