// Ejercicio 4.5: vocales con o sin tilde, en cualquier caja.
pub fn contar_vocales(texto: &str) -> usize {
    texto
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|c| "aeiouáéíóúü".contains(*c))
        .count()
}
