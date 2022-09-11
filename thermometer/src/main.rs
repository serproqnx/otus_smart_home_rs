use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::prelude::*;

struct Thermometer {
  name: &'static str,
  about: &'static str,
  temp: Arc<Mutex<i32>>,
}

impl Thermometer {
  fn gen_temp(&mut self) {
    let cur_temp = Arc::clone(&self.temp);
    let mut rng = thread_rng();

    loop {
      
      *cur_temp.lock().unwrap() += rng.gen_range( -1..2 );

      println!("TEMP: {:?}", *self.temp.lock().unwrap());
      //println!("TEMP: {:?}", test_temp);
      thread::sleep(Duration::from_millis(100));
    }

  }

}

fn main() {
  //let mut home_1: Home = Home::new("Home1");

  let mut trm: Thermometer = Thermometer {
    name: "trm1",
    about: "about",
    temp: Arc::new(Mutex::new(0)),
  };

  let temp_arc = Arc::clone(&trm.temp);
  let t_temp = thread::spawn( move || trm.gen_temp() );
  let socket = UdpSocket::bind("127.0.0.1:8182").expect("couldn't bind to adress");
  let mut count = 0i32;

  loop {
    count += 1;
    let mut buf = [0; 10];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't recieve data");

    socket.connect(&src_addr).expect("connection fail");
    let buf = &mut buf[..number_of_bytes];

    println!("{:?}", &buf);

    let temp_bytes = &temp_arc.lock().unwrap().to_be_bytes();
    //let buf = &trm.temp;
    println!("Addr: {:?}, Buf: {:?}", &src_addr, &buf);
    println!("{:?}", temp_bytes);

    socket.send_to(temp_bytes, &src_addr).expect("couldn't send data");

    if count == 100 {
      break;
    };

    //thread::sleep(Duration::from_millis(100));
  }

  t_temp.join().unwrap();
}
