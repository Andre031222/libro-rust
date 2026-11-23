//! Un contador compartido por ocho hilos: `Arc<Mutex<u64>>`.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0_u64));
    let mut hilos = Vec::new();

    for _ in 0..8 {
        let contador = Arc::clone(&contador); // un dueño más
        hilos.push(thread::spawn(move || {
            for _ in 0..1_000 {
                let mut guarda = contador.lock().unwrap();
                *guarda += 1;
            } // `guarda` se libera aquí y suelta el cerrojo
        }));
    }
    for hilo in hilos {
        hilo.join().unwrap();
    }

    println!("total = {}", *contador.lock().unwrap());
    println!("dueños: {}", Arc::strong_count(&contador));
}
