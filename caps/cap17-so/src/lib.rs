//! Capítulo 17: estructuras para un núcleo, sin `std`.
//!
//! El crate solo usa `core` (y `alloc` si se activa la
//! característica del mismo nombre). Los ejemplos están en
//! `examples/`: `cargo run -p cap17-so --example so_marcos`.

#![no_std]

// Con la característica `alloc`, hay Vec y Box, pero no
// archivos, hilos ni red: eso sigue siendo cosa de `std`.
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod cabecera;
pub mod cola;
pub mod marcos;
