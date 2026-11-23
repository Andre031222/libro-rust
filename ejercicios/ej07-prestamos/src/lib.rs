//! Ejercicios del capítulo 7: préstamos y tiempos de vida.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej07-prestamos`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

/// Ejercicio 7.4. Devuelve la primera palabra de `s` (hasta el
/// primer espacio) como una rebanada de `s`, sin copiar bytes.
pub fn primera_palabra(s: &str) -> &str {
    todo!("use s.find(' ') y una rebanada")
}

/// Ejercicio 7.5. Elimina de `v` los números negativos y duplica
/// los que quedan, modificando el vector prestado en su sitio.
// Hace falta `Vec` (no `&mut [i32]`) porque se quitan elementos.
#[allow(clippy::ptr_arg)]
pub fn depurar(v: &mut Vec<i32>) {
    todo!()
}

/// Ejercicio 7.6. Devuelve la más larga de dos cadenas; si miden
/// lo mismo, devuelve `x`. Explique en un comentario qué promete
/// el tiempo de vida `'a` de la firma.
pub fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!()
}

/// Ejercicio 7.7 (reto). Divide `texto` en palabras separadas por
/// espacios en blanco. Cada palabra es una rebanada de `texto`.
pub fn palabras(texto: &str) -> Vec<&str> {
    todo!()
}
