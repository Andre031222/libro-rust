//! Desbordamiento de un `u8`: pánico en debug y envoltura en
//! release.
//! Compare `cargo run --example desborde` con `--release`.

use std::hint::black_box;

fn main() {
    // black_box oculta el valor al compilador, como si llegara
    // de un archivo o de la red.
    let nivel: u8 = black_box(255);
    println!("nivel = {nivel}");
    let siguiente = nivel + 1; // 256 no cabe en un u8
    println!("nivel + 1 = {siguiente}");
}
