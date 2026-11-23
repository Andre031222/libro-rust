//! Ejercicios del capítulo 18: proyecto integrador.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej18-proyecto`. Las funciones amplían el
//! servicio de `caps/cap18-proyecto` sin modificarlo.

// Parámetros y campos aún no se usan mientras haya `todo!()`.
#![allow(unused_variables, dead_code, unused_imports)]

use cap18_proyecto::formato::validar_estacion;
use cap18_proyecto::{
    Almacen, ErrorPeticion, Medicion, Metodo, Peticion,
    Respuesta, rutas,
};
use std::fmt;
use std::io::{self, Read};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

/// Ejercicio 18.4. Responde a `GET /resumen` con
/// `{"cantidad":N,"media":M}`; si no hay datos, `"media":null`.
pub fn resumen(almacen: &Almacen) -> Respuesta {
    todo!()
}

/// Ejercicio 18.4. Atiende `GET /resumen` y
/// `GET /estaciones/{nombre}` (ejercicio 18.5); el resto lo
/// delega en `rutas::atender`.
pub fn atender_ampliado(
    pet: &Peticion,
    almacen: &Almacen,
) -> Respuesta {
    todo!()
}

/// Ejercicio 18.5. Mediciones de una estación, como arreglo JSON:
/// 400 si el nombre no es válido, 404 si no tiene mediciones.
pub fn por_estacion(
    nombre: &str,
    almacen: &Almacen,
) -> Respuesta {
    todo!()
}

/// Ejercicio 18.6. Lee un cuerpo de `declarada` bytes. Si supera
/// `limite`, falla con `DemasiadoGrande` sin leer nada; si la
/// conexión se corta antes, con `Incompleta`.
pub fn leer_cuerpo<R: Read>(
    lector: &mut R,
    declarada: usize,
    limite: usize,
) -> Result<Vec<u8>, ErrorPeticion> {
    todo!()
}

/// Ejercicio 18.7. Contador de peticiones compartido por los
/// hilos del servidor, sin `Mutex`.
#[derive(Debug, Default)]
pub struct Contador {
    total: AtomicU64,
}

impl Contador {
    /// Suma una petición.
    pub fn registrar(&self) {
        todo!()
    }

    /// Peticiones registradas hasta ahora.
    pub fn total(&self) -> u64 {
        todo!()
    }

    /// Respuesta de `GET /metricas`: `{"peticiones":N}`.
    pub fn metricas(&self) -> Respuesta {
        todo!()
    }
}

/// Ejercicio 18.8 (reto). Fallos al guardar o cargar.
#[derive(Debug)]
pub enum ErrorPersistencia {
    /// Fallo del sistema de archivos.
    Io(io::Error),
    /// Línea `linea` (desde 1) con formato inválido.
    Formato { linea: usize },
}

impl fmt::Display for ErrorPersistencia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(_) => {
                write!(f, "no se pudo usar el archivo")
            }
            Self::Formato { linea } => {
                write!(f, "formato inválido en la línea {linea}")
            }
        }
    }
}

impl std::error::Error for ErrorPersistencia {
    fn source(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Formato { .. } => None,
        }
    }
}

impl From<io::Error> for ErrorPersistencia {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// Ejercicio 18.8 (reto). Guarda una medición por línea:
/// `id<TAB>estacion<TAB>valor`. Escribe primero en un archivo
/// temporal y luego lo renombra.
pub fn guardar(
    almacen: &Almacen,
    ruta: &Path,
) -> Result<(), ErrorPersistencia> {
    todo!()
}

/// Ejercicio 18.8 (reto). Carga lo que `guardar` escribió. Si el
/// archivo no existe, devuelve un almacén vacío. Valida cada
/// estación con `validar_estacion`.
pub fn cargar(ruta: &Path) -> Result<Almacen, ErrorPersistencia> {
    todo!()
}
