//! Ejercicios del capítulo 1: la crisis de la memoria.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej01-memoria`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

/// Ejercicio 1.4. Devuelve el elemento `i` de `datos`, o `None` si
/// el índice está fuera de límites. No use `[]`.
pub fn leer(datos: &[i32], i: usize) -> Option<i32> {
    todo!()
}

/// Ejercicio 1.5. Suma dos bytes sin desbordar: devuelve `None` si
/// el resultado no cabe en un `u8`.
pub fn suma_segura(a: u8, b: u8) -> Option<u8> {
    todo!()
}

/// Ejercicio 1.6 (reto). Devuelve el prefijo más largo de `s` que
/// ocupe como mucho `max_bytes` bytes sin partir ningún carácter.
pub fn truncar_utf8(s: &str, max_bytes: usize) -> &str {
    todo!()
}
