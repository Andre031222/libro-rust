//! Prestar un valor: usarlo sin tomar su propiedad.

// Referencia compartida: puede leer, no modificar.
// (Más adelante se verá por qué &str es mejor parámetro.)
#[allow(clippy::ptr_arg)]
fn medir(s: &String) -> usize {
    s.len()
} // s sale de ámbito, pero no era dueña: nada se libera

// Referencia mutable: puede modificar el valor prestado.
fn exclamar(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut texto = String::from("hola");

    let n = medir(&texto); // se presta para leer
    println!("{texto} mide {n} bytes");

    exclamar(&mut texto); // se presta para escribir
    println!("ahora: {texto}");
}
