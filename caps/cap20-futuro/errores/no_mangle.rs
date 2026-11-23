// En la edición 2024, no_mangle debe marcarse como unsafe.
#[no_mangle]
pub extern "C" fn mcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { mcd(b, a % b) }
}
