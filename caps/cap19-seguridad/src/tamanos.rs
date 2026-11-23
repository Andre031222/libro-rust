//! Tamaño en bytes de una imagen RGBA (4 bytes por píxel).

/// Límite que el servicio acepta: 16 MiB.
pub const LIMITE: u32 = 16 * 1024 * 1024;

/// Versión ingenua: la multiplicación puede desbordarse.
pub fn bytes_ingenuo(ancho: u32, alto: u32) -> u32 {
    ancho * alto * 4
}

/// ¿Cabe la imagen en el límite? Usa la versión ingenua.
pub fn acepta_ingenuo(ancho: u32, alto: u32) -> bool {
    bytes_ingenuo(ancho, alto) <= LIMITE
}

/// Versión corregida: `None` si el producto no cabe en `u32`.
pub fn bytes(ancho: u32, alto: u32) -> Option<u32> {
    ancho.checked_mul(alto)?.checked_mul(4)
}

/// ¿Cabe la imagen en el límite? Un desbordamiento la rechaza.
pub fn acepta(ancho: u32, alto: u32) -> bool {
    bytes(ancho, alto).is_some_and(|n| n <= LIMITE)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn tamanos_normales() {
        assert_eq!(bytes(1920, 1080), Some(8_294_400));
        assert!(acepta(1920, 1080));
        assert!(!acepta(4096, 4097));
    }

    #[test]
    fn el_desbordamiento_se_rechaza() {
        assert_eq!(bytes(65_536, 16_384), None);
        assert!(!acepta(65_536, 16_384));
    }

    // En debug la versión ingenua entra en pánico; en
    // release (`cargo test --release`) da la vuelta a 0.
    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "overflow")
    )]
    fn la_ingenua_acepta_4_gib() {
        assert!(acepta_ingenuo(65_536, 16_384));
    }
}
