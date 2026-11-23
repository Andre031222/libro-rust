//! Ejecutor de un solo hilo con una cola de tareas.

use std::pin::Pin;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Wake, Waker};

type Tarea = Pin<Box<dyn Future<Output = ()> + Send>>;

/// Una tarea: su futuro y la cola donde volver a formarse.
struct Celda {
    futuro: Mutex<Option<Tarea>>,
    cola: Sender<Arc<Celda>>,
}

impl Wake for Celda {
    fn wake(self: Arc<Self>) {
        let cola = self.cola.clone();
        cola.send(self).expect("el ejecutor ya terminó");
    }
}

/// Saca tareas de la cola y las sondea.
pub struct Ejecutor {
    cola: Receiver<Arc<Celda>>,
}

/// Pone tareas nuevas en la cola.
#[derive(Clone)]
pub struct Lanzador {
    cola: Sender<Arc<Celda>>,
}

/// Crea un ejecutor y el lanzador que lo alimenta.
pub fn nuevo() -> (Ejecutor, Lanzador) {
    let (tx, rx) = mpsc::channel();
    (Ejecutor { cola: rx }, Lanzador { cola: tx })
}

impl Lanzador {
    pub fn lanzar<F>(&self, futuro: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let celda = Arc::new(Celda {
            futuro: Mutex::new(Some(Box::pin(futuro))),
            cola: self.cola.clone(),
        });
        self.cola.send(celda).expect("el ejecutor ya terminó");
    }
}

impl Ejecutor {
    /// Ejecuta hasta que no quede ninguna tarea viva.
    pub fn ejecutar(self) {
        while let Ok(celda) = self.cola.recv() {
            let mut ranura = celda.futuro.lock().unwrap();
            if let Some(mut futuro) = ranura.take() {
                let waker = Waker::from(Arc::clone(&celda));
                let mut cx = Context::from_waker(&waker);
                if futuro.as_mut().poll(&mut cx).is_pending() {
                    *ranura = Some(futuro); // aún no termina
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::nuevo;
    use crate::futuros::ceder;
    use std::sync::{Arc, Mutex};

    #[test]
    fn las_tareas_se_intercalan() {
        let registro = Arc::new(Mutex::new(Vec::new()));
        let (ejecutor, lanzador) = nuevo();
        for nombre in ["A", "B"] {
            let r = Arc::clone(&registro);
            lanzador.lanzar(async move {
                r.lock().unwrap().push(nombre);
                ceder().await;
                r.lock().unwrap().push(nombre);
            });
        }
        drop(lanzador);
        ejecutor.ejecutar();
        assert_eq!(
            *registro.lock().unwrap(),
            ["A", "B", "A", "B"]
        );
    }
}
