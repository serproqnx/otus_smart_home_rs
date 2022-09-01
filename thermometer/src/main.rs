use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

struct Thermometer {
  name: &'static str,
  about: &'static str,
  temp: i32,
}

impl Thermometer {
  fn gen_temp(&mut self) {
      loop {

        let cur_temp = self.temp;
        self.temp = cur_temp + 1; 

        println!("TEMP: {}", self.temp);
        thread::sleep(Duration::from_millis(1000));

      }
  } 
}

    
fn main() {
  //let mut home_1: Home = Home::new("Home1");

  let mut trm: Thermometer = Thermometer { name: "trm1", about: "about", temp: 42 };
  let t_temp = thread::spawn(move || { trm.gen_temp() }); 


  let socket = UdpSocket::bind("127.0.0.1:8182").expect("couldn't bind to adress");
  let mut buf = [0; 10];

  let (number_of_bytes, src_addr) = socket.peek_from(&mut buf)
        .expect("Didn't recieve data");

  let filled_buf = &mut buf[..number_of_bytes];
  println!("Addr: {:?}, Buf: {:?}", src_addr, filled_buf);
 
  t_temp.join().unwrap(); 
  
}
