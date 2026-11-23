//! Tipos `Copy`: no poseen recursos y se copian bit a bit.

#[derive(Debug, Clone, Copy)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let a = 5;
    let b = a; // copia: a sigue siendo válida
    println!("a = {a}, b = {b}");

    let p = Punto { x: 1, y: 2 };
    let q = p; // Punto es Copy: no hay movimiento
    println!("p = {p:?}, q = {q:?}");
    println!("suma de coordenadas: {}", q.x + q.y);
}
