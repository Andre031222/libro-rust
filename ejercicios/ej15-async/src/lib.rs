//! Ejercicios del capítulo 15: programación asíncrona.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej15-async`.

// Parámetros y campos aún no se usan mientras haya `todo!()`.
#![allow(unused_variables, unused_mut, dead_code)]

use std::collections::VecDeque;
use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

/// Ejecutor mínimo del capítulo (ya resuelto): sondea `futuro`
/// en este hilo hasta que esté listo.
pub fn bloquear<F: Future>(futuro: F) -> F::Output {
    struct DespertarHilo(Thread);
    impl Wake for DespertarHilo {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }
    let mut futuro = pin!(futuro);
    let hilo = Arc::new(DespertarHilo(thread::current()));
    let waker = Waker::from(hilo);
    let mut cx = Context::from_waker(&waker);
    loop {
        match futuro.as_mut().poll(&mut cx) {
            Poll::Ready(valor) => return valor,
            Poll::Pending => thread::park(),
        }
    }
}

/// Ejercicio 15.4. Futuro que devuelve `Pending` en sus primeros
/// `n` sondeos y `Ready(n)` en el siguiente. Antes de cada
/// `Pending` debe llamar a `wake_by_ref` para pedir otro sondeo.
/// Si se sondea después de `Ready`, entra en pánico con el
/// mensaje "sondeo después de Ready".
pub struct ListoTras {
    n: u32,
    sondeos: u32,
    terminado: bool,
}

impl ListoTras {
    pub fn new(n: u32) -> Self {
        ListoTras {
            n,
            sondeos: 0,
            terminado: false,
        }
    }
}

impl Future for ListoTras {
    type Output = u32;
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<u32> {
        todo!()
    }
}

/// Ejercicio 15.5. Espera a `ListoTras::new(x)`; devuelve x².
pub async fn cuadrado_lento(x: u32) -> u32 {
    todo!()
}

/// Ejercicio 15.5. Ejecuta `cuadrado_lento` para cada valor con
/// `bloquear` y devuelve los resultados en el mismo orden.
pub fn ejecutar_todas(valores: &[u32]) -> Vec<u32> {
    todo!()
}

/// Ejercicio 15.6. Futuro que sondea `a` y `b` en cada turno y
/// termina con los dos resultados. No debe volver a sondear un
/// futuro que ya devolvió `Ready`.
pub struct Unir<A: Future, B: Future> {
    a: A,
    b: B,
    ra: Option<A::Output>,
    rb: Option<B::Output>,
}

/// Ejercicio 15.6. Combina dos futuros en uno.
pub fn unir<A, B>(a: A, b: B) -> Unir<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin,
{
    Unir {
        a,
        b,
        ra: None,
        rb: None,
    }
}

impl<A, B> Future for Unir<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin,
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        todo!()
    }
}

type Tarea = Pin<Box<dyn Future<Output = ()>>>;

/// Ejercicio 15.7 (reto). Despertador que vuelve a poner el
/// número de su tarea al final de la cola.
struct Despertador {
    id: usize,
    cola: Arc<Mutex<VecDeque<usize>>>,
}

impl Wake for Despertador {
    fn wake(self: Arc<Self>) {
        todo!()
    }
}

/// Ejercicio 15.7 (reto). Ejecutor *round-robin* de un hilo.
#[derive(Default)]
pub struct RoundRobin {
    tareas: Vec<Option<Tarea>>,
    cola: Arc<Mutex<VecDeque<usize>>>,
}

impl RoundRobin {
    /// Guarda la tarea, la pone en la cola y devuelve su número
    /// (0, 1, 2, ... en el orden de llegada).
    pub fn lanzar(
        &mut self,
        tarea: impl Future<Output = ()> + 'static,
    ) -> usize {
        todo!()
    }

    /// Saca números de la cola y sondea esas tareas hasta que la
    /// cola quede vacía. Devuelve los números en el orden en que
    /// se sondearon. Una tarea terminada no se vuelve a sondear.
    pub fn ejecutar(&mut self) -> Vec<usize> {
        todo!()
    }
}
