//! Un error propio: dos formas de fallar al leer una edad.
use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug)]
pub enum ErrorEdad {
    NoNumero(ParseIntError), // el texto no es un entero
    FueraDeRango(i32),       // es un entero, pero no una edad
}

impl fmt::Display for ErrorEdad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoNumero(_) => write!(f, "no es un número"),
            Self::FueraDeRango(n) => {
                write!(f, "edad absurda: {n}")
            }
        }
    }
}

impl Error for ErrorEdad {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoNumero(e) => Some(e), // la causa original
            Self::FueraDeRango(_) => None,
        }
    }
}

impl From<ParseIntError> for ErrorEdad {
    fn from(e: ParseIntError) -> Self {
        Self::NoNumero(e)
    }
}

/// Lee una edad entre 0 y 150; `?` convierte el error con `From`.
pub fn leer_edad(texto: &str) -> Result<i32, ErrorEdad> {
    let n: i32 = texto.trim().parse()?;
    if !(0..=150).contains(&n) {
        return Err(ErrorEdad::FueraDeRango(n));
    }
    Ok(n)
}
