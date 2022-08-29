use std::net::TcpStream;

fn send_string(&mut stream: TcpStream) {

  let mut request = [0; 4];
  stream.read_exact(&mut request).unwrap();
  let req_len = u32::from_be_bytes(request);

  let mut request = vec![0; req_len as _];
  stream.read_exact(&mut request).unwrapa();

}

fn recv_string() {
    
    

}
