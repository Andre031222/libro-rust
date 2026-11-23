//! Lectura y análisis de peticiones HTTP/1.1.

use crate::error::ErrorPeticion;
use std::io::Read;

/// Tamaño máximo de la cabecera, en bytes.
pub const LIMITE_CABECERA: usize = 8 * 1024;
/// Tamaño máximo del cuerpo, en bytes.
pub const LIMITE_CUERPO: usize = 64 * 1024;

/// Los métodos que el servicio entiende.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metodo {
    Get,
    Post,
}

impl Metodo {
    /// Nombre del método tal como se escribe en HTTP.
    pub fn nombre(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
        }
    }
}

/// Una petición ya validada.
#[derive(Debug, PartialEq, Eq)]
pub struct Peticion {
    pub metodo: Metodo,
    pub ruta: String,
    pub campos: Vec<(String, String)>,
    pub cuerpo: Vec<u8>,
}

impl Peticion {
    /// Valor de un campo; el nombre no distingue mayúsculas.
    pub fn campo(&self, nombre: &str) -> Option<&str> {
        self.campos
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(nombre))
            .map(|(_, v)| v.as_str())
    }

    /// Longitud del cuerpo según `Content-Length`.
    pub fn longitud_cuerpo(
        &self,
    ) -> Result<usize, ErrorPeticion> {
        if self.campo("Transfer-Encoding").is_some() {
            return Err(ErrorPeticion::NoImplementado(
                "Transfer-Encoding",
            ));
        }
        let Some(valor) = self.campo("Content-Length") else {
            return Ok(0);
        };
        // Solo dígitos: `parse` aceptaría también «+5».
        if valor.is_empty()
            || !valor.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(ErrorPeticion::Malformada(
                "Content-Length",
            ));
        }
        // Un número enorme no cabe en usize: también es excesivo.
        valor.parse().map_err(|_| ErrorPeticion::DemasiadoGrande)
    }
}

/// Analiza la línea de petición: `MÉTODO RUTA HTTP/1.1`.
fn analizar_linea(
    linea: &str,
) -> Result<(Metodo, &str), ErrorPeticion> {
    let partes: Vec<&str> = linea.split(' ').collect();
    let [metodo, ruta, version] = partes[..] else {
        return Err(ErrorPeticion::Malformada("línea"));
    };
    let metodo = match metodo {
        "GET" => Metodo::Get,
        "POST" => Metodo::Post,
        _ => return Err(ErrorPeticion::NoImplementado("método")),
    };
    if version != "HTTP/1.1" {
        return Err(ErrorPeticion::Malformada("versión"));
    }
    // Solo ASCII visible: nada de espacios ni de controles.
    if !ruta.starts_with('/')
        || !ruta.bytes().all(|b| b.is_ascii_graphic())
    {
        return Err(ErrorPeticion::Malformada("ruta"));
    }
    Ok((metodo, ruta))
}

/// Analiza el texto de la cabecera, sin la línea vacía final.
/// Es una función pura: no lee de la red.
pub fn analizar_cabecera(
    texto: &str,
) -> Result<Peticion, ErrorPeticion> {
    let mut lineas = texto.split("\r\n");
    let primera = lineas.next().unwrap_or_default();
    let (metodo, ruta) = analizar_linea(primera)?;
    let mut campos = Vec::new();
    for linea in lineas {
        let Some((nombre, valor)) = linea.split_once(':') else {
            return Err(ErrorPeticion::Malformada("campo"));
        };
        if nombre.is_empty() || nombre.contains([' ', '\t']) {
            return Err(ErrorPeticion::Malformada("campo"));
        }
        campos.push((nombre.to_owned(), valor.trim().to_owned()));
    }
    let ruta = ruta.to_owned();
    let cuerpo = Vec::new();
    Ok(Peticion {
        metodo,
        ruta,
        campos,
        cuerpo,
    })
}

