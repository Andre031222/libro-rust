//! Varios productores y un consumidor con `mpsc`.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone(); // un emisor para cada productor
        thread::spawn(move || {
            for n in 0..4 {
                let mensaje = format!("p{id}-{n}");
                tx.send(mensaje).unwrap(); // el texto se mueve
            }
        });
    }
    drop(tx); // sin esta línea, el consumidor esperaría siempre

    let mut recibidos: Vec<String> = rx.iter().collect();
    println!("recibí {} mensajes", recibidos.len());
    recibidos.sort(); // el orden de llegada varía
    println!("primeros tras ordenar: {:?}", &recibidos[..4]);
}
