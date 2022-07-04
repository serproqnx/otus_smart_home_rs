#[derive(Debug, Clone)]

pub struct Thermometer {
  pub name: &'static str,
  pub about: &'static str,
  pub on_status: bool,
  pub current_temperature: i32,
}

impl Thermometer {
  pub fn get_current_temperature(&self) -> i32 {
    self.current_temperature
  }
}

#[cfg(test)]
mod tests {
  use super::Thermometer;

  #[test]
  fn create_thermometer() {
    let thermometer1: Thermometer = Thermometer {
      name: "1",
      about: "1",
      on_status: true,
      current_temperature: 21,
    };

    assert_eq!(thermometer1.name, "1");
    assert_eq!(thermometer1.about, "1");
    assert_eq!(thermometer1.on_status, true);
    assert_eq!(thermometer1.current_temperature, 21);
    assert_eq!(thermometer1.get_current_temperature(), 21);
  }
}
