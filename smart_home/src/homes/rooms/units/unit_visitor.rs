use super::{socket::Socket, thermometer::Thermometer, unit::SmartHomeUnit};

pub trait Report {
  fn get_report_v(&self) -> String;
  fn accept(&mut self, v: &dyn Visitor);
}

impl Report for Socket {
  fn get_report_v(&self) -> String {
    //self.get_report
    "get_report test output".to_string()
  }
  fn accept(&mut self, v: &dyn Visitor) {
    v.visit_socket(self)
  }
}

impl Report for Thermometer {
  fn get_report_v(&self) -> String {
    "get_report test output".to_string()
  }
  fn accept(&mut self, v: &dyn Visitor) {
    v.visit_thermometer(self)
  }
}

pub trait Visitor {
  fn visit_socket(&self, report: &mut Socket);
  fn visit_thermometer(&self, report: &mut Thermometer);
}

pub struct GetReportVisitor;

impl Visitor for GetReportVisitor {
  fn visit_socket(&self, unit: &mut Socket) {
    unit.get_device_report().unwrap();
  }
  fn visit_thermometer(&self, unit: &mut Thermometer) {
    unit.get_device_report().unwrap();
  }
}

pub struct GetAboutVisitor;

impl Visitor for GetAboutVisitor {
  fn visit_socket(&self, unit: &mut Socket) {
    unit.get_about();
  }
  fn visit_thermometer(&self, unit: &mut Thermometer) {
    unit.get_about();
  }
}

pub struct TurnOnVisitor;

impl Visitor for TurnOnVisitor {
  fn visit_socket(&self, unit: &mut Socket) {
    let _ = unit.turn_on();
  }
  fn visit_thermometer(&self, unit: &mut Thermometer) {
    let _ = unit.turn_on();
  }
}
