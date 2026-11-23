//! Ayudas compartidas por las pruebas de integración.

use cap12_modulos::{ErrorExistencias, Existencias};

/// Almacén de `capacidad` con `cantidad` unidades ya puestas.
pub fn caja_con(
    capacidad: u32,
    cantidad: u32,
) -> Result<Existencias, ErrorExistencias> {
    let mut caja = Existencias::new(capacidad)?;
    caja.agregar(cantidad)?;
    Ok(caja)
}
