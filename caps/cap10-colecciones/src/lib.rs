//! Capítulo 10: colecciones, iteradores y clausuras.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap10-colecciones --example utf8`.

#[cfg(test)]
mod tests {
    #[test]
    fn el_crate_compila() {
        let v: Vec<i32> =
            (1..=4).filter(|x| x % 2 == 0).collect();
        assert_eq!(v, [2, 4]);
    }
}
