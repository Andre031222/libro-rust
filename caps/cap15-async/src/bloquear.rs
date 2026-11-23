//! `bloquear`: ejecuta un futuro hasta el final en este hilo.

use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

/// Despertador que reactiva el hilo dormido.
struct DespertarHilo(Thread);

impl Wake for DespertarHilo {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Sondea `futuro` hasta que esté listo y devuelve su valor.
pub fn bloquear<F: Future>(futuro: F) -> F::Output {
    let mut futuro = pin!(futuro);
    let hilo = thread::current();
    let waker = Waker::from(Arc::new(DespertarHilo(hilo)));
    let mut cx = Context::from_waker(&waker);
    loop {
        match futuro.as_mut().poll(&mut cx) {
            Poll::Ready(valor) => return valor,
            Poll::Pending => thread::park(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bloquear;
    use crate::futuros::ceder;

    #[test]
    fn bloquear_devuelve_el_valor() {
        let n = bloquear(async {
            ceder().await;
            41 + 1
        });
        assert_eq!(n, 42);
    }
}
