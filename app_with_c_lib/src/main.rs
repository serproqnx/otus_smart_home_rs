extern crate libloading;

use libloading::{Library, Symbol};

fn main() {
  unsafe {
    // Load the library
    let lib = Library::new("./c_socket").unwrap();

    // Get the `socket_new` function from the library
    let socket_new: Symbol<unsafe extern "C" fn() -> *mut u8> = lib.get(b"socket_new").unwrap();
    let socket = socket_new();

    // Get the `socket_turn_on_off` function from the library
    let socket_turn_on_off: Symbol<unsafe extern "C" fn(*mut u8)> =
      lib.get(b"socket_turn_on_off").unwrap();
    socket_turn_on_off(socket);

    // Get the `socket_report` function from the library
    let socket_report: Symbol<unsafe extern "C" fn(*const u8) -> *mut i8> =
      lib.get(b"socket_report").unwrap();
    let report = socket_report(socket);
    let report_str = std::ffi::CStr::from_ptr(report)
      .to_string_lossy()
      .into_owned();
    println!("{}", report_str);

    // Get the `socket_free` function from the library
    let socket_free: Symbol<unsafe extern "C" fn(*mut i8)> = lib.get(b"socket_free").unwrap();
    socket_free(report);

    // Get the `socket_dealloc` function from the library
    let socket_dealloc: Symbol<unsafe extern "C" fn(*mut u8)> = lib.get(b"socket_dealloc").unwrap();
    socket_dealloc(socket);
  }
}
