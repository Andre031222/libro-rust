//! Capítulo 6: propiedad (*ownership*).
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno con
//! `cargo run -p cap06-propiedad --example movimiento`.

#[cfg(test)]
mod tests {
    #[test]
    fn el_crate_compila() {
        let s = String::from("Rust");
        assert_eq!(s.len(), 4);
    }
}
