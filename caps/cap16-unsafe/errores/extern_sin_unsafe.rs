use std::ffi::c_int;

extern "C" {
    fn abs(x: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn c16_doble(x: c_int) -> c_int {
    2 * x
}

fn main() {
    println!("{}", unsafe { abs(-3) });
}
