extern "C" {
    fn say_hello(name: *const i8);
}
fn main() {
    let name = "Anthony";//std::ffi::CString::new("World").unwrap();
    unsafe {
        say_hello(name.as_ptr());
    }
}