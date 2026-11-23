//! Capítulo 13: punteros inteligentes.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap13-punteros --example rc_conteo`.

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn el_crate_compila() {
        let a = Rc::new(5);
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&b), 2);
    }
}
