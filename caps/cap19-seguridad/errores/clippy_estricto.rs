#![warn(clippy::unwrap_used, clippy::indexing_slicing)]
#![warn(clippy::arithmetic_side_effects)]

fn puerto(texto: &str) -> u16 {
    texto.parse().unwrap()
}

fn cabecera(m: &[u8]) -> u8 {
    m[2]
}

fn total(ancho: u32, alto: u32) -> u32 {
    ancho * alto
}

fn main() {
    let m = [0, 1, 2];
    println!("{} {} {}", puerto("80"), cabecera(&m), total(2, 3));
}
