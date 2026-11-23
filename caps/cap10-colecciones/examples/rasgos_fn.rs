//! `Fn`, `FnMut` y `FnOnce` como límites de un genérico.

fn dos_veces<F: Fn() -> i32>(f: F) -> i32 {
    f() + f() // Fn: se llama varias veces sin cambiar nada
}

fn repetir<F: FnMut()>(n: usize, mut f: F) {
    for _ in 0..n {
        f(); // FnMut: puede modificar lo capturado
    }
}

fn una_vez<F: FnOnce() -> String>(f: F) -> String {
    f() // FnOnce: puede consumir lo capturado
}

fn main() {
    let base = 21;
    println!("{}", dos_veces(|| base));

    let mut log = Vec::new();
    repetir(3, || log.push("tic"));
    println!("{log:?}");

    let texto = String::from("adiós");
    println!("{}", una_vez(move || texto + "!"));
}
