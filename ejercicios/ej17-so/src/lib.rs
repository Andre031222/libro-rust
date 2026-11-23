//! Ejercicios del capítulo 17: Rust y sistemas operativos.
//!
//! Todo el crate es `#![no_std]`: no hay `Vec`, `String` ni
//! `println!`. Reemplace cada `todo!()` por su solución y
//! ejecute `cargo test -p ej17-so`.

#![no_std]
// Parámetros y campos aún no se usan mientras haya `todo!()`.
#![allow(unused_variables, dead_code)]

/// Errores del asignador de marcos.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorMarco {
    FueraDeRango,
    DobleLiberacion,
}

/// Ejercicio 17.4. Mapa de bits con bytes: el byte `i`
/// guarda los marcos `8*i` a `8*i + 7` (bit 0 = el menor).
pub struct MapaMarcos<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> MapaMarcos<N> {
    /// Todos los marcos libres.
    pub const fn new() -> Self {
        MapaMarcos { bytes: [0; N] }
    }

    /// Ocupa el marco libre de número más bajo y lo devuelve.
    pub fn asignar(&mut self) -> Option<usize> {
        todo!()
    }

    /// Libera el marco `n`. Error si no existe o ya estaba libre.
    pub fn liberar(
        &mut self,
        n: usize,
    ) -> Result<(), ErrorMarco> {
        todo!()
    }

    /// Número de marcos libres.
    pub fn libres(&self) -> usize {
        todo!()
    }

    /// Ocupa `k` marcos consecutivos (el primer hueco donde
    /// quepan) y devuelve el número del primero. Con `k == 0`
    /// devuelve `None`.
    pub fn asignar_contiguos(
        &mut self,
        k: usize,
    ) -> Option<usize> {
        todo!()
    }
}

impl<const N: usize> Default for MapaMarcos<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Ejercicio 17.5. Búfer de teclado de `N` bytes que, cuando
/// está lleno, descarta la tecla más antigua para guardar la
/// nueva.
pub struct ColaTeclado<const N: usize> {
    datos: [u8; N],
    inicio: usize,
    largo: usize,
}

impl<const N: usize> ColaTeclado<N> {
    pub const fn new() -> Self {
        ColaTeclado {
            datos: [0; N],
            inicio: 0,
            largo: 0,
        }
    }

    /// Encola `b`. Si la cola estaba llena, devuelve la tecla
    /// descartada.
    pub fn push(&mut self, b: u8) -> Option<u8> {
        todo!()
    }

    /// Saca la tecla más antigua.
    pub fn pop(&mut self) -> Option<u8> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

impl<const N: usize> Default for ColaTeclado<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Datos que interesan a un cargador en una cabecera ELF64.
#[derive(Debug, PartialEq, Eq)]
pub struct InfoElf {
    pub tipo: u16,    // e_type, bytes 16..18
    pub maquina: u16, // e_machine, bytes 18..20 (62 = x86-64)
    pub entrada: u64, // e_entry, bytes 24..32
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorElf {
    Corta,
    NoEsElf,
    NoEs64Bits,
    NoEsLittleEndian,
}

/// Ejercicio 17.6. Lee una cabecera ELF64 little-endian.
/// Firma `7F 45 4C 46`; byte 4 = 2 (64 bits); byte 5 = 1
/// (little-endian). La cabecera ocupa al menos 64 bytes.
pub fn leer_elf(b: &[u8]) -> Result<InfoElf, ErrorElf> {
    todo!()
}

/// Errores del sistema de archivos en memoria.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorFs {
    NombreLargo,
    YaExiste,
    NoExiste,
    SinEntradas,
    SinEspacio,
}

/// Largo máximo de un nombre, en bytes.
pub const MAX_NOMBRE: usize = 16;

/// Una entrada del directorio: nombre, datos y su largo.
#[derive(Clone, Copy)]
struct Archivo<const B: usize> {
    nombre: [u8; MAX_NOMBRE],
    largo_nombre: usize,
    datos: [u8; B],
    largo: usize,
}

/// Ejercicio 17.7 (reto). Hasta `A` archivos de hasta `B`
/// bytes cada uno, en arreglos fijos.
pub struct SistemaArchivos<const A: usize, const B: usize> {
    archivos: [Option<Archivo<B>>; A],
}

impl<const A: usize, const B: usize> SistemaArchivos<A, B> {
    pub const fn new() -> Self {
        SistemaArchivos {
            archivos: [None; A],
        }
    }

    /// Crea un archivo vacío.
    pub fn crear(&mut self, nombre: &str) -> Result<(), ErrorFs> {
        todo!()
    }

    /// Reemplaza el contenido del archivo por `datos`.
    pub fn escribir(
        &mut self,
        nombre: &str,
        datos: &[u8],
    ) -> Result<(), ErrorFs> {
        todo!()
    }

    /// Devuelve el contenido, prestado: sin copiar.
    pub fn leer(&self, nombre: &str) -> Result<&[u8], ErrorFs> {
        todo!()
    }

    /// Borra el archivo y libera su entrada.
    pub fn borrar(
        &mut self,
        nombre: &str,
    ) -> Result<(), ErrorFs> {
        todo!()
    }

    /// Número de archivos.
    pub fn cuantos(&self) -> usize {
        todo!()
    }
}

impl<const A: usize, const B: usize> Default
    for SistemaArchivos<A, B>
{
    fn default() -> Self {
        Self::new()
    }
}
