// Ejercicio 9.5: mensajes legibles y lectura de «clave=valor».
use crate::apoyo::ErrorConfig;
use std::fmt;

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SinIgual { linea } => {
                write!(f, "línea {linea}: falta '='")
            }
            Self::ClaveVacia { linea } => {
                write!(f, "línea {linea}: clave vacía")
            }
        }
    }
}

impl std::error::Error for ErrorConfig {}

pub fn leer_config(
    texto: &str,
) -> Result<Vec<(String, String)>, ErrorConfig> {
    let mut pares = Vec::new();
    for (i, linea) in texto.lines().enumerate() {
        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }
        let (clave, valor) = linea
            .split_once('=')
            .ok_or(ErrorConfig::SinIgual { linea: i + 1 })?;
        let clave = clave.trim();
        if clave.is_empty() {
            return Err(ErrorConfig::ClaveVacia { linea: i + 1 });
        }
        pares.push((clave.to_string(), valor.trim().to_string()));
    }
    Ok(pares)
}
