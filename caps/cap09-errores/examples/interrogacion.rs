//! El operador `?` propaga el error a quien llama.
use std::num::ParseIntError;

/// Suma los enteros de un texto separado por comas.
fn sumar(texto: &str) -> Result<i32, ParseIntError> {
    let mut total = 0;
    for parte in texto.split(',') {
        let n: i32 = parte.trim().parse()?; // Err: sale aquí
        total += n;
    }
    Ok(total)
}

fn main() {
    println!("{:?}", sumar("1, 2, 3"));
    println!("{:?}", sumar("1, dos, 3"));
}
