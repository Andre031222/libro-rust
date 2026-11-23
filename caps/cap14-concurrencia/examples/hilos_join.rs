//! Lanzar hilos con `spawn` y esperar su resultado con `join`.

use std::thread;

fn main() {
    let mut manejadores = Vec::new();
    for id in 1..=3_u64 {
        // `move`: la clausura se queda con una copia de `id`
        let manejador = thread::spawn(move || {
            let suma: u64 = (1..=id * 1_000).sum();
            (id, suma)
        });
        manejadores.push(manejador);
    }
    println!("main sigue trabajando mientras tanto");

    for manejador in manejadores {
        // `join` bloquea hasta que ese hilo termina
        let (id, suma) = manejador.join().unwrap();
        println!("hilo {id}: suma = {suma}");
    }
}
