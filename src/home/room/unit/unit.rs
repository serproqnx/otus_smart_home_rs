// pub mod socket;
// pub mod thermometer;

use crate::home::room::unit::{socket::Socket, thermometer::Thermometer};

pub trait SmartHomeUnit {
  // fn new(name: &'static str) -> Self;
  fn get_name(&self) -> &'static str;
  fn get_bool_on_status(&self) -> bool;
  fn turn_on_off(&mut self);
  fn get_about(&self) -> &'static str;
  fn get_on_status(&self) -> &'static str;
  fn get_device_report(&self) -> String;
}

impl SmartHomeUnit for Socket {
  fn get_device_report(&self) -> String {
    let report = format!(
      "\nName: {}\nAbout: {}\nPower: {}\nCurrent power consumption: {}\n",
      self.get_name(),
      self.get_about(),
      self.get_on_status(),
      self.get_current_power_consumption(),
    );
    println!("{}", report);
    report
  }

  fn get_about(&self) -> &'static str {
    self.about
  }

  fn get_on_status(&self) -> &'static str {
    (if self.get_bool_on_status() {
      "ON"
    } else {
      "OFF"
    }) as _
  }

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
}

impl SmartHomeUnit for Thermometer {
  fn get_device_report(&self) -> String {
    let report = format!(
      "\nName: {}\nAbout: {}\nPower: {}\nTemperature: {}\n",
      self.get_name(),
      self.get_about(),
      self.get_on_status(),
      self.get_current_temperature(),
    );
    println!("{}", report);
    report
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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_smarthomeunit_socket() {
    let mut new_socket = Socket {
      name: "1",
      on_status: true,
      about: "1",
      current_power_consumption: 1,
    };

    assert_eq!(new_socket.name, "1");
    assert_eq!(new_socket.on_status, true);
    assert_eq!(new_socket.about, "1");
    assert_eq!(new_socket.current_power_consumption, 1);

    assert_eq!(new_socket.get_name(), "1");
    assert_eq!(new_socket.get_bool_on_status(), true);

    assert_eq!(new_socket.get_on_status(), "ON");
    new_socket.turn_on_off();
    assert_eq!(new_socket.get_on_status(), "OFF");

    assert_eq!(
      new_socket.get_device_report(),
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
    };
    assert_eq!(new_therm.name, "1");
    assert_eq!(new_therm.on_status, true);
    assert_eq!(new_therm.about, "1");
    assert_eq!(new_therm.current_temperature, 1);

    assert_eq!(new_therm.get_name(), "1");
    assert_eq!(new_therm.get_bool_on_status(), true);

    assert_eq!(new_therm.get_on_status(), "ON");
    new_therm.turn_on_off();
    assert_eq!(new_therm.get_on_status(), "OFF");

    assert_eq!(
      new_therm.get_device_report(),
      "\nName: 1\nAbout: 1\nPower: OFF\nTemperature: 1\n",
    );
  }
}
