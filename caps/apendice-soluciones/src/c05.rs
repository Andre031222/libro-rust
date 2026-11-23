// Ejercicio 5.5: tamaño de un búfer sin desbordar un u32.
pub fn tamano_bufer(
    ancho: u32,
    alto: u32,
    bytes_por_pixel: u32,
) -> Option<u32> {
    ancho.checked_mul(alto)?.checked_mul(bytes_por_pixel)
}
