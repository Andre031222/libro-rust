//! Tipos que los enunciados daban hechos. No se imprimen en
//! el apéndice: son los mismos de cada crate de ejercicios.

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

/// Errores de `leer_config` (ejercicio 9.5).
#[derive(Debug, PartialEq)]
pub enum ErrorConfig {
    SinIgual { linea: usize },
    ClaveVacia { linea: usize },
}

/// Rasgo de las figuras con área (ejercicios 11.4 y 11.7).
pub trait Area {
    fn area(&self) -> f64;

    fn es_mayor_que(&self, umbral: f64) -> bool {
        self.area() > umbral
    }
}

pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
}

pub struct Circulo {
    pub radio: f64,
}

impl Area for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
}

impl Area for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
}

/// Errores de `Porcentaje` (ejercicios 12.5 a 12.7).
#[derive(Debug, PartialEq)]
pub enum ErrorPorcentaje {
    FueraDeRango,
    TotalCero,
    Formato,
}

/// Ejecutor mínimo del capítulo 15.
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

/// Futuro del ejercicio 15.4: `Pending` en sus primeros `n`
/// sondeos y `Ready(n)` en el siguiente.
pub struct ListoTras {
    n: u32,
    sondeos: u32,
}

impl ListoTras {
    pub fn new(n: u32) -> Self {
        ListoTras { n, sondeos: 0 }
    }
}

impl Future for ListoTras {
    type Output = u32;
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<u32> {
        if self.sondeos == self.n {
            return Poll::Ready(self.n);
        }
        self.sondeos += 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

/// Motivo por el que se rechaza un texto `"ANCHOxALTO"`
/// (ejercicio 19.5).
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorDimension {
    SinSeparador,
    NoEsNumero,
    Cero,
}
