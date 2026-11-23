//! Un búfer de teclado: la interrupción encola, el núcleo saca.

use cap17_so::cola::ColaCircular;

fn main() {
    // Capacidad fija de 4 teclas: sin montículo.
    let mut teclado = ColaCircular::<char, 4>::new();

    // La "interrupción" llega más rápido de lo que se atiende.
    for tecla in "rust!".chars() {
        match teclado.push(tecla) {
            Ok(()) => println!("encola {tecla:?}"),
            Err(t) => println!("llena: se pierde {t:?}"),
        }
    }

    // El núcleo atiende cuando puede, en orden de llegada.
    while let Some(t) = teclado.pop() {
        print!("{t}");
    }
    println!("  (quedan {})", teclado.len());
}
