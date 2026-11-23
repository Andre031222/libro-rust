//! Un contador atómico: sin cerrojo y sin `Arc`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let atendidas = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..250_000 {
                    atendidas.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    let total = atendidas.load(Ordering::Relaxed);
    println!("peticiones atendidas: {total}");
}
