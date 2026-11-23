//! Servicio web de mediciones escrito solo con `std`.
//!
//! Un servidor HTTP/1.1 mínimo: recibe mediciones por POST, las
//! guarda en memoria y las devuelve en JSON. Cada módulo aplica
//! uno o más capítulos del libro.

// Ni una línea de `unsafe` en el servicio (capítulo 16).
#![forbid(unsafe_code)]

pub mod almacen;
pub mod error;
pub mod formato;
pub mod grupo;
pub mod peticion;
pub mod respuesta;
pub mod rutas;
pub mod servidor;

pub use almacen::{Almacen, Medicion};
pub use error::ErrorPeticion;
pub use peticion::{Metodo, Peticion};
pub use respuesta::Respuesta;
pub use servidor::{Apagador, Servidor};
