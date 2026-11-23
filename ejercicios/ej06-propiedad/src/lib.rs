//! Ejercicios del capítulo 6: propiedad (*ownership*).
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej06-propiedad`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

/// Ejercicio 6.4. Devuelve la longitud en bytes de la primera palabra
/// de `s` (hasta el primer espacio), sin tomar la propiedad de `s`.
// En el capítulo 7 se verá por qué `&str` es mejor parámetro.
#[allow(clippy::ptr_arg)]
pub fn primera_palabra_len(s: &String) -> usize {
    todo!("use s.find(' ')")
}

/// Ejercicio 6.5. Toma la propiedad de `a`, presta `b` y devuelve una
/// cadena nueva con `b` agregado al final de `a`.
pub fn concatenar(a: String, b: &str) -> String {
    todo!()
}

/// Ejercicio 6.6. Toma la propiedad de `v` y devuelve dos vectores
/// iguales: el original y una copia profunda.
pub fn duplicar(v: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    todo!()
}

/// Ejercicio 6.7. Toma la propiedad de dos cadenas y devuelve la más
/// larga; si miden lo mismo, devuelve `a`. La otra se libera.
pub fn mover_mayor(a: String, b: String) -> String {
    todo!()
}
