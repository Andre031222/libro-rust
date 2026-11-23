//! Rebanadas: vistas prestadas de parte de un String o un Vec.

// Devuelve la primera palabra como una vista del texto original.
fn primera_palabra(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn suma(datos: &[i32]) -> i32 {
    datos.iter().sum()
}

fn main() {
    let frase = String::from("hola mundo");
    let p = primera_palabra(&frase); // &String pasa a &str
    println!("de un String: {p}");
    println!(
        "de un literal: {}",
        primera_palabra("tiempo de vida")
    );

    let numeros = vec![10, 20, 30, 40];
    println!("todo: {}", suma(&numeros));
    println!("[1..3]: {}", suma(&numeros[1..3]));
}
