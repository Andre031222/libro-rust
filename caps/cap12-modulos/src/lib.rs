//! Inventario mínimo para el capítulo 12.
//!
//! El crate se organiza en módulos, pero la API pública se
//! reexporta en la raíz, de modo que basta un `use` corto:
//!
//! ```
//! use cap12_modulos::Existencias;
//!
//! let mut caja = Existencias::new(10)?;
//! caja.agregar(4)?;
//! assert_eq!(caja.libre(), 6);
//! # Ok::<(), cap12_modulos::ErrorExistencias>(())
//! ```

pub mod inventario;

pub use inventario::existencias::{
    ErrorExistencias, Existencias,
};
pub use inventario::precio_final;
