#[derive(Debug, Clone)]
pub struct Socket {
  pub name: &'static str,
  pub about: &'static str,
  pub on_status: bool,
  pub current_power_consumption: i32,
}

impl Socket {
  pub fn get_current_power_consumption(&self) -> i32 {
    self.current_power_consumption
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_socket() {
    let test_socket: Socket = Socket {
      name: "1",
      about: "1",
      on_status: true,
      current_power_consumption: 21,
    };

    assert_eq!(test_socket.name, "1");
    assert_eq!(test_socket.about, "1");
    assert!(test_socket.on_status);
    assert_eq!(test_socket.current_power_consumption, 21);
    assert_eq!(test_socket.get_current_power_consumption(), 21);
  }
}
