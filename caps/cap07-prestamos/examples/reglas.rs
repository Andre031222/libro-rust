//! Muchos lectores o un solo escritor, nunca ambos a la vez.

fn main() {
    let mut v = vec![1, 2, 3];

    let a = &v; // primer lector
    let b = &v; // segundo lector: permitido
    println!("lectores: {a:?} {b:?}"); // último uso de a y b

    let c = &mut v; // escritor: los lectores ya terminaron
    c.push(4);
    println!("escritor: {c:?}");

    println!("final: {v:?}"); // c ya no se usa: v queda libre
}