/// Lee hasta el final de la cabecera (`\r\n\r\n`). Devuelve
/// los bytes leídos y dónde termina la cabecera.
fn leer_cabecera<R: Read>(
    lector: &mut R,
) -> Result<(Vec<u8>, usize), ErrorPeticion> {
    let mut datos = Vec::new();
    let mut trozo = [0u8; 1024];
    loop {
        let fin = datos.windows(4).position(|v| v == b"\r\n\r\n");
        match fin {
            Some(fin) if fin <= LIMITE_CABECERA => {
                return Ok((datos, fin));
            }
            Some(_) => {
                return Err(ErrorPeticion::DemasiadoGrande);
            }
            None if datos.len() > LIMITE_CABECERA => {
                return Err(ErrorPeticion::DemasiadoGrande);
            }
            None => {}
        }
        let n = lector.read(&mut trozo)?;
        if n == 0 {
            return Err(ErrorPeticion::Incompleta);
        }
        datos.extend_from_slice(&trozo[..n]);
    }
}

/// Lee una petición completa, con límites de tamaño.
pub fn leer_peticion<R: Read>(
    lector: &mut R,
) -> Result<Peticion, ErrorPeticion> {
    let (mut datos, fin) = leer_cabecera(lector)?;
    let texto = str::from_utf8(&datos[..fin])
        .map_err(|_| ErrorPeticion::Malformada("UTF-8"))?;
    let mut peticion = analizar_cabecera(texto)?;
    let largo = peticion.longitud_cuerpo()?;
    if largo > LIMITE_CUERPO {
        return Err(ErrorPeticion::DemasiadoGrande);
    }
    // Lo que llegó tras la cabecera ya es parte del cuerpo.
    let mut cuerpo = datos.split_off(fin + 4);
    cuerpo.truncate(largo);
    let leido = cuerpo.len();
    cuerpo.resize(largo, 0);
    lector.read_exact(&mut cuerpo[leido..])?;
    peticion.cuerpo = cuerpo;
    Ok(peticion)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn leer(texto: &[u8]) -> Result<Peticion, ErrorPeticion> {
        leer_peticion(&mut &texto[..])
    }

    #[test]
    fn analiza_una_peticion_get() {
        let p = analizar_cabecera(
            "GET /mediciones HTTP/1.1\r\nHost: ejemplo",
        )
        .unwrap();
        assert_eq!(p.metodo, Metodo::Get);
        assert_eq!(p.ruta, "/mediciones");
        assert_eq!(p.campo("host"), Some("ejemplo"));
    }

    #[test]
    fn rechaza_lineas_mal_formadas() {
        for malo in [
            "",
            "GET /",
            "GET / HTTP/1.1 extra",
            "GET / HTTP/2",
            "GET sin-barra HTTP/1.1",
            "GET / HTTP/1.1\r\nsin dos puntos",
            "GET / HTTP/1.1\r\nmal nombre: x",
        ] {
            assert!(matches!(
                analizar_cabecera(malo),
                Err(ErrorPeticion::Malformada(_))
            ));
        }
    }

    #[test]
    fn metodo_desconocido_no_esta_implementado() {
        let e = analizar_cabecera("PUT / HTTP/1.1").unwrap_err();
        assert_eq!(e.estado(), 501);
    }

    #[test]
    fn lee_el_cuerpo_segun_content_length() {
        let p = leer(
            b"POST /m HTTP/1.1\r\nContent-Length: 4\r\n\r\nhola",
        )
        .unwrap();
        assert_eq!(p.cuerpo, b"hola");
    }

    #[test]
    fn content_length_solo_admite_digitos() {
        for valor in ["+4", "-1", "4x", "cuatro", ""] {
            let texto = format!(
                "POST /m HTTP/1.1\r\n\
                 Content-Length:{valor}\r\n\r\n"
            );
            let e = leer(texto.as_bytes()).unwrap_err();
            assert_eq!(e.estado(), 400, "valor {valor:?}");
        }
    }

    #[test]
    fn aplica_los_limites_de_tamano() {
        let largo = LIMITE_CUERPO + 1;
        let texto = format!(
            "POST /m HTTP/1.1\r\nContent-Length: {largo}\r\n\r\n"
        );
        let e = leer(texto.as_bytes()).unwrap_err();
        assert_eq!(e.estado(), 413);
        let enorme = vec![b'a'; LIMITE_CABECERA + 10];
        assert_eq!(leer(&enorme).unwrap_err().estado(), 413);
    }

    #[test]
    fn detecta_peticiones_cortadas() {
        let corta =
            b"POST /m HTTP/1.1\r\nContent-Length: 9\r\n\r\nab";
        for texto in [&b"GET / HTTP/1.1\r\n"[..], &corta[..]] {
            assert!(matches!(
                leer(texto),
                Err(ErrorPeticion::Incompleta)
            ));
        }
    }
}
