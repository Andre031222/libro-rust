//! Devolver un valor transfiere la propiedad a quien llama.

fn crear() -> String {
    String::from("nuevo") // la propiedad sale de la función
}

fn medir(s: String) -> (String, usize) {
    let n = s.len();
    (s, n) // devolvemos s para no perderla
}

fn main() {
    let a = crear();
    let (a, n) = medir(a);
    println!("{a} mide {n} bytes");
}
