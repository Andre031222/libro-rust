//! Ejercicios del capítulo 19: seguridad sin mitos.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej19-seguridad`. No hace falta `unsafe`
//! (ejercicio 19.2: añada aquí `#![forbid(unsafe_code)]`).

// Parámetros aún sin usar mientras haya `todo!()`.
#![allow(unused_variables)]

use std::path::{Path, PathBuf};

/// Ejercicio 19.4. Une `nombre` a `base` solo si el
/// resultado queda dentro de `base`. Acepta `.` y también
/// `..` mientras no suba por encima de `base`:
/// `"a/../b.txt"` da `base/b.txt`, y `"a/../../x"` da
/// `None`. Rechaza rutas absolutas y nombres vacíos. El
/// resultado no debe contener `.` ni `..`.
pub fn ruta_dentro(base: &Path, nombre: &str) -> Option<PathBuf> {
    todo!()
}

/// Motivo por el que se rechaza un texto `"ANCHOxALTO"`.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorDimension {
    /// Falta la `x` que separa los dos números.
    SinSeparador,
    /// Alguna de las dos partes no es un `u32`.
    NoEsNumero,
    /// El ancho o el alto vale cero.
    Cero,
}

/// La versión frágil que hay que reemplazar: entra en
/// pánico con cualquier texto mal formado.
pub fn dimensiones_fragil(texto: &str) -> (u32, u32) {
    let (a, b) = texto.split_once('x').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

/// Ejercicio 19.5. Igual que `dimensiones_fragil`, pero sin
/// `unwrap`: devuelve un error para cada entrada inválida.
pub fn dimensiones(
    texto: &str,
) -> Result<(u32, u32), ErrorDimension> {
    todo!()
}

/// Ejercicio 19.6. Longitud de un paquete que precede cada
/// campo con su longitud en 2 bytes: la suma de `2 + len`
/// de todos los campos. `None` si un campo o el total no
/// caben en un `u16`. Use aritmética comprobada.
pub fn longitud_paquete(campos: &[&[u8]]) -> Option<u16> {
    todo!()
}

/// Generador xorshift64, el mismo del capítulo.
pub struct Xorshift(pub u64);

impl Xorshift {
    pub fn siguiente(&mut self) -> u64 {
        let mut x = self.0.max(1);
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

/// Analizador TLV (tipo, longitud, valor) con un error
/// sembrado. Cada campo es un byte de tipo, un byte de
/// longitud `n` y `n` bytes de valor.
pub fn analizar_tlv(e: &[u8]) -> Vec<(u8, &[u8])> {
    let mut campos = Vec::new();
    let mut pos = 0;
    while pos + 2 <= e.len() {
        let tipo = e[pos];
        let largo = e[pos + 1] as usize;
        if pos + largo > e.len() {
            break;
        }
        campos.push((tipo, &e[pos + 2..pos + 2 + largo]));
        pos += 2 + largo;
    }
    campos
}

/// Motivo por el que se rechaza una entrada TLV.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorTlv {
    /// La entrada termina en medio de un campo.
    Truncado,
}

/// Reto 19.7 (a). Genera hasta `n` entradas aleatorias de
/// 0 a 8 bytes con `Xorshift(semilla)` y devuelve la primera
/// con la que `objetivo` entra en pánico. No debe imprimir
/// los mensajes de pánico.
pub fn buscar_fallo(
    objetivo: fn(&[u8]),
    semilla: u64,
    n: u64,
) -> Option<Vec<u8>> {
    todo!()
}

/// Reto 19.7 (b). Reduce `entrada`, que hace fallar a
/// `objetivo`, hasta que quitar cualquier byte la haga pasar.
pub fn reducir(objetivo: fn(&[u8]), entrada: Vec<u8>) -> Vec<u8> {
    todo!()
}

/// Reto 19.7 (c). Versión corregida de `analizar_tlv`:
/// nunca entra en pánico y rechaza las entradas truncadas,
/// incluido un byte suelto al final.
pub fn analizar_tlv_corregido(
    e: &[u8],
) -> Result<Vec<(u8, &[u8])>, ErrorTlv> {
    todo!()
}
