//! Capítulo 19: seguridad sin mitos.
//!
//! Tres errores que Rust seguro no evita, con su versión
//! corregida, y un mini fuzzer hecho solo con la biblioteca
//! estándar. Ejecute un ejemplo con
//! `cargo run -p cap19-seguridad --example seg_rutas`.

// Ningún módulo de este crate necesita `unsafe`.
#![forbid(unsafe_code)]

pub mod fuzz;
pub mod mensajes;
pub mod rutas;
pub mod tamanos;
