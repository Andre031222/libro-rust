//! Una etiqueta permite salir de dos bucles a la vez.

fn main() {
    // Buscar el primer par (fila, columna) cuyo producto sea 12.
    'filas: for fila in 1..10 {
        for col in 1..10 {
            if fila * col == 12 {
                println!("encontrado: {fila} x {col}");
                break 'filas; // sale de los dos bucles
            }
        }
    }
}
