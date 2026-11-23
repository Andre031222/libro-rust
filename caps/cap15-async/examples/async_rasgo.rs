use cap15_async::{bloquear::bloquear, futuros::ceder};

trait Sensor {
    async fn leer(&self) -> u32;
}

struct Remoto(u32);

impl Sensor for Remoto {
    async fn leer(&self) -> u32 {
        ceder().await; // simula una espera de red
        self.0 * 10
    }
}

async fn sumar(a: &impl Sensor, b: &impl Sensor) -> u32 {
    a.leer().await + b.leer().await
}

fn main() {
    let total = bloquear(sumar(&Remoto(1), &Remoto(4)));
    println!("total = {total}");
}
