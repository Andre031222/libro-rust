//! Precios con impuesto, en céntimos.

/// Tasa del impuesto, en por ciento.
pub(crate) const TASA: u64 = 18;

/// Precio con impuesto, redondeado al céntimo.
///
/// ```
/// assert_eq!(cap12_modulos::precio_final(1000), 1180);
/// ```
pub fn precio_final(centimos: u64) -> u64 {
    redondear(centimos * (100 + TASA))
}

/// Divide entre 100 y redondea a la unidad más cercana.
fn redondear(x: u64) -> u64 {
    (x + 50) / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redondear_mitad_hacia_arriba() {
        assert_eq!(redondear(150), 2);
        assert_eq!(redondear(149), 1);
    }
}
