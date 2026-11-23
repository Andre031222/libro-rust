//! Un hilo entra en pánico con el cerrojo tomado.

use std::sync::Mutex;
use std::thread;

fn main() {
    let saldo = Mutex::new(100);
    let resultado = thread::scope(|s| {
        s.spawn(|| {
            let mut guarda = saldo.lock().unwrap();
            *guarda -= 30; // la operación empieza...
            panic!("fallo a mitad de camino"); // ...y no termina
        })
        .join()
    });
    println!("¿el hilo terminó bien? {}", resultado.is_ok());
    println!("¿envenenado? {}", saldo.is_poisoned());

    // `into_inner` recupera la guarda aunque haya veneno
    let guarda = saldo.lock().unwrap_or_else(|e| e.into_inner());
    println!("el dato quedó en {}", *guarda);
}
