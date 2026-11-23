async fn guardar(dato: u32) {
    println!("guardado: {dato}");
}

async fn procesar() {
    guardar(7); // falta .await
}

fn main() {
    let _ = procesar();
}
