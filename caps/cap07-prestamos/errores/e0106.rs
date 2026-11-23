// Este archivo NO compila a propósito: muestra el error E0106.
fn mas_largo(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", mas_largo("uno", "tres"));
}
