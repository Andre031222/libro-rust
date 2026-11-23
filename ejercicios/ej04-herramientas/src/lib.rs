//! Ejercicios del capítulo 4: herramientas.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej04-herramientas`.

// Los parámetros aún no se usan mientras haya `todo!()`.
#![allow(unused_variables)]

/// Ejercicio 4.4. Devuelve `true` si `anio` es bisiesto en el
/// calendario gregoriano: divisible entre 4, salvo los divisibles
/// entre 100 que no lo son entre 400. Añada un ejemplo de
/// documentación que `cargo test` ejecute.
pub fn es_bisiesto(anio: u32) -> bool {
    todo!()
}

/// Ejercicio 4.5. Cuenta las vocales de `texto`, con o sin tilde
/// y en mayúscula o minúscula (a, e, i, o, u, á, é, í, ó, ú, ü).
pub fn contar_vocales(texto: &str) -> usize {
    todo!()
}

/// Ejercicio 4.6 (reto). Suma los argumentos si todos son números
/// enteros; devuelve `None` si alguno no lo es. Sin argumentos,
/// la suma es 0.
pub fn sumar_argumentos(args: &[String]) -> Option<i64> {
    todo!()
}
