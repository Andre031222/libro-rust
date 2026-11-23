//! Variables, mutabilidad, sombreado, constantes y estáticos.

const SEGUNDOS_POR_HORA: u32 = 60 * 60;
static SALUDO: &str = "hola";

fn main() {
    let x = 5; // inmutable
    let mut contador = 0; // mutable
    contador += 1;
    println!("x = {x}, contador = {contador}");

    // Sombreado: un nuevo `x` que oculta al anterior.
    let x = x * 2;
    {
        let x = "texto"; // otro tipo, solo en este bloque
        println!("dentro del bloque: x = {x}");
    }
    println!("fuera del bloque: x = {x}");

    // Sombreado que cambia el tipo: de &str a usize.
    let espacios = "   ";
    let espacios = espacios.len();
    println!("espacios = {espacios}");

    println!("{SALUDO}: una hora tiene {SEGUNDOS_POR_HORA} s");
}
