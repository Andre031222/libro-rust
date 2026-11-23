//! Ejercicios del capítulo 14: concurrencia sin miedo.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej14-concurrencia`.

// Parámetros y campos aún no se usan mientras haya `todo!()`.
#![allow(unused_variables, dead_code)]

use std::sync::atomic::AtomicUsize;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Ejercicio 14.4. Suma `datos` repartidos en `hilos` trozos;
/// cada trozo lo suma un hilo de `thread::scope`. Si `hilos`
/// es 0, use un solo hilo. Pista: `chunks` y `div_ceil`.
pub fn suma_paralela(datos: &[i64], hilos: usize) -> i64 {
    todo!()
}

/// Ejercicio 14.5. Lanza `hilos` hilos con `thread::spawn`; cada
/// uno suma 1 al contador `por_hilo` veces. Devuelve el contador
/// compartido, ya sin otros dueños que el que se devuelve.
pub fn contar_con_mutex(
    hilos: usize,
    por_hilo: u64,
) -> Arc<Mutex<u64>> {
    todo!()
}

/// Ejercicio 14.6. Lanza `productores` hilos; el productor `p`
/// envía por un canal `mpsc` los números `p * 100 + i`, con `i`
/// de 0 a `por_productor - 1`. El hilo que llama es el consumidor
/// y devuelve todo lo recibido, ordenado de menor a mayor.
pub fn producir_y_consumir(
    productores: u32,
    por_productor: u32,
) -> Vec<u32> {
    todo!()
}

/// Ejercicio 14.7. Cuenta cuántos elementos de `datos` son pares
/// con `hilos` hilos de `thread::scope` que comparten un
/// `AtomicUsize`. No use `Mutex` ni sumas parciales.
pub fn contar_pares(datos: &[u32], hilos: usize) -> usize {
    todo!()
}

/// Ejercicio 14.7 (segunda parte). Reparte identificadores únicos
/// desde 1: cada llamada a `siguiente` devuelve un número nuevo,
/// aunque la llamen varios hilos a la vez. Pista: `fetch_add`.
pub struct GeneradorIds {
    proximo: AtomicUsize,
}

impl GeneradorIds {
    /// Crea un generador cuyo primer identificador es 1.
    pub const fn new() -> Self {
        GeneradorIds {
            proximo: AtomicUsize::new(1),
        }
    }

    /// Devuelve un identificador que ningún otro hilo recibió.
    pub fn siguiente(&self) -> usize {
        todo!()
    }
}

impl Default for GeneradorIds {
    fn default() -> Self {
        Self::new()
    }
}

/// Una tarea para el grupo de trabajadores.
pub type Tarea = Box<dyn FnOnce() + Send + 'static>;

/// Reto 14.8. Un grupo fijo de hilos trabajadores que reciben
/// tareas por un canal compartido.
pub struct GrupoTrabajadores {
    emisor: Option<mpsc::Sender<Tarea>>,
    hilos: Vec<thread::JoinHandle<()>>,
}

impl GrupoTrabajadores {
    /// Crea `n` trabajadores (`n > 0`). Todos comparten el
    /// receptor dentro de un `Arc<Mutex<mpsc::Receiver<Tarea>>>`.
    pub fn new(n: usize) -> Self {
        todo!()
    }

    /// Número de trabajadores.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// `true` si el grupo no tiene trabajadores.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Envía `tarea` a algún trabajador libre.
    pub fn ejecutar<F>(&self, tarea: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

/// Reto 14.8 (segunda parte). Al liberarse, el grupo cierra el
/// canal y espera a que cada trabajador termine.
impl Drop for GrupoTrabajadores {
    fn drop(&mut self) {
        // Libere el emisor con `take` y luego haga `join` de
        // cada hilo.
    }
}
