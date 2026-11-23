/// # Safety
///
/// `p` debe apuntar a un `i32` válido y sin otros préstamos.
pub unsafe fn duplicar(p: *mut i32) {
    *p *= 2;
}

fn main() {
    let mut x = 21;
    // SAFETY: `x` vive y nadie más la toma prestada.
    unsafe { duplicar(&raw mut x) };
    println!("{x}");
}
