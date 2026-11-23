// Este archivo NO compila a propósito: muestra el error E0502.
fn main() {
    let mut v = vec![1, 2, 3];
    let primero = &v[0];
    v.push(4);
    println!("{primero}");
}
