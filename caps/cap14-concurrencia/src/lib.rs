//! Capítulo 14: concurrencia sin miedo.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap14-concurrencia --example hilos_join`.

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn el_crate_compila() {
        let total = Arc::new(Mutex::new(0));
        let otro = Arc::clone(&total);
        thread::spawn(move || *otro.lock().unwrap() += 1)
            .join()
            .unwrap();
        assert_eq!(*total.lock().unwrap(), 1);
    }
}
