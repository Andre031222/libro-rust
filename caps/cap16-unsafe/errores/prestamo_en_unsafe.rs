fn main() {
    let mut x = 1;
    unsafe {
        let a = &mut x;
        let b = &mut x;
        *a += 1;
        *b += 1;
    }
}
