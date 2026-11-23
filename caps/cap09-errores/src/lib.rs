//! Capítulo 9: manejo de errores.
//!
//! Los ejemplos del capítulo están en `examples/`. Ejecute uno
//! con `cargo run -p cap09-errores --example edad`.

pub mod edad;

#[cfg(test)]
mod tests {
    use super::edad::{ErrorEdad, leer_edad};

    #[test]
    fn edad_valida() {
        assert_eq!(leer_edad("42").unwrap(), 42);
    }

    #[test]
    fn edad_fuera_de_rango() {
        assert!(matches!(
            leer_edad("200"),
            Err(ErrorEdad::FueraDeRango(200))
        ));
    }
}
