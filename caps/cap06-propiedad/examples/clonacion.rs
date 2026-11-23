//! Clonación: `clone` copia también los datos del montículo.

fn main() {
    let s1 = String::from("hola");
    // copia profunda: dos bloques distintos en el montículo
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    println!(
        "¿mismo bloque? {}",
        std::ptr::eq(s1.as_ptr(), s2.as_ptr())
    );
}
