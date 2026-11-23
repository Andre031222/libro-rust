//! `iter`, `iter_mut` e `into_iter`: prestar, mutar o consumir.

fn main() {
    let mut precios = vec![10.0, 25.5, 4.0];

    // iter: referencias &f64; precios sigue intacto.
    let total: f64 = precios.iter().sum();
    println!("total = {total}");

    // iter_mut: referencias &mut f64; modifica en su sitio.
    for p in precios.iter_mut() {
        *p *= 1.18; // agrega el impuesto
    }
    println!("con impuesto = {precios:.2?}");

    // into_iter: toma la propiedad; después precios no existe.
    let enteros: Vec<i64> =
        precios.into_iter().map(|p| p.round() as i64).collect();
    println!("redondeados = {enteros:?}");
}
