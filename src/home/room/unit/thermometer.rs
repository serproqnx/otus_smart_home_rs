#[derive(Debug, Clone)]
// #[derive(Debug)]
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

// #[cfg(test)]
// mod tests {
// 	#[test]
// }
