//! Mensajes con el formato `RS`, un byte `n` y `n` bytes.

/// Versión frágil: indexa sin comprobar la longitud.
pub fn carga_fragil(m: &[u8]) -> Option<&[u8]> {
    if !m.starts_with(b"RS") {
        return None;
    }
    let n = m[2] as usize;
    Some(&m[3..3 + n])
}

/// Motivo por el que se rechaza un mensaje.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorMensaje {
    SinFirma,
    Truncado,
}

/// Versión corregida: toda lectura puede fallar y devuelve
/// un error en lugar de entrar en pánico.
pub fn carga(m: &[u8]) -> Result<&[u8], ErrorMensaje> {
    let resto =
        m.strip_prefix(b"RS").ok_or(ErrorMensaje::SinFirma)?;
    let (&n, datos) =
        resto.split_first().ok_or(ErrorMensaje::Truncado)?;
    datos.get(..usize::from(n)).ok_or(ErrorMensaje::Truncado)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn ambas_leen_un_mensaje_valido() {
        let m = b"RS\x03abcZZ";
        assert_eq!(carga_fragil(m), Some(&b"abc"[..]));
        assert_eq!(carga(m), Ok(&b"abc"[..]));
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn la_fragil_cae_con_un_mensaje_corto() {
        carga_fragil(b"RS\x09ab");
    }

    #[test]
    fn la_corregida_devuelve_errores() {
        assert_eq!(
            carga(b"XY\x01a"),
            Err(ErrorMensaje::SinFirma)
        );
        assert_eq!(carga(b"RS"), Err(ErrorMensaje::Truncado));
        assert_eq!(
            carga(b"RS\x09ab"),
            Err(ErrorMensaje::Truncado)
        );
        assert_eq!(carga(b"RS\x00"), Ok(&b""[..]));
    }
}
