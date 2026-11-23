//! Un puntero propio que implementa `Deref`.

use std::ops::Deref;

struct MiCaja<T>(T);

impl<T> Deref for MiCaja<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn saludar(nombre: &str) {
    println!("Hola, {nombre}");
}

fn main() {
    let x = MiCaja(5);
    // `*x` se traduce como `*(x.deref())`.
    println!("x vale {}", *x);

    let nombre = MiCaja(String::from("Ana"));
    // &MiCaja<String> -> &String -> &str
    saludar(&nombre);
    println!("{} letras", nombre.len()); // métodos de String
}
