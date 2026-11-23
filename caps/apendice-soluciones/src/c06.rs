// Ejercicio 6.5: `a` es dueña de su búfer y puede crecer en él.
pub fn concatenar(mut a: String, b: &str) -> String {
    a.push_str(b);
    a
}

// Ejercicio 6.7: la cadena que no se devuelve se libera al
// salir de la función.
pub fn mover_mayor(a: String, b: String) -> String {
    if b.len() > a.len() { b } else { a }
}
