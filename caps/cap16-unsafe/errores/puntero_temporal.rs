use std::ffi::{CString, c_char};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let p = CString::new("hola").unwrap().as_ptr();
    let n = unsafe { strlen(p) };
    println!("{n}");
}
