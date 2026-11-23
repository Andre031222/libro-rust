//! Binario del capítulo 4: usa la biblioteca del mismo paquete.

use cap04_herramientas::promedio;

fn main() {
    let notas = [14.0, 17.5, 12.0];
    match promedio(&notas) {
        Some(p) => println!("promedio: {p:.2}"),
        None => println!("no hay notas"),
    }
}
