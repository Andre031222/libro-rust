use cap15_async::ejecutor;
use cap15_async::futuros::Temporizador;
use std::time::{Duration, Instant};

async fn esperar(nombre: &str, ms: u64) {
    Temporizador::new(Duration::from_millis(ms)).await;
    println!("{nombre} terminó su espera de {ms} ms");
}

fn main() {
    let inicio = Instant::now();
    let (ejecutor, lanzador) = ejecutor::nuevo();
    lanzador.lanzar(esperar("A", 300));
    lanzador.lanzar(esperar("B", 100));
    lanzador.lanzar(esperar("C", 200));
    drop(lanzador);
    ejecutor.ejecutar();
    // Se redondea a centenas de milisegundos.
    let ms = inicio.elapsed().as_millis();
    println!("total: unos {} ms, no 600", (ms + 50) / 100 * 100);
}
