//! Pasar un valor a una función también lo mueve (o lo copia).

fn tomar(s: String) {
    println!("tomar recibió: {s}");
} // aquí se libera s

fn copiar(n: i32) {
    println!("copiar recibió: {n}");
}

fn main() {
    let texto = String::from("propiedad");
    tomar(texto); // texto se mueve a la función
    // println!("{texto}"); // error E0382

    let numero = 42;
    copiar(numero); // i32 es Copy
    println!("numero sigue disponible: {numero}");
}
