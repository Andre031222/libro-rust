//! Lee los argumentos de la línea de comandos.
//! Ejecute: cargo run -p cap04-herramientas --example argumentos
//!          -- hola 42 -7

use std::env;

fn main() {
    // El primer elemento es la ruta del propio programa.
    let args: Vec<String> = env::args().collect();
    println!("programa: {}", args[0]);
    println!("recibí {} argumento(s)", args.len() - 1);

    for (i, arg) in args.iter().enumerate().skip(1) {
        match arg.parse::<i64>() {
            Ok(n) => println!("  {i}: {arg} es el número {n}"),
            Err(_) => println!("  {i}: {arg} es texto"),
        }
    }
}
