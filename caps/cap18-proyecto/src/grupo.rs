//! Grupo de hilos de trabajo (reto 14.8).

use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};

type Tarea = Box<dyn FnOnce() + Send + 'static>;

/// `n` hilos creados una vez que reciben tareas por un canal.
pub struct GrupoHilos {
    emisor: Option<Sender<Tarea>>,
    hilos: Vec<JoinHandle<()>>,
}

impl GrupoHilos {
    /// Crea `n` hilos. Entra en pánico si `n` es 0.
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "el grupo necesita al menos un hilo");
        let (emisor, receptor) = mpsc::channel::<Tarea>();
        let receptor = Arc::new(Mutex::new(receptor));
        let hilos = (0..n)
            .map(|_| {
                let receptor = Arc::clone(&receptor);
                thread::spawn(move || trabajar(&receptor))
            })
            .collect();
        Self {
            emisor: Some(emisor),
            hilos,
        }
    }

    /// Envía una tarea al primer hilo libre.
    pub fn ejecutar<F>(&self, tarea: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(emisor) = &self.emisor {
            // Solo falla si ya no hay hilos que reciban.
            let _ = emisor.send(Box::new(tarea));
        }
    }
}

fn trabajar(receptor: &Mutex<Receiver<Tarea>>) {
    loop {
        // El cerrojo se suelta al terminar esta sentencia,
        // antes de ejecutar la tarea.
        let siguiente = receptor
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .recv();
        let Ok(tarea) = siguiente else {
            return; // canal cerrado: el grupo se liberó
        };
        // Un pánico en una tarea no mata al hilo.
        let _ = panic::catch_unwind(AssertUnwindSafe(tarea));
    }
}

impl Drop for GrupoHilos {
    fn drop(&mut self) {
        drop(self.emisor.take()); // cierra el canal
        for hilo in self.hilos.drain(..) {
            let _ = hilo.join();
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn ejecuta_todas_las_tareas_antes_de_liberarse() {
        let hechas = Arc::new(AtomicUsize::new(0));
        {
            let grupo = GrupoHilos::new(3);
            for _ in 0..100 {
                let hechas = Arc::clone(&hechas);
                grupo.ejecutar(move || {
                    hechas.fetch_add(1, Ordering::Relaxed);
                });
            }
        } // el Drop espera a los hilos
        assert_eq!(hechas.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn sobrevive_a_una_tarea_que_entra_en_panico() {
        let hechas = Arc::new(AtomicUsize::new(0));
        {
            let grupo = GrupoHilos::new(1);
            grupo.ejecutar(|| panic!("tarea rota"));
            let h = Arc::clone(&hechas);
            grupo.ejecutar(move || {
                h.fetch_add(1, Ordering::Relaxed);
            });
        }
        assert_eq!(hechas.load(Ordering::Relaxed), 1);
    }
}
