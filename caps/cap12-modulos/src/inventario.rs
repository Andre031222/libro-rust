//! Existencias y precios de los productos.

pub mod existencias;
mod precios;

// `precios` es privado: solo su función pública sale afuera.
pub use self::precios::precio_final;
