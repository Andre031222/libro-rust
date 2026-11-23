//! Capítulo 7: préstamos y tiempos de vida.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap07-prestamos --example prestamo`.

#[cfg(test)]
mod tests {
    #[test]
    fn el_crate_compila() {
        let s = String::from("Rust");
        let r = &s;
        assert_eq!(r.len(), 4);
    }
}
