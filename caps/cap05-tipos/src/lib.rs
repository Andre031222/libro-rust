//! Capítulo 5: tipos, variables y control de flujo.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap05-tipos --example variables`.

#[cfg(test)]
mod tests {
    #[test]
    fn el_crate_compila() {
        assert_eq!(u8::MAX.checked_add(1), None);
    }
}
