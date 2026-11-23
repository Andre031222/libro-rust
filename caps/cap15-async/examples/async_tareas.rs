use cap15_async::ejecutor;
use cap15_async::futuros::ceder;

async fn trabajar(nombre: &str, pasos: u32) {
    for i in 1..=pasos {
        println!("{nombre}: paso {i}");
        ceder().await; // devuelve el turno al ejecutor
    }
    println!("{nombre}: fin");
}

fn main() {
    let (ejecutor, lanzador) = ejecutor::nuevo();
    lanzador.lanzar(trabajar("A", 2));
    lanzador.lanzar(trabajar("B", 3));
    lanzador.lanzar(trabajar("C", 1));
    drop(lanzador); // ya no habrá tareas nuevas
    ejecutor.ejecutar();
}
