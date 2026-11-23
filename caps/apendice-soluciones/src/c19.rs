// Ejercicio 19.5: cada entrada inválida tiene su error.
use crate::apoyo::ErrorDimension;

pub fn dimensiones(
    texto: &str,
) -> Result<(u32, u32), ErrorDimension> {
    let (a, b) = texto
        .split_once('x')
        .ok_or(ErrorDimension::SinSeparador)?;
    let leer = |t: &str| {
        t.parse::<u32>().map_err(|_| ErrorDimension::NoEsNumero)
    };
    let (ancho, alto) = (leer(a)?, leer(b)?);
    if ancho == 0 || alto == 0 {
        return Err(ErrorDimension::Cero);
    }
    Ok((ancho, alto))
}
