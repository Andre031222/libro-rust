//! El ejemplo clásico de tiempos de vida.

// El resultado vive tanto como la más corta de las dos entradas.
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("préstamo");
    let resultado;
    {
        let b = String::from("vida");
        resultado = mas_largo(&a, &b);
        println!("dentro: {resultado}"); // b todavía existe
    }
    println!("a sigue viva: {a}");
}
