//! Varios dueños que además modifican: Rc<RefCell<T>>.

use std::cell::RefCell;
use std::rc::Rc;

type Fondo = Rc<RefCell<Vec<i32>>>;

fn depositar(fondo: &Fondo, quien: &str, monto: i32) {
    fondo.borrow_mut().push(monto); // préstamo mutable breve
    println!("{quien} deposita {monto}");
}

fn main() {
    let fondo: Fondo = Rc::new(RefCell::new(Vec::new()));
    let ana = Rc::clone(&fondo); // dos dueños más
    let luis = Rc::clone(&fondo);

    depositar(&ana, "Ana", 50);
    depositar(&luis, "Luis", 20);
    depositar(&ana, "Ana", 5);

    let total: i32 = fondo.borrow().iter().sum();
    println!("movimientos: {:?}", fondo.borrow());
    let n = Rc::strong_count(&fondo);
    println!("total: {total}, dueños: {n}");
}
