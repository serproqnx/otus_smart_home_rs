use super::{socket::Socket, thermometer::Thermometer};

pub trait Report {
  fn get_report(&self) -> String;
  fn accept(&mut self, v: &Box<dyn Visitor>);
}

impl Report for Socket {
  fn get_report(&self) -> String {
    todo!()
  }
  fn accept(&mut self, v: &Box<dyn Visitor>) {
    v.visit_socket(self)
  }
}

impl Report for Thermometer {
  fn get_report(&self) -> String {
    todo!()
  }
  fn accept(&mut self, v: &Box<dyn Visitor>) {
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
    unit.get_report();
  }
  fn visit_thermometer(&self, unit: &mut Thermometer) {
    unit.get_report();
  }
}
