//! El asignador de marcos en acción, sobre 128 marcos (512 KiB).

use cap17_so::marcos::{AsignadorMarcos, Marco};

fn main() {
    let mut mem = AsignadorMarcos::<2>::new();
    mem.reservar(0, 3); // marcos del firmware y del núcleo
    println!("libres al arrancar: {}", mem.libres());

    let a = mem.asignar().unwrap();
    let b = mem.asignar().unwrap();
    println!("a = {a:?} en {:#07x}", a.direccion());
    println!("b = {b:?} en {:#07x}", b.direccion());

    println!("liberar a:      {:?}", mem.liberar(a));
    println!("liberar a otra: {:?}", mem.liberar(a));
    println!("fuera de rango: {:?}", mem.liberar(Marco(500)));

    // El hueco de `a` se reutiliza antes que los demás.
    println!("siguiente: {:?}", mem.asignar().unwrap());
    println!("libres al final: {}", mem.libres());
}
