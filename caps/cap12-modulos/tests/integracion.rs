//! Pruebas de integración: solo ven la API pública.

mod comun;

use cap12_modulos::{
    ErrorExistencias, Existencias, precio_final,
};

#[test]
fn retirar_tras_agregar() -> Result<(), ErrorExistencias> {
    let mut caja = comun::caja_con(10, 7)?;
    caja.retirar(2)?;
    assert_eq!(caja.cantidad(), 5);
    assert_eq!(caja.libre(), 5);
    Ok(())
}

#[test]
fn capacidad_cero_es_error() {
    let error = Existencias::new(0).unwrap_err();
    assert_eq!(error, ErrorExistencias::CapacidadCero);
}

#[test]
fn valor_incluye_impuesto() -> Result<(), ErrorExistencias> {
    let caja = comun::caja_con(10, 4)?;
    assert_eq!(caja.valor(250), precio_final(1000));
    Ok(())
}

#[test]
#[ignore = "lenta: dos millones de operaciones"]
fn muchas_operaciones() {
    let mut caja = Existencias::new(u32::MAX).unwrap();
    for _ in 0..1_000_000 {
        caja.agregar(1).unwrap();
        caja.retirar(1).unwrap();
    }
    assert_eq!(caja.cantidad(), 0);
}
