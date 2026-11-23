//! Un pánico: acceder fuera de los límites de un arreglo.

fn main() {
    let v = [10, 20, 30];
    // El índice se conoce en ejecución: sin argumentos vale 3.
    let i = std::env::args().count() + 2;
    println!("antes del pánico");
    let x = v[i]; // Rust comprueba el límite y entra en pánico
    println!("nunca se imprime: {x}");
}
