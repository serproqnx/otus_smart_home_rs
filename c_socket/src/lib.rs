use std::os::raw::c_char;
use std::ffi::CString;

// Define the PowerStatus enum
pub enum PowerStatus {
    On,
    Off,
}

// Implement a method to toggle the PowerStatus
impl PowerStatus {
    fn toggle(&mut self) {
        *self = match *self {
            PowerStatus::On => PowerStatus::Off,
            PowerStatus::Off => PowerStatus::On,
        };
    }
}

// Define the Socket struct
#[repr(C)]
pub struct Socket {
    power_status: PowerStatus,
    current_power_consumption: i32,
}

// Implement the methods for the Socket struct
impl Socket {
    pub fn new() -> Socket {
        Socket {
            power_status: PowerStatus::Off,
            current_power_consumption: 0,
        }
    }

    pub fn turn_on_off(&mut self) {
        self.power_status.toggle();
    }

    pub fn report(&self) -> String {
        let power_status_str = match self.power_status {
            PowerStatus::On => "On",
            PowerStatus::Off => "Off",
        };
        format!("Power Status: {}, Current Power Consumption: {}", power_status_str, self.current_power_consumption)
    }
}

#[no_mangle]
pub extern "C" fn socket_new() -> *mut Socket {
    Box::into_raw(Box::new(Socket::new()))
}

#[no_mangle]
pub extern "C" fn socket_turn_on_off(socket: *mut Socket) {
    let mut socket = unsafe {
        assert!(!socket.is_null());
        &mut *socket
    };
    socket.turn_on_off();
}

#[no_mangle]
pub extern "C" fn socket_report(socket: *const Socket) -> *mut c_char {
    let socket = unsafe {
        assert!(!socket.is_null());
        &*socket
    };
    let report_str = socket.report();
    let c_str = CString::new(report_str).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn socket_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return
    }
    drop(CString::from_raw(ptr))
}

#[no_mangle]
pub unsafe extern "C" fn socket_dealloc(socket: *mut Socket) {
    if socket.is_null() {
        return;
    }
    drop(Box::from_raw(socket));
}
