// Ejercicio 1.5: suma de dos bytes sin desbordar.
pub fn suma_segura(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}
