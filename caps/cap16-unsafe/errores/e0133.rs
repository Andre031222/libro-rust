/// # Safety
///
/// `p` debe apuntar a un `i32` válido.
unsafe fn leer(p: *const i32) -> i32 {
    unsafe { *p }
}

fn main() {
    let x = 7;
    let n = leer(&raw const x);
    println!("{n}");
}
