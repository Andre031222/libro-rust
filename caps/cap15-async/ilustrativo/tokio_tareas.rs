// LISTADO ILUSTRATIVO: necesita la dependencia
// tokio = { version = "1", features = ["full"] }.
// No forma parte del workspace y no se compila con él.
use std::time::Duration;
use tokio::time::sleep;

async fn esperar(nombre: &'static str, ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await; // no bloquea el hilo
    nombre
}

#[tokio::main]
async fn main() {
    let a = tokio::spawn(esperar("A", 300)); // tarea nueva
    let b = tokio::spawn(esperar("B", 100));
    let (ra, rb) = (a.await.unwrap(), b.await.unwrap());
    println!("terminaron {ra} y {rb}");
}
