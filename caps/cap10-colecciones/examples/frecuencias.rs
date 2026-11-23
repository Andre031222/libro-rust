//! Contar palabras con `HashMap` y la API `entry`.
use std::collections::HashMap;

fn main() {
    let texto = "el gato y el perro y el loro";
    let mut cuenta: HashMap<&str, u32> = HashMap::new();

    for palabra in texto.split_whitespace() {
        // entry busca la clave una vez e inserta 0 si falta.
        *cuenta.entry(palabra).or_insert(0) += 1;
    }

    // HashMap no tiene orden: se ordena para imprimir.
    let mut pares: Vec<_> = cuenta.into_iter().collect();
    pares.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (palabra, n) in pares {
        println!("{palabra:>5}: {n}");
    }
}
