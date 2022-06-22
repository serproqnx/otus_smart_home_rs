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
