// Ejercicio 12.5: el campo privado protege el invariante.
use crate::apoyo::ErrorPorcentaje;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Porcentaje {
    valor: u8,
}

impl Porcentaje {
    pub fn new(valor: u32) -> Result<Self, ErrorPorcentaje> {
        if valor > 100 {
            return Err(ErrorPorcentaje::FueraDeRango);
        }
        Ok(Porcentaje { valor: valor as u8 })
    }

    pub fn valor(&self) -> u8 {
        self.valor
    }

    pub fn complemento(&self) -> Porcentaje {
        Porcentaje {
            valor: 100 - self.valor,
        }
    }

    pub fn aplicar(&self, monto: u32) -> u32 {
        (u64::from(monto) * u64::from(self.valor) / 100) as u32
    }
}
