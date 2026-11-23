//! Ejercicios 12.5 a 12.7. Un tipo con invariante.

/// Errores al crear un [`Porcentaje`].
#[derive(Debug, PartialEq)]
pub enum ErrorPorcentaje {
    /// El valor pasa de 100.
    FueraDeRango,
    /// El total es cero (ejercicio 12.6).
    TotalCero,
    /// El texto no tiene la forma `"45%"` (reto 12.7).
    Formato,
}

/// Ejercicio 12.5. Un porcentaje entero entre 0 y 100.
///
/// El campo es privado: fuera de este módulo la única forma
/// de obtener un `Porcentaje` es pasar por [`Porcentaje::new`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Porcentaje {
    valor: u8,
}

impl Porcentaje {
    /// Ejercicio 12.5. Crea el porcentaje si `valor <= 100`; si
    /// no, devuelve [`ErrorPorcentaje::FueraDeRango`].
    pub fn new(valor: u32) -> Result<Self, ErrorPorcentaje> {
        todo!()
    }

    /// Ejercicio 12.5. El valor guardado, entre 0 y 100.
    pub fn valor(&self) -> u8 {
        todo!()
    }

    /// Ejercicio 12.5. Lo que falta para llegar a 100.
    pub fn complemento(&self) -> Porcentaje {
        todo!()
    }

    /// Ejercicio 12.5. Aplica el porcentaje a `monto`, truncado.
    /// No debe desbordarse ni con `u32::MAX`.
    pub fn aplicar(&self, monto: u32) -> u32 {
        todo!()
    }

    /// Reto 12.7. Lee un porcentaje escrito como `"45%"`, sin
    /// espacios. Documente esta función con las secciones
    /// `# Examples` (con `?` y líneas ocultas) y `# Errors`.
    pub fn desde_texto(
        texto: &str,
    ) -> Result<Self, ErrorPorcentaje> {
        todo!()
    }
}

/// Ejercicio 12.6. Qué porcentaje de `total` representa
/// `parte`, truncado. Devuelve [`ErrorPorcentaje::TotalCero`]
/// si `total` es 0 y [`ErrorPorcentaje::FueraDeRango`] si
/// `parte > total`.
pub fn porcentaje_de(
    parte: u32,
    total: u32,
) -> Result<Porcentaje, ErrorPorcentaje> {
    todo!()
}
