//! Uso del tipo `ErrorEdad`: mensaje propio y causa original.
use cap09_errores::edad::leer_edad;
use std::error::Error;

fn main() {
    for texto in ["42", "doce", "200"] {
        match leer_edad(texto) {
            Ok(n) => println!("{texto}: {n} años"),
            Err(e) => {
                println!("{texto}: error: {e}");
                if let Some(causa) = e.source() {
                    println!("    causa: {causa}");
                }
            }
        }
    }
}
