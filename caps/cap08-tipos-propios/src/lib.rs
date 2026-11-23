//! Capítulo 8: structs, enums y coincidencia de patrones.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap08-tipos-propios --example structs`.

#[cfg(test)]
mod tests {
    #[test]
    fn el_crate_compila() {
        let x: Option<i32> = Some(4);
        assert_eq!(x.map(|n| n * 2), Some(8));
    }
}
