//! Lectura de una cabecera binaria de 16 bytes, sin copiar.
//!
//! Formato (enteros en little-endian):
//! bytes 0..4 firma `RSO1` | 4..6 versión | 6..8 banderas |
//! 8..12 desplazamiento de entrada | 12..16 tamaño del cuerpo.

pub const FIRMA: [u8; 4] = *b"RSO1";
pub const LARGO: usize = 16;

#[derive(Debug, PartialEq, Eq)]
pub struct Cabecera {
    pub version: u16,
    pub banderas: u16,
    pub entrada: u32,
    pub tamano: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCabecera {
    Corta,
    FirmaInvalida,
    EntradaFuera,
}

fn u16_le(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}

fn u32_le(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

/// Valida la cabecera y devuelve también el cuerpo, que es
/// una rebanada de `b`: no se copia ni se reserva nada.
pub fn leer(
    b: &[u8],
) -> Result<(Cabecera, &[u8]), ErrorCabecera> {
    if b.len() < LARGO {
        return Err(ErrorCabecera::Corta);
    }
    if b[0..4] != FIRMA {
        return Err(ErrorCabecera::FirmaInvalida);
    }
    let c = Cabecera {
        version: u16_le(b, 4),
        banderas: u16_le(b, 6),
        entrada: u32_le(b, 8),
        tamano: u32_le(b, 12),
    };
    let cuerpo = b[LARGO..]
        .get(..c.tamano as usize)
        .ok_or(ErrorCabecera::Corta)?;
    if c.entrada >= c.tamano {
        return Err(ErrorCabecera::EntradaFuera);
    }
    Ok((c, cuerpo))
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMAGEN: [u8; 20] = [
        b'R', b'S', b'O', b'1', 1, 0, 0x02, 0, // versión 1
        2, 0, 0, 0, 4, 0, 0, 0, // entrada 2, tamaño 4
        0xAA, 0xBB, 0xCC, 0xDD, // cuerpo
    ];

    #[test]
    fn lee_una_imagen_valida() {
        let (c, cuerpo) = leer(&IMAGEN).unwrap();
        assert_eq!(c.version, 1);
        assert_eq!(c.banderas, 2);
        assert_eq!(c.entrada, 2);
        assert_eq!(cuerpo, &[0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn rechaza_imagenes_malas() {
        assert_eq!(
            leer(&IMAGEN[..10]),
            Err(ErrorCabecera::Corta)
        );
        assert_eq!(
            leer(&IMAGEN[..18]),
            Err(ErrorCabecera::Corta)
        );
        let mut mala = IMAGEN;
        mala[0] = b'X';
        assert_eq!(
            leer(&mala),
            Err(ErrorCabecera::FirmaInvalida)
        );
        let mut mala = IMAGEN;
        mala[8] = 9;
        assert_eq!(leer(&mala), Err(ErrorCabecera::EntradaFuera));
    }
}
