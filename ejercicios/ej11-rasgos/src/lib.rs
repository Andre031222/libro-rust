//! Ejercicios del capítulo 11: rasgos y genéricos.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej11-rasgos`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

use std::fmt;

/// Ejercicio 11.4. Rasgo para las figuras que tienen área.
pub trait Area {
    /// Devuelve el área de la figura.
    fn area(&self) -> f64;

    /// Método por defecto: `true` si el área supera `umbral`.
    fn es_mayor_que(&self, umbral: f64) -> bool {
        self.area() > umbral
    }
}

/// Un rectángulo de lados `ancho` y `alto`.
pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
}

/// Un círculo de radio `radio`.
pub struct Circulo {
    pub radio: f64,
}

/// Ejercicio 11.4. Área del rectángulo: ancho por alto.
impl Area for Rectangulo {
    fn area(&self) -> f64 {
        todo!()
    }
}

/// Ejercicio 11.4. Área del círculo: pi por el radio al cuadrado
/// (use `std::f64::consts::PI`).
impl Area for Circulo {
    fn area(&self) -> f64 {
        todo!()
    }
}

/// Ejercicio 11.5. Devuelve el mayor de `a` y `b`; si son
/// iguales, devuelve `a`.
pub fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    todo!()
}

/// Ejercicio 11.6. Una fracción `num/den` (`den` distinto de 0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraccion {
    pub num: i64,
    pub den: i64,
}

/// Ejercicio 11.6. Se muestra como `3/4`; si `den` es 1, solo
/// como el numerador (`5`).
impl fmt::Display for Fraccion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

/// Ejercicio 11.6. Un entero `n` se convierte en `n/1`.
impl From<i64> for Fraccion {
    fn from(n: i64) -> Self {
        todo!()
    }
}

/// Ejercicio 11.7. Suma las áreas de una colección
/// heterogénea de figuras.
pub fn area_total(figuras: &[Box<dyn Area>]) -> f64 {
    todo!()
}

/// Ejercicio 11.7 (segunda parte). Cuenta cuántas figuras
/// tienen un área mayor que `umbral`. Use `es_mayor_que`.
pub fn contar_grandes(
    figuras: &[Box<dyn Area>],
    umbral: f64,
) -> usize {
    todo!()
}

/// Reto 11.8. Iterador de los números de Fibonacci
/// (0, 1, 1, 2, 3, 5...) menores o iguales que `limite`.
/// Agregue los campos que necesite para guardar el estado.
/// No debe desbordar aunque `limite` sea `u64::MAX`.
pub struct Fibonacci {}

/// Reto 11.8. Crea el iterador acotado por `limite`.
pub fn fibonacci(limite: u64) -> Fibonacci {
    todo!()
}

/// Reto 11.8. Implemente `next`; los adaptadores vienen solos.
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Reto 11.8 (segunda parte). Suma los términos pares que no
/// superan `limite`, combinando `fibonacci` con adaptadores.
pub fn suma_pares_fibonacci(limite: u64) -> u64 {
    todo!()
}
