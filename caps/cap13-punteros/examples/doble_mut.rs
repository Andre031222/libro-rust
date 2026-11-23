//! Las reglas de préstamo, comprobadas en ejecución.

use std::cell::RefCell;

fn main() {
    let saldo = RefCell::new(100); // `saldo` no es `mut`
    *saldo.borrow_mut() -= 30; // préstamo mutable breve
    println!("saldo: {}", saldo.borrow());

    let mut primero = saldo.borrow_mut(); // vive hasta el final
    *primero += 5;
    // `try_borrow_mut` informa el conflicto sin detenerse.
    println!("intento: {:?}", saldo.try_borrow_mut());

    // `borrow_mut` no puede devolver un error: entra en pánico.
    let mut segundo = saldo.borrow_mut();
    *segundo += 10;
    println!("nunca se imprime");
}
