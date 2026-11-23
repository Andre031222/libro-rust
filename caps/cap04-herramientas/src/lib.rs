//! Capítulo 4: biblioteca mínima para practicar `cargo test`.

/// Devuelve el promedio de `datos`, o `None` si está vacío.
///
/// ```
/// use cap04_herramientas::promedio;
///
/// assert_eq!(promedio(&[2.0, 4.0]), Some(3.0));
/// assert_eq!(promedio(&[]), None);
/// ```
pub fn promedio(datos: &[f64]) -> Option<f64> {
    if datos.is_empty() {
        return None;
    }
    let suma: f64 = datos.iter().sum();
    Some(suma / datos.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promedio_de_tres_valores() {
        assert_eq!(promedio(&[1.0, 2.0, 6.0]), Some(3.0));
    }
}
