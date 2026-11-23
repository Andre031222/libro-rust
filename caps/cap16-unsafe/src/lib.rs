//! Capítulo 16: `unsafe` y FFI con C.
//!
//! Los ejemplos están en `examples/`. Ejecute uno con
//! `cargo run -p cap16-unsafe --example unsafe_punteros`.

use std::ffi::c_int;
use std::slice::from_raw_parts_mut;

/// Divide `v` en dos rebanadas mutables: `[0, mitad)` y
/// `[mitad, len)`.
///
/// # Panics
///
/// Si `mitad > v.len()`.
pub fn partir_mut(
    v: &mut [i32],
    mitad: usize,
) -> (&mut [i32], &mut [i32]) {
    let largo = v.len();
    let p = v.as_mut_ptr();
    assert!(mitad <= largo, "mitad fuera de rango");
    // SAFETY: `p` vale para `largo` elementos de una reserva
    // y `mitad <= largo`: las mitades caben y no se solapan.
    unsafe {
        let resto = p.add(mitad);
        (
            from_raw_parts_mut(p, mitad),
            from_raw_parts_mut(resto, largo - mitad),
        )
    }
}

/// Suma dos enteros de C. Se exporta sin decorar el nombre
/// para que un programa en C la llame como `c16_sumar`.
#[unsafe(no_mangle)]
pub extern "C" fn c16_sumar(a: c_int, b: c_int) -> c_int {
    a.wrapping_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partir_mut_da_dos_mitades() {
        let mut v = [1, 2, 3, 4, 5];
        let (a, b) = partir_mut(&mut v, 2);
        a[0] = 10;
        b[0] = 30;
        assert_eq!(v, [10, 2, 30, 4, 5]);
    }

    #[test]
    fn partir_mut_en_los_extremos() {
        let mut v = [1, 2, 3];
        assert_eq!(partir_mut(&mut v, 0).0.len(), 0);
        assert_eq!(partir_mut(&mut v, 3).1.len(), 0);
    }

    #[test]
    #[should_panic(expected = "mitad fuera de rango")]
    fn partir_mut_rechaza_mitad_grande() {
        let mut v = [1, 2, 3];
        partir_mut(&mut v, 4);
    }

    #[test]
    fn sumar_desde_rust() {
        assert_eq!(c16_sumar(2, 40), 42);
    }
}
