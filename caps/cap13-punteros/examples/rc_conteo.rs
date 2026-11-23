//! Varios dueños de un mismo texto con `Rc`.

use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("compartido"));
    println!("tras crear a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a); // no copia el texto
    println!("tras clonar en b: {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("dentro del bloque: {}", Rc::strong_count(&c));
    } // c sale de ámbito: el conteo baja
    println!("tras el bloque: {}", Rc::strong_count(&a));

    drop(a);
    println!("tras drop(a): {}", Rc::strong_count(&b));
    println!("b sigue leyendo: {b}");
} // b sale de ámbito: conteo 0, se libera el texto
