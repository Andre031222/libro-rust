//! Ejercicios del capítulo 9: manejo de errores.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej09-errores`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Ejercicio 9.4. Convierte `texto` en un `i32`, ignorando los
/// espacios de los extremos. No use `unwrap` ni `expect`.
pub fn parsear_numero(texto: &str) -> Result<i32, ParseIntError> {
    todo!("use trim y parse")
}

/// Error al leer una configuración «clave=valor». Las líneas se
/// numeran desde 1.
#[derive(Debug, PartialEq)]
pub enum ErrorConfig {
    /// La línea no contiene `=`.
    SinIgual { linea: usize },
    /// La clave, a la izquierda de `=`, está vacía.
    ClaveVacia { linea: usize },
}

/// Ejercicio 9.5 (primera parte). Mensajes:
/// «línea N: falta '='» y «línea N: clave vacía».
impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Error for ErrorConfig {}

/// Ejercicio 9.5 (segunda parte). Lee pares «clave=valor», uno
/// por línea; recorta espacios y omite las líneas en blanco.
pub fn leer_config(
    texto: &str,
) -> Result<Vec<(String, String)>, ErrorConfig> {
    todo!()
}

/// Ejercicio 9.6. Lee la configuración con `leer_config` y suma
/// los valores como `i64`. Encadene los dos errores con `?`.
pub fn sumar_valores(texto: &str) -> Result<i64, Box<dyn Error>> {
    todo!()
}

/// Error de un campo de texto vacío (reto 9.7).
#[derive(Debug, PartialEq)]
pub struct CampoVacio;

/// Error de un correo sin `@` (reto 9.7).
#[derive(Debug, PartialEq)]
pub struct CorreoInvalido(pub String);

/// Todos los errores posibles del validador (reto 9.7).
#[derive(Debug, PartialEq)]
pub enum ErrorValidacion {
    Nombre(CampoVacio),
    Edad(ParseIntError),
    Correo(CorreoInvalido),
}

impl From<CampoVacio> for ErrorValidacion {
    fn from(e: CampoVacio) -> Self {
        todo!()
    }
}

impl From<ParseIntError> for ErrorValidacion {
    fn from(e: ParseIntError) -> Self {
        todo!()
    }
}

impl From<CorreoInvalido> for ErrorValidacion {
    fn from(e: CorreoInvalido) -> Self {
        todo!()
    }
}

/// Un usuario ya validado.
#[derive(Debug, PartialEq)]
pub struct Usuario {
    pub nombre: String,
    pub edad: u8,
    pub correo: String,
}

/// Reto 9.7 (auxiliar). Devuelve el nombre recortado o
/// `CampoVacio`.
pub fn validar_nombre(
    nombre: &str,
) -> Result<String, CampoVacio> {
    todo!()
}

/// Reto 9.7 (auxiliar). Devuelve el correo o `CorreoInvalido`.
pub fn validar_correo(
    correo: &str,
) -> Result<String, CorreoInvalido> {
    todo!()
}

/// Reto 9.7. Valida los tres campos en orden (nombre, edad,
/// correo) y devuelve el primer error. Use `?` en cada paso: las
/// implementaciones de `From` hacen la conversión.
pub fn validar(
    nombre: &str,
    edad: &str,
    correo: &str,
) -> Result<Usuario, ErrorValidacion> {
    todo!()
}
