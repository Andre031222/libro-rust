//! Respuestas HTTP/1.1.

use crate::formato::escapar_json;
use std::io::{self, Write};

/// Una respuesta lista para enviarse.
#[derive(Debug, Clone, PartialEq)]
pub struct Respuesta {
    pub estado: u16,
    pub tipo: &'static str,
    pub cuerpo: String,
    pub extra: Vec<(&'static str, String)>,
}

impl Respuesta {
    /// Respuesta con cuerpo JSON.
    pub fn json(estado: u16, cuerpo: String) -> Self {
        let tipo = "application/json";
        Self {
            estado,
            tipo,
            cuerpo,
            extra: Vec::new(),
        }
    }

    /// Respuesta con cuerpo de texto plano.
    pub fn texto(estado: u16, cuerpo: &str) -> Self {
        let tipo = "text/plain; charset=utf-8";
        let cuerpo = cuerpo.to_owned();
        Self {
            estado,
            tipo,
            cuerpo,
            extra: Vec::new(),
        }
    }

    /// Error en JSON: `{"error":"..."}`.
    pub fn error(estado: u16, mensaje: &str) -> Self {
        let m = escapar_json(mensaje);
        Self::json(estado, format!(r#"{{"error":"{m}"}}"#))
    }

    /// Añade un campo de cabecera, como `Allow`.
    pub fn con_campo(
        mut self,
        nombre: &'static str,
        v: &str,
    ) -> Self {
        self.extra.push((nombre, v.to_owned()));
        self
    }

    /// Escribe la respuesta completa y pide cerrar la conexión.
    pub fn escribir<W: Write>(
        &self,
        w: &mut W,
    ) -> io::Result<()> {
        let razon = razon(self.estado);
        write!(w, "HTTP/1.1 {} {razon}\r\n", self.estado)?;
        write!(w, "Content-Type: {}\r\n", self.tipo)?;
        write!(w, "Content-Length: {}\r\n", self.cuerpo.len())?;
        write!(w, "Connection: close\r\n")?;
        for (nombre, valor) in &self.extra {
            write!(w, "{nombre}: {valor}\r\n")?;
        }
        write!(w, "\r\n{}", self.cuerpo)?;
        w.flush()
    }
}

/// Frase de razón de cada código (RFC 9110, sección 15).
pub fn razon(estado: u16) -> &'static str {
    match estado {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        413 => "Content Too Large",
        501 => "Not Implemented",
        _ => "Internal Server Error",
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn escribe_linea_de_estado_campos_y_cuerpo() {
        let mut salida = Vec::new();
        Respuesta::texto(200, "ok\n")
            .escribir(&mut salida)
            .unwrap();
        let esperado = "HTTP/1.1 200 OK\r\n\
            Content-Type: text/plain; charset=utf-8\r\n\
            Content-Length: 3\r\nConnection: close\r\n\r\nok\n";
        assert_eq!(String::from_utf8(salida).unwrap(), esperado);
    }

    #[test]
    fn el_error_es_json_valido() {
        let r = Respuesta::error(400, "falta \"valor\"");
        assert_eq!(r.cuerpo, r#"{"error":"falta \"valor\""}"#);
    }
}
