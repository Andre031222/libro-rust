//! Clonación: `clone` copia también los datos del montículo.

fn main() {
    let s1 = String::from("hola");
    let s2 = s1.clone(); // copia profunda: dos bloques en el montículo

    println!("s1 = {s1}, s2 = {s2}");
    println!(
        "¿mismo bloque? {}",
        std::ptr::eq(s1.as_ptr(), s2.as_ptr())
    );
}
