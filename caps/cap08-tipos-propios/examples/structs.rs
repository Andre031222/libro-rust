//! Tres formas de struct: con nombre, tupla y unitario.

#[derive(Debug)]
struct Usuario {
    nombre: String,
    edad: u32,
}

#[derive(Debug)]
struct Metros(f64); // tupla: un tipo nuevo sobre un f64

#[derive(Debug)]
struct Vacio; // unitario: sin campos

fn main() {
    let nombre = String::from("Ada");
    let u = Usuario { nombre, edad: 36 };
    println!("{} tiene {} años", u.nombre, u.edad);
    println!("{u:?}");

    let d = Metros(42.5);
    println!("distancia: {} m", d.0);
    println!("{:?} y {:?}", d, Vacio);
}
