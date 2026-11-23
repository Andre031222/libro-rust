//! Bloques e if como expresiones con valor.

fn cuadrado(x: i32) -> i32 {
    x * x // sin punto y coma: es el valor de la función
}

fn main() {
    let y = {
        let x = 3;
        x + 1 // el bloque vale 4
    };
    println!("y = {y}, cuadrado(y) = {}", cuadrado(y));

    let temperatura = 31;
    let estado = if temperatura > 30 {
        "calor"
    } else {
        "templado"
    };
    println!("estado = {estado}");
}
