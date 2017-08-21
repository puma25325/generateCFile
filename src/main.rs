include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;



fn main() {
    let name = CString::new("World").expect("CString::new failed");
    unsafe {
        say_hello(name.as_ptr());
    }
}