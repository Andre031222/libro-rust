//! Errores al leer una petición HTTP.

use std::error::Error;
use std::fmt;
use std::io;

/// Todo lo que puede salir mal al leer una petición.
#[derive(Debug)]
pub enum ErrorPeticion {
    /// La conexión se cerró antes de terminar la petición.
    Incompleta,
    /// La petición no respeta la sintaxis de HTTP/1.1.
    Malformada(&'static str),
    /// Método o codificación que el servicio no ofrece.
    NoImplementado(&'static str),
    /// La cabecera o el cuerpo superan su límite.
    DemasiadoGrande,
    /// El cliente no terminó a tiempo.
    TiempoAgotado,
    /// Cualquier otro fallo de entrada y salida.
    Io(io::Error),
}

impl ErrorPeticion {
    /// Código de estado HTTP con que se responde.
    pub fn estado(&self) -> u16 {
        match self {
            Self::Incompleta | Self::Malformada(_) => 400,
            Self::TiempoAgotado => 408,
            Self::DemasiadoGrande => 413,
            Self::NoImplementado(_) => 501,
            Self::Io(_) => 500,
        }
    }
}

impl From<io::Error> for ErrorPeticion {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::UnexpectedEof => Self::Incompleta,
            // Unix da WouldBlock; Windows, TimedOut.
            io::ErrorKind::WouldBlock
            | io::ErrorKind::TimedOut => Self::TiempoAgotado,
            _ => Self::Io(e),
        }
    }
}

impl fmt::Display for ErrorPeticion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Incompleta => write!(f, "petición incompleta"),
            Self::Malformada(que) => {
                write!(f, "petición malformada: {que}")
            }
            Self::NoImplementado(que) => {
                write!(f, "no implementado: {que}")
            }
            Self::DemasiadoGrande => {
                write!(f, "petición demasiado grande")
            }
            Self::TiempoAgotado => write!(f, "tiempo agotado"),
            // La causa se expone con `source`, no aquí.
            Self::Io(_) => write!(f, "error de E/S"),
        }
    }
}

impl Error for ErrorPeticion {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}
