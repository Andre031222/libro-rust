//! Capítulo 1: la crisis de la memoria.
//!
//! Ejemplos en `examples/` y, para comparar, en `c/`.

/// Copia `origen` en un búfer de `N` bytes. A diferencia de `strcpy`
/// en C, nunca escribe fuera del búfer: si no cabe, devuelve `None`.
pub fn copiar_seguro<const N: usize>(
    origen: &[u8],
) -> Option<[u8; N]> {
    if origen.len() > N {
        return None;
    }
    let mut bufer = [0u8; N];
    bufer[..origen.len()].copy_from_slice(origen);
    Some(bufer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copia_lo_que_cabe() {
        let b = copiar_seguro::<8>(b"hola").unwrap();
        assert_eq!(&b[..4], b"hola");
        assert_eq!(&b[4..], &[0, 0, 0, 0]);
    }

    #[test]
    fn rechaza_lo_que_no_cabe() {
        assert!(copiar_seguro::<8>(b"demasiado largo").is_none());
    }

    #[test]
    fn cabe_justo() {
        assert!(copiar_seguro::<4>(b"rust").is_some());
    }
}
