//! Árbol: el padre es dueño de sus hijos, pero no al revés.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Nodo {
    valor: i32,
    padre: RefCell<Weak<Nodo>>,
    hijos: RefCell<Vec<Rc<Nodo>>>,
}

fn nodo(valor: i32) -> Rc<Nodo> {
    Rc::new(Nodo {
        valor,
        ..Default::default()
    })
}

fn valor_del_padre(n: &Nodo) -> Option<i32> {
    n.padre.borrow().upgrade().map(|p| p.valor)
}

fn main() {
    let hoja = nodo(3);
    {
        let rama = nodo(5);
        rama.hijos.borrow_mut().push(Rc::clone(&hoja));
        *hoja.padre.borrow_mut() = Rc::downgrade(&rama);
        println!("padre: {:?}", valor_del_padre(&hoja));
        let f = Rc::strong_count(&rama);
        let d = Rc::weak_count(&rama);
        println!("rama: fuertes = {f}, débiles = {d}");
    } // rama se libera: el Weak no la retiene
    println!("padre luego: {:?}", valor_del_padre(&hoja));
}
