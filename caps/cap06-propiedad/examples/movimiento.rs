//! Movimiento: asignar un `String` transfiere la propiedad.

fn main() {
    let s1 = String::from("hola");
    println!("s1 = {s1} (len {}, cap {})", s1.len(), s1.capacity());

    let s2 = s1; // la propiedad pasa de s1 a s2
    println!("s2 = {s2}");

    // println!("{s1}"); // error E0382: s1 ya no es válida

    // Al terminar main, solo s2 libera la memoria del montículo:
    // una sola liberación, nunca dos.
}
