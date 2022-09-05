use std::net::UdpSocket;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct Thermometer {
  name: &'static str,
  about: &'static str,
  temp: Arc<Mutex<i32>>,
}

impl Thermometer {
  fn gen_temp(&mut self) {
      loop {

        let cur_temp = Arc::clone(&self.temp);
        *cur_temp.lock().unwrap() += 1;
        //self.temp = cur_temp.lock().unwrap() + 1; 

        println!("TEMP: {:?}", self.temp);
        thread::sleep(Duration::from_millis(1000));

      }
  } 

  fn send_temp() {
    
    loop {
        
        let reciever = UdpSocket::bind("127.0.0.1:8182").expect("couldn't bind to adress");
        let mut buf = [0; 10];

        let (number_of_bytes, src_addr) = reciever.peek_from(&mut buf)
            .expect("Didn't recieve data");

        let filled_buf = &mut buf[..number_of_bytes];
        println!("Addr: {:?}, Buf: {:?}", src_addr, filled_buf);

        let sender = UdpSocket::bind("127.0.0.1:8182").expect("couldn't bind to adress");
        sender.send_to(&[0; 10], "127.0.0.1:34254").expect("couldn't send data");

    }

  }
}

    
fn main() {
  //let mut home_1: Home = Home::new("Home1");

  let mut trm: Thermometer = Thermometer { name: "trm1", about: "about", 
        temp: Arc::new(Mutex::new(42)) };
    
  let temp_arc = Arc::clone(&trm.temp);
  let t_temp = thread::spawn( move || { trm.gen_temp() } ); 
  let socket = UdpSocket::bind("127.0.0.1:8182").expect("couldn't bind to adress");
  let mut count = 0u32;

  loop {
      count += 1;
      let mut buf = [0; 10];
      let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
            .expect("Didn't recieve data");
      
      socket.connect(&src_addr).expect("connection fail");
      let buf = &mut buf[..number_of_bytes];
        //
      //let buf = &trm.temp;
      println!("Addr: {:?}, Buf: {:?}", &src_addr, &buf);
      println!("{:?}", &temp_arc.lock().unwrap());
      socket.send_to(buf, &src_addr).expect("couldn't send data");
    
     if count == 100 { break; };

     thread::sleep(Duration::from_millis(1000));
  }
 
  t_temp.join().unwrap(); 
  
}
