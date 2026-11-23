//! Leer la cabecera de una imagen binaria sin copiarla.

use cap17_so::cabecera::leer;

fn main() {
    let imagen: [u8; 20] = [
        b'R', b'S', b'O', b'1', // firma
        1, 0, // versión 1
        0, 0, // banderas
        2, 0, 0, 0, // entrada: byte 2 del cuerpo
        4, 0, 0, 0, // tamaño del cuerpo: 4 bytes
        0x90, 0x90, 0xF4, 0xEB, // cuerpo
    ];

    match leer(&imagen) {
        Ok((c, cuerpo)) => {
            println!("{c:?}");
            println!("cuerpo: {cuerpo:02X?}");
        }
        Err(e) => println!("imagen inválida: {e:?}"),
    }
    println!("truncada: {:?}", leer(&imagen[..12]).err());
}
