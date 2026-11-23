// Este archivo NO compila a propósito: muestra el error E0382.
fn main() {
    let s1 = String::from("hola");
    let s2 = s1;
    println!("{s1}, {s2}");
}
