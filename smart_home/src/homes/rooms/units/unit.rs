use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use super::unit_visitor::Visitor;
use crate::homes::rooms::units::error::{SHResult, SmartHomeError};
use crate::homes::rooms::units::{socket::Socket, thermometer::Thermometer};
use std::net::UdpSocket;

#[async_trait]
pub trait SmartHomeUnit {
  fn get_name(&self) -> &'static str;
  fn get_bool_on_status(&self) -> bool;
  fn turn_on_off(&mut self);
  fn get_about(&self) -> &'static str;
  fn get_on_status(&self) -> &'static str;
  fn get_device_report(&self) -> Option<String>;

  async fn turn_on(&self) -> SHResult<()>;
  async fn turn_off(&self) -> SHResult<()>;
  async fn send_cmd(&self, cmd: &'static str) -> SHResult<()>;
  async fn get_report(&self) -> SHResult<()>;

  fn accept(&mut self, v: &dyn Visitor);
}

#[async_trait]
impl SmartHomeUnit for Socket {
  fn get_name(&self) -> &'static str {
    self.name
  }

  fn get_bool_on_status(&self) -> bool {
    self.on_status
  }

  fn turn_on_off(&mut self) {
    self.on_status = !&self.on_status;
    println!("{} turned {}", self.name, self.get_on_status());
  }

  fn get_about(&self) -> &'static str {
    println!("About socket builder: {}", self.about);
    self.about
  }

  fn get_on_status(&self) -> &'static str {
    (if self.get_bool_on_status() {
      "ON"
    } else {
      "OFF"
    }) as _
  }

  fn get_device_report(&self) -> Option<String> {
    let report = format!(
      "\nName: {}\nAbout: {}\nPower: {}\nCurrent power consumption: {}\n",
      self.get_name(),
      self.get_about(),
      self.get_on_status(),
      self.get_current_power_consumption(),
    );
    println!("{}", report);
    Some(report)
  }

  async fn turn_on(&self) -> SHResult<()> {
    self.send_cmd("turnOn").await?;
    println!("fn turn_on");
    Ok(())
  }

  async fn turn_off(&self) -> SHResult<()> {
    self.send_cmd("turnOff").await?;
    Ok(())
  }

  async fn send_cmd(&self, cmd: &'static str) -> SHResult<()> {
    let mut stream = TcpStream::connect(self.ip)
      .await
      .map_err(SmartHomeError::TcpStreamError)?;

    let data = cmd;

    let len = data.len() as u32;
    let len_bytes = len.to_be_bytes();

    stream
      .write_all(&len_bytes)
      .await
      .map_err(SmartHomeError::TcpStreamError)?;
    stream
      .write_all(data.as_bytes())
      .await
      .map_err(SmartHomeError::TcpStreamError)?;

    let mut device_response = [0; 4];
    stream
      .read_exact(&mut device_response)
      .await
      .map_err(SmartHomeError::TcpStreamError)?;
    let resp_len = u32::from_be_bytes(device_response);

    let mut device_response = vec![0; resp_len as _];

    stream
      .read_exact(&mut device_response)
      .await
      .map_err(SmartHomeError::TcpStreamError)?;
    println!("Response: {}", String::from_utf8_lossy(&device_response));

    Ok(())
  }

  async fn get_report(&self) -> SHResult<()> {
    self.send_cmd("report").await?;
    Ok(())
  }

  fn accept(&mut self, v: &dyn Visitor) {
    v.visit_socket(self)
  }
}

#[async_trait]
impl SmartHomeUnit for Thermometer {
  fn get_name(&self) -> &'static str {
    self.name
  }

  fn get_bool_on_status(&self) -> bool {
    self.on_status
  }

  fn turn_on_off(&mut self) {
    self.on_status = !&self.on_status;
    println!("{} turned {}", self.name, self.get_on_status());
  }

  fn get_about(&self) -> &'static str {
    // println!("{}", self.about);
    self.about
  }

  fn get_on_status(&self) -> &'static str {
    (if self.get_bool_on_status() {
      "ON"
    } else {
      "OFF"
    }) as _
  }

  fn get_device_report(&self) -> Option<String> {
    let report = format!(
      "\nName: {}\nAbout: {}\nPower: {}\nTemperature: {}\n",
      self.get_name(),
      self.get_about(),
      self.get_on_status(),
      self.get_current_temperature(),
    );
    println!("{}", report);
    Some(report)
  }

  async fn turn_on(&self) -> SHResult<()> {
    Ok(())
  }

  async fn turn_off(&self) -> SHResult<()> {
    Ok(())
  }

  async fn send_cmd(&self, _cmd: &'static str) -> SHResult<()> {
    // let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to adress");
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to adress");

    let send_buf: [u8; 10] = [9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    socket
      .send_to(&send_buf, "127.0.0.1:8182")
      .expect("couldn't send data");

    let mut buf = [0; 10];
    let (amt, src_addr) = socket
      .recv_from(&mut buf)
      .map_err(SmartHomeError::UdpSocketError)?;
    let buf = &mut buf[..amt];
    let temp_from_bytes = i32::from_be_bytes(buf.try_into().expect("it's not temp"));

    println!("Addr: {:?}, Temp: {:?}", &src_addr, &temp_from_bytes);

    Ok(())
  }

  async fn get_report(&self) -> SHResult<()> {
    Ok(())
  }

  fn accept(&mut self, v: &dyn Visitor) {
    v.visit_thermometer(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::net::{Ipv4Addr, SocketAddrV4};
  #[test]
  fn create_smarthomeunit_socket() {
    let mut new_socket = Socket {
      name: "1",
      on_status: true,
      about: "1",
      current_power_consumption: 1,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };

    assert_eq!(new_socket.name, "1");
    assert!(new_socket.on_status);
    assert_eq!(new_socket.about, "1");
    assert_eq!(new_socket.current_power_consumption, 1);

    assert_eq!(new_socket.get_name(), "1");
    assert!(new_socket.get_bool_on_status());

    assert_eq!(new_socket.get_on_status(), "ON");
    new_socket.turn_on_off();
    assert_eq!(new_socket.get_on_status(), "OFF");

    assert_eq!(
      new_socket.get_device_report().unwrap(),
      "\nName: 1\nAbout: 1\nPower: OFF\nCurrent power consumption: 1\n",
    );
  }

  #[test]
  fn create_smarthomeunit_thermometer() {
    let mut new_therm = Thermometer {
      name: "1",
      on_status: true,
      about: "1",
      current_temperature: 1,
      ip: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8181),
    };

    assert_eq!(new_therm.name, "1");
    assert!(new_therm.on_status);
    assert_eq!(new_therm.about, "1");
    assert_eq!(new_therm.current_temperature, 1);

    assert_eq!(new_therm.get_name(), "1");
    assert!(new_therm.get_bool_on_status());

    assert_eq!(new_therm.get_on_status(), "ON");
    new_therm.turn_on_off();
    assert_eq!(new_therm.get_on_status(), "OFF");

    assert_eq!(
      new_therm.get_device_report().unwrap(),
      "\nName: 1\nAbout: 1\nPower: OFF\nTemperature: 1\n",
    );
  }
}
