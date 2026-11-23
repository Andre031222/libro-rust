//! Binario del paquete: usa la biblioteca como un crate externo.

use cap12_modulos::{
    ErrorExistencias, Existencias, precio_final,
};

fn main() -> Result<(), ErrorExistencias> {
    let mut caja = Existencias::new(12)?;
    caja.agregar(10)?;
    caja.retirar(4)?;
    let (hay, libre) = (caja.cantidad(), caja.libre());
    println!("hay {hay}; caben {libre} más");
    println!("valor a 250 la unidad: {}", caja.valor(250));
    println!("precio final de 1000: {}", precio_final(1000));
    if let Err(e) = caja.agregar(20) {
        println!("rechazado: {e:?}");
    }
    Ok(())
}
