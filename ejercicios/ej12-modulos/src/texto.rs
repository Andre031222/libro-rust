//! Ejercicio 12.4. Un módulo con una función pública y otra
//! privada.

/// Ejercicio 12.4. Indica si `frase` se lee igual al derecho y
/// al revés, sin contar espacios, signos ni mayúsculas. Use
/// `normalizar`.
pub fn es_palindromo(frase: &str) -> bool {
    todo!()
}

/// Ejercicio 12.4. Función privada: deja solo las letras y los
/// dígitos de `frase`, en minúscula. Quite el `allow` cuando
/// `es_palindromo` la use.
#[allow(dead_code)]
fn normalizar(frase: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    // Ejercicio 12.4: escriba aquí al menos dos pruebas de
    // `normalizar`, que es privada y solo se ve desde aquí.
}
