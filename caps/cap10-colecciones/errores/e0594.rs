fn dos_veces<F: Fn()>(f: F) {
    f();
    f();
}

fn main() {
    let mut cuenta = 0;
    dos_veces(|| cuenta += 1);
    println!("{cuenta}");
}
