use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

// Ejercicio 14.5: `join` asegura que cada clon ya se liberó.
pub fn contar_con_mutex(
    hilos: usize,
    por_hilo: u64,
) -> Arc<Mutex<u64>> {
    let contador = Arc::new(Mutex::new(0));
    let manejadores: Vec<_> = (0..hilos)
        .map(|_| {
            let contador = Arc::clone(&contador);
            thread::spawn(move || {
                for _ in 0..por_hilo {
                    *contador.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in manejadores {
        h.join().unwrap();
    }
    contador
}

// Ejercicio 14.7: un contador atómico, sin cerrojo.
pub fn contar_pares(datos: &[u32], hilos: usize) -> usize {
    if datos.is_empty() {
        return 0;
    }
    let pares = AtomicUsize::new(0);
    let tamano = datos.len().div_ceil(hilos.max(1));
    thread::scope(|s| {
        for trozo in datos.chunks(tamano) {
            let pares = &pares;
            s.spawn(move || {
                for x in trozo {
                    if x % 2 == 0 {
                        pares.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });
    pares.load(Ordering::Relaxed)
}

pub struct GeneradorIds {
    proximo: AtomicUsize,
}

impl GeneradorIds {
    pub const fn new() -> Self {
        GeneradorIds {
            proximo: AtomicUsize::new(1),
        }
    }

    pub fn siguiente(&self) -> usize {
        self.proximo.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for GeneradorIds {
    fn default() -> Self {
        Self::new()
    }
}
