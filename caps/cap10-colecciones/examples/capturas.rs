//! Tres formas de capturar: por `&`, por `&mut` y por valor.

fn main() {
    let saludo = String::from("hola");
    let leer = || println!("leo: {saludo}"); // captura &saludo
    leer();
    leer();
    println!("sigo usando saludo: {saludo}");

    let mut cuenta = 0;
    let mut sumar = || cuenta += 1; // captura &mut cuenta
    sumar();
    sumar();
    println!("cuenta = {cuenta}");

    let nombre = String::from("Ferris");
    let consumir = move || nombre; // move: toma nombre por valor
    let recuperado = consumir(); // FnOnce: se llama una vez
    println!("recuperado: {recuperado}");
}
