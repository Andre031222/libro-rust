#![forbid(unsafe_code)]

// Un `allow` local no puede rebajar un `forbid`
#[allow(unsafe_code)]
fn primero(v: &[u8]) -> u8 {
    // SAFETY: (no llega a compilar)
    unsafe { *v.get_unchecked(0) }
}

fn main() {
    println!("{}", primero(&[7]));
}
