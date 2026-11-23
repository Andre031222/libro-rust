use cap15_async::bloquear::bloquear;

async fn saludar(nombre: &str) -> usize {
    println!("   dentro de saludar(\"{nombre}\")");
    nombre.len()
}

fn main() {
    println!("1. llamo a saludar");
    let futuro = saludar("Ada"); // solo crea el futuro
    println!("2. todavía no se ha impreso nada");
    let n = bloquear(futuro); // ahora sí se ejecuta
    println!("3. el ejecutor lo terminó: {n}");
}
