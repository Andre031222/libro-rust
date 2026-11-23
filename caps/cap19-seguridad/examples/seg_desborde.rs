use cap19_seguridad::tamanos::{
    LIMITE, acepta, acepta_ingenuo, bytes_ingenuo,
};
use std::env;

fn main() {
    // Ancho y alto llegan de fuera, de la línea de órdenes
    let args: Vec<String> = env::args().skip(1).collect();
    let leer = |i: usize, defecto: u32| {
        args.get(i)
            .and_then(|s| s.parse().ok())
            .unwrap_or(defecto)
    };
    let (ancho, alto) = (leer(0, 65_536), leer(1, 16_384));

    println!("imagen {ancho} x {alto}, límite {LIMITE} bytes");
    println!(
        "bytes (ingenuo):     {}",
        bytes_ingenuo(ancho, alto)
    );
    println!(
        "¿acepta? ingenuo:    {}",
        acepta_ingenuo(ancho, alto)
    );
    println!("¿acepta? corregido:  {}", acepta(ancho, alto));
}
