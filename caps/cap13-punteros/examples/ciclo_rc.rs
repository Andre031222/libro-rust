//! Dos nodos que se apuntan entre sí con `Rc`: nunca se liberan.

use std::cell::RefCell;
use std::rc::Rc;

struct Nodo {
    nombre: &'static str,
    otro: RefCell<Option<Rc<Nodo>>>,
}

impl Drop for Nodo {
    fn drop(&mut self) {
        println!("libero {}", self.nombre);
    }
}

fn nodo(nombre: &'static str) -> Rc<Nodo> {
    let otro = RefCell::new(None);
    Rc::new(Nodo { nombre, otro })
}

fn main() {
    let a = nodo("a");
    let b = nodo("b");
    *a.otro.borrow_mut() = Some(Rc::clone(&b)); // a -> b
    *b.otro.borrow_mut() = Some(Rc::clone(&a)); // b -> a

    println!("dueños de a: {}", Rc::strong_count(&a));
    println!("dueños de b: {}", Rc::strong_count(&b));
    drop(a);
    drop(b);
    println!("fin de main: nadie imprimió «libero»");
}
