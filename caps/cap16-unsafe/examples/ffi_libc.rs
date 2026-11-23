// Llamar a dos funciones de la biblioteca de C del sistema.
use std::ffi::{CString, c_char, c_int};

unsafe extern "C" {
    // abs acepta cualquier int: se declara `safe`.
    safe fn abs(x: c_int) -> c_int;
    // strlen exige un puntero a texto terminado en nulo.
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    println!("abs(-7) = {}", abs(-7));
    let saludo = CString::new("hola, C").expect("sin nulos");
    // SAFETY: `saludo` vive hasta el final de `main` y su
    // puntero señala bytes terminados en nulo.
    let n = unsafe { strlen(saludo.as_ptr()) };
    println!("strlen(\"hola, C\") = {n}");
    // SAFETY: un literal c"..." siempre termina en nulo.
    let m = unsafe { strlen(c"ñandú".as_ptr()) };
    println!("strlen(\"ñandú\") = {m} bytes");
}
