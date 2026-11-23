//! Formatos de entrada (formulario) y de salida (JSON).

/// Longitud máxima del nombre de una estación.
pub const MAX_ESTACION: usize = 32;

/// Valida un nombre de estación: 1 a 32 caracteres entre
/// minúsculas ASCII, dígitos, `-` y `_`.
pub fn validar_estacion(
    nombre: &str,
) -> Result<(), &'static str> {
    let valido = |b: u8| {
        b.is_ascii_lowercase()
            || b.is_ascii_digit()
            || b == b'-'
            || b == b'_'
    };
    if nombre.is_empty() || nombre.len() > MAX_ESTACION {
        return Err("estacion debe tener de 1 a 32 caracteres");
    }
    if !nombre.bytes().all(valido) {
        return Err("estacion solo admite a-z, 0-9, - y _");
    }
    Ok(())
}

/// Lee `estacion=...&valor=...` del cuerpo de un POST.
pub fn analizar_formulario(
    cuerpo: &[u8],
) -> Result<(String, f64), &'static str> {
    let texto =
        str::from_utf8(cuerpo).map_err(|_| "cuerpo no UTF-8")?;
    let (mut estacion, mut valor) = (None, None);
    for par in texto.trim_end().split('&') {
        match par.split_once('=') {
            Some(("estacion", v)) => estacion = Some(v),
            Some(("valor", v)) => valor = Some(v),
            _ => return Err("campo desconocido"),
        }
    }
    let estacion = estacion.ok_or("falta estacion")?;
    validar_estacion(estacion)?;
    let valor: f64 = valor
        .ok_or("falta valor")?
        .parse()
        .map_err(|_| "valor no es un numero")?;
    if !valor.is_finite() {
        return Err("valor debe ser finito");
    }
    Ok((estacion.to_owned(), valor))
}

/// Escapa un texto para ponerlo entre comillas en JSON.
pub fn escapar_json(texto: &str) -> String {
    let mut salida = String::with_capacity(texto.len());
    for c in texto.chars() {
        match c {
            '"' => salida.push_str("\\\""),
            '\\' => salida.push_str("\\\\"),
            c if c < ' ' => {
                salida.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => salida.push(c),
        }
    }
    salida
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn lee_un_formulario_valido() {
        let r = analizar_formulario(b"valor=-3.5&estacion=puno");
        assert_eq!(r, Ok(("puno".to_owned(), -3.5)));
    }

    #[test]
    fn rechaza_formularios_invalidos() {
        for malo in [
            &b""[..],
            b"estacion=puno",
            b"valor=1",
            b"estacion=Puno&valor=1",
            b"estacion=puno&valor=uno",
            b"estacion=puno&valor=inf",
            b"estacion=puno&valor=NaN",
            b"estacion=puno&valor=1&extra=2",
            b"estacion=\xff&valor=1",
        ] {
            assert!(analizar_formulario(malo).is_err());
        }
    }

    #[test]
    fn la_estacion_tiene_limite() {
        assert!(validar_estacion(&"a".repeat(32)).is_ok());
        assert!(validar_estacion(&"a".repeat(33)).is_err());
    }

    #[test]
    fn escapa_comillas_y_controles() {
        assert_eq!(
            escapar_json("a\"b\\c\n"),
            "a\\\"b\\\\c\\u000a"
        );
    }
}
