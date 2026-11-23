//! Futuros escritos a mano: `Ceder` y `Temporizador`.

use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

/// Devuelve `Pending` al primer sondeo y `Ready` al segundo.
pub struct Ceder {
    cedido: bool,
}

/// Cede el turno al ejecutor una sola vez.
pub fn ceder() -> Ceder {
    Ceder { cedido: false }
}

impl Future for Ceder {
    type Output = ();
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<()> {
        if self.cedido {
            return Poll::Ready(());
        }
        self.cedido = true;
        cx.waker().wake_by_ref(); // pide otro sondeo
        Poll::Pending
    }
}

/// Lo que comparten el futuro y el hilo que cuenta el tiempo.
struct Estado {
    listo: bool,
    waker: Option<Waker>,
}

/// Futuro que está listo cuando pasa `espera`.
pub struct Temporizador {
    estado: Arc<Mutex<Estado>>,
}

impl Temporizador {
    pub fn new(espera: Duration) -> Self {
        let estado = Arc::new(Mutex::new(Estado {
            listo: false,
            waker: None,
        }));
        let otro = Arc::clone(&estado);
        thread::spawn(move || {
            thread::sleep(espera);
            let mut e = otro.lock().unwrap();
            e.listo = true;
            if let Some(waker) = e.waker.take() {
                waker.wake(); // avisa al ejecutor
            }
        });
        Temporizador { estado }
    }
}

impl Future for Temporizador {
    type Output = ();
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<()> {
        let mut e = self.estado.lock().unwrap();
        if e.listo {
            Poll::Ready(())
        } else {
            e.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::pin;

    #[test]
    fn ceder_pide_un_segundo_sondeo() {
        let mut cx = Context::from_waker(Waker::noop());
        let mut f = pin!(ceder());
        assert!(f.as_mut().poll(&mut cx).is_pending());
        assert!(f.as_mut().poll(&mut cx).is_ready());
    }
}
