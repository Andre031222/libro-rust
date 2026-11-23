use cap15_async::{bloquear::bloquear, futuros::Temporizador};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// Envuelve un futuro (campo 0) y cuenta sus sondeos (campo 1).
struct Espia<F>(F, u32);

impl<F: Future + Unpin> Future for Espia<F> {
    type Output = F::Output;
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<F::Output> {
        self.1 += 1;
        let r = Pin::new(&mut self.0).poll(cx);
        let s = if r.is_ready() { "Ready" } else { "Pending" };
        println!("sondeo {}: {s}", self.1);
        r
    }
}

fn main() {
    let t = Temporizador::new(Duration::from_millis(50));
    bloquear(Espia(t, 0));
}
