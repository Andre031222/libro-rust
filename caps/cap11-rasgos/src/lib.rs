//! Capítulo 11: rasgos y genéricos.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap11-rasgos --example genericos`.

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    fn texto<T: Display>(x: T) -> String {
        format!("<{x}>")
    }

    #[test]
    fn el_crate_compila() {
        assert_eq!(texto(7), "<7>");
        assert_eq!(texto("rasgo"), "<rasgo>");
    }
}
