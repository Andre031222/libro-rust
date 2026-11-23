//! Un mini fuzzer hecho solo con la biblioteca estándar.

use std::panic;

/// Generador xorshift64: la misma semilla da la misma serie.
pub struct Xorshift(u64);

impl Xorshift {
    pub fn new(semilla: u64) -> Self {
        Xorshift(semilla.max(1)) // el estado no puede ser 0
    }

    pub fn siguiente(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Una entrada aleatoria de 0 a `max` bytes.
    pub fn entrada(&mut self, max: usize) -> Vec<u8> {
        let largo = self.siguiente() as usize % (max + 1);
        (0..largo)
            .map(|_| (self.siguiente() >> 56) as u8)
            .collect()
    }
}

/// ¿Entra `objetivo` en pánico con `entrada`?
pub fn falla(objetivo: fn(&[u8]), entrada: &[u8]) -> bool {
    panic::catch_unwind(|| objetivo(entrada)).is_err()
}

/// Ejecuta `f` sin imprimir los mensajes de pánico.
fn en_silencio<R>(f: impl FnOnce() -> R) -> R {
    let previo = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let r = f();
    panic::set_hook(previo);
    r
}

/// Prueba hasta `n` entradas; devuelve la primera que falla
/// y el número de la iteración.
pub fn fuzzear(
    objetivo: fn(&[u8]),
    semilla: u64,
    n: u64,
) -> Option<(u64, Vec<u8>)> {
    let mut rng = Xorshift::new(semilla);
    en_silencio(|| {
        (1..=n).find_map(|i| {
            let e = rng.entrada(8);
            falla(objetivo, &e).then_some((i, e))
        })
    })
}

/// Quita bytes mientras la entrada siga haciendo fallar.
pub fn minimizar(objetivo: fn(&[u8]), mut e: Vec<u8>) -> Vec<u8> {
    en_silencio(|| {
        let mut i = 0;
        while i < e.len() {
            let mut menor = e.clone();
            menor.remove(i);
            if falla(objetivo, &menor) {
                e = menor; // sigue fallando: nos quedamos
            } else {
                i += 1; // ese byte hacía falta
            }
        }
        e
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::mensajes::{carga, carga_fragil};

    fn fragil(e: &[u8]) {
        let _ = carga_fragil(e);
    }

    fn corregida(e: &[u8]) {
        let _ = carga(e);
    }

    #[test]
    fn el_generador_es_determinista() {
        let mut a = Xorshift::new(7);
        let mut b = Xorshift::new(7);
        assert_eq!(a.entrada(8), b.entrada(8));
        assert!(a.entrada(8).len() <= 8);
    }

    // Una sola prueba: el gancho de pánico es global y las
    // pruebas corren en paralelo.
    #[test]
    fn encuentra_minimiza_y_confirma() {
        let (_, e) = fuzzear(fragil, 42, 1_000_000).unwrap();
        assert!(falla(fragil, &e));
        assert_eq!(minimizar(fragil, e), b"RS");
        assert_eq!(fuzzear(corregida, 42, 200_000), None);
    }
}
