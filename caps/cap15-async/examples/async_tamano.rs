use cap15_async::futuros::ceder;
use std::mem::size_of_val;

async fn sin_estado() -> u32 {
    7
}

async fn con_arreglo() -> usize {
    let datos = [1u8; 1024];
    ceder().await; // `datos` debe sobrevivir a la espera
    datos.len()
}

fn main() {
    let a = size_of_val(&sin_estado());
    let b = size_of_val(&con_arreglo());
    println!("sin_estado:  {a:>5} bytes");
    println!("con_arreglo: {b:>5} bytes");
    println!("pila de un hilo nuevo: {} bytes", 2 * 1024 * 1024);
}
