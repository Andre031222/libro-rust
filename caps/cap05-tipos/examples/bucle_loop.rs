//! loop es una expresión: devuelve el valor de break.

fn main() {
    let mut n = 1;
    let potencia = loop {
        n *= 2;
        if n > 100 {
            break n; // el valor del loop
        }
    };
    println!("primera potencia de 2 mayor que 100: {potencia}");
}
