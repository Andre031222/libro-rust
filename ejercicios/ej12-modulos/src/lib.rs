//! Ejercicios del capítulo 12: módulos, pruebas y documentación.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej12-modulos`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

pub mod porcentaje;
pub mod texto;

pub use porcentaje::{
    ErrorPorcentaje, Porcentaje, porcentaje_de,
};
