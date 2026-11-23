//! Ejercicios del capítulo 5: tipos, variables y control de
//! flujo.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej05-tipos`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

/// Ejercicio 5.4. Convierte grados Celsius a Fahrenheit:
/// F = C × 9/5 + 32. Escriba el cuerpo como una sola expresión.
pub fn celsius_a_fahrenheit(c: f64) -> f64 {
    todo!()
}

/// Ejercicio 5.4 (cont.). Clasifica una temperatura en Celsius
/// con un `if` usado como expresión: menos de 10 es "frío",
/// menos de 25 es "templado" y el resto, "caluroso".
pub fn clasificar(c: f64) -> &'static str {
    todo!()
}

/// Ejercicio 5.5. Calcula los bytes de una imagen de
/// `ancho` × `alto` píxeles con `bytes_por_pixel` bytes cada uno.
/// Devuelve `None` si el resultado no cabe en un `u32`.
pub fn tamano_bufer(
    ancho: u32,
    alto: u32,
    bytes_por_pixel: u32,
) -> Option<u32> {
    todo!()
}

/// Ejercicio 5.6. Recorre `datos` con `for` y devuelve el mayor
/// valor y su posición; si se repite, la primera posición.
pub fn maximo_y_posicion(datos: [i32; 5]) -> (i32, usize) {
    todo!()
}

/// Ejercicio 5.7 (reto). Devuelve FizzBuzz de 1 a `n`: "Fizz"
/// para múltiplos de 3, "Buzz" de 5, "FizzBuzz" de ambos y el
/// número en los demás casos. No use `%` ni divisiones: lleve
/// dos contadores y decida con `match` sobre una tupla.
pub fn fizzbuzz(n: u32) -> Vec<String> {
    todo!()
}
