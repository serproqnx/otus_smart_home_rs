use std::net::UdpSocket;

fn main() {

  let socket = UdpSocket::bind("127.0.0.1:8182").expect("couldn't bind to adress");
  let mut buf = [0; 10];

  let (number_of_bytes, src_addr) = socket.peek_from(&mut buf)
        .expect("Didn't recieve data");

  let filled_buf = &mut buf[..number_of_bytes];
  println!("Addr: {:?}, Buf: {:?}", src_addr, filled_buf);

}
