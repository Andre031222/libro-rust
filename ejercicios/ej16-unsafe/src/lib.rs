//! Ejercicios del capítulo 16: `unsafe` y FFI con C.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej16-unsafe`. Escriba un comentario
//! `// SAFETY:` sobre cada bloque `unsafe` que agregue.

// Parámetros y campos aún no se usan mientras haya `todo!()`.
#![allow(unused_variables, dead_code)]

use std::ffi::{CStr, c_char};
use std::mem::ManuallyDrop;

unsafe extern "C" {
    /// `strlen` de la biblioteca de C del sistema.
    fn strlen(s: *const c_char) -> usize;
}

/// Ejercicio 16.4. Divide `v` en `[0, mitad)` y
/// `[mitad, len)` sin usar `split_at_mut`. Entra en pánico
/// si `mitad > v.len()`. Pista: `as_mut_ptr` y
/// `std::slice::from_raw_parts_mut`.
pub fn partir_en<T>(
    v: &mut [T],
    mitad: usize,
) -> (&mut [T], &mut [T]) {
    todo!()
}

/// Ejercicio 16.5. Longitud de `s` en bytes, sin contar el
/// terminador, calculada con `strlen`.
pub fn longitud_c(s: &CStr) -> usize {
    todo!()
}

/// Ejercicio 16.5. Longitud de `s` en bytes calculada con
/// `strlen`, o `None` si `s` contiene un byte nulo.
/// Pista: `CString::new`.
pub fn longitud_texto(s: &str) -> Option<usize> {
    todo!()
}

/// Ejercicio 16.6. Versión corregida y documentada de
/// `ultimo` (listado del capítulo): `None` si `v` está vacío.
/// Mantenga `get_unchecked` y justifique su uso.
pub fn ultimo(v: &[i32]) -> Option<i32> {
    todo!()
}

/// Ejercicio 16.7 (reto). Cola circular de capacidad fija
/// sobre un búfer reservado a mano.
pub struct BuferCircular<T> {
    datos: *mut T,  // primer elemento del búfer
    reserva: usize, // capacidad real de la reserva
    capacidad: usize,
    inicio: usize, // índice del elemento más antiguo
    largo: usize,
}

impl<T> BuferCircular<T> {
    /// Crea un búfer vacío para `capacidad` elementos.
    /// Entra en pánico si `capacidad` es cero.
    pub fn new(capacidad: usize) -> Self {
        assert!(capacidad > 0, "capacidad cero");
        let mut v =
            ManuallyDrop::new(Vec::with_capacity(capacidad));
        BuferCircular {
            datos: v.as_mut_ptr(),
            reserva: v.capacity(),
            capacidad,
            inicio: 0,
            largo: 0,
        }
    }

    /// Agrega `valor` al final. Si está lleno, lo devuelve
    /// en `Err` sin modificar el búfer.
    pub fn push(&mut self, valor: T) -> Result<(), T> {
        todo!()
    }

    /// Quita y devuelve el elemento más antiguo.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Referencia al elemento más antiguo, sin quitarlo.
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    /// Número de elementos guardados.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// `true` si no hay elementos.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// `true` si no cabe ningún elemento más.
    pub fn is_full(&self) -> bool {
        todo!()
    }
}

impl<T> Drop for BuferCircular<T> {
    /// Ejercicio 16.7: libere los elementos que queden y,
    /// después, la reserva con `Vec::from_raw_parts`. Tal como
    /// está, el búfer pierde memoria.
    fn drop(&mut self) {}
}
