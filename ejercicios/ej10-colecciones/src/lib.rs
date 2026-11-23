//! Ejercicios del capítulo 10: colecciones e iteradores.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej10-colecciones`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

use std::collections::HashMap;

/// Ejercicio 10.4. Cuenta cuántas veces aparece cada palabra de
/// `texto`, en minúsculas. Separe con `split_whitespace` y use la
/// API `entry`.
pub fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    todo!()
}

/// Ejercicio 10.5. Devuelve los cuadrados de los números pares de
/// `v`, en el mismo orden. Sin bucles `for` ni `while`.
pub fn cuadrados_de_pares(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Ejercicio 10.5 (segunda parte). Producto punto de dos vectores
/// con `zip`, `map` y `sum`; si difieren en longitud, se ignoran
/// los elementos sobrantes.
pub fn producto_punto(a: &[f64], b: &[f64]) -> f64 {
    todo!()
}

/// Ejercicio 10.6. Devuelve una clausura que acumula: cada
/// llamada suma su argumento al total y devuelve el total
/// hasta ese momento.
pub fn crear_acumulador() -> impl FnMut(i32) -> i32 {
    // Reemplace este cuerpo; la clausura debe capturar el total.
    |x: i32| -> i32 { todo!() }
}

/// Reto 10.7. Índice invertido: a cada palabra (en minúsculas y
/// sin signos de puntuación en los extremos) le asocia los
/// números de línea, desde 1, en que aparece, en orden y sin
/// repetir.
pub fn indice_invertido(
    texto: &str,
) -> HashMap<String, Vec<usize>> {
    todo!()
}
