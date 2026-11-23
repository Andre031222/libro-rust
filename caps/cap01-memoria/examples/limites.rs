//! Rust comprueba los límites: leer fuera de un arreglo nunca
//! devuelve basura.

fn main() {
    let datos = [10, 20, 30];

    // get devuelve Option: el caso fuera de límites es visible.
    match datos.get(7) {
        Some(v) => println!("valor: {v}"),
        None => println!("índice 7 fuera de límites"),
    }

    // Con [] el programa se detiene de forma controlada (panic).
    // Aquí capturamos el pánico solo para mostrar su mensaje.
    std::panic::set_hook(Box::new(|info| {
        println!("pánico: {info}")
    }));
    let i = datos.len() + 4;
    let r = std::panic::catch_unwind(|| datos[i]);
    println!("¿acceso [7] permitido? {}", r.is_ok());
}
