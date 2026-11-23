//! match con patrones literales, guardas y comodín.

fn clasificar(n: Option<i32>) -> &'static str {
    match n {
        None => "nada",
        Some(0) => "cero",
        Some(k) if k < 0 => "negativo", // guarda
        Some(_) => "positivo",
    }
}

fn main() {
    let entradas = [None, Some(0), Some(-3), Some(7)];
    println!("{:?}", entradas.map(clasificar));
}
