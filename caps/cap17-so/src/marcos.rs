//! Asignador de marcos físicos con un mapa de bits.

/// Tamaño de un marco: 4 KiB, como una página de x86-64.
pub const TAM_MARCO: usize = 4096;

/// Un marco de memoria física, identificado por su número.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Marco(pub usize);

impl Marco {
    /// Dirección física donde empieza el marco.
    pub fn direccion(self) -> usize {
        self.0 * TAM_MARCO
    }
}

/// Por qué falló una liberación.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorMarco {
    FueraDeRango,
    DobleLiberacion,
}

/// Un bit por marco: 1 ocupado, 0 libre. Con `P` palabras
/// de 64 bits se administran `64 * P` marcos.
pub struct AsignadorMarcos<const P: usize> {
    mapa: [u64; P],
}

impl<const P: usize> AsignadorMarcos<P> {
    /// Todos libres. Es `const fn`: sirve para un `static`.
    pub const fn new() -> Self {
        AsignadorMarcos { mapa: [0; P] }
    }

    /// Ocupa el primer marco libre y lo devuelve.
    pub fn asignar(&mut self) -> Option<Marco> {
        for (i, palabra) in self.mapa.iter_mut().enumerate() {
            if *palabra != u64::MAX {
                // Unos seguidos desde el bit 0 = marcos ocupados.
                let bit = palabra.trailing_ones() as usize;
                *palabra |= 1 << bit;
                return Some(Marco(i * 64 + bit));
            }
        }
        None // memoria agotada
    }

    /// Devuelve un marco al mapa. Liberarlo dos veces es error.
    pub fn liberar(
        &mut self,
        m: Marco,
    ) -> Result<(), ErrorMarco> {
        let (i, bit) = (m.0 / 64, m.0 % 64);
        let palabra = self
            .mapa
            .get_mut(i)
            .ok_or(ErrorMarco::FueraDeRango)?;
        if *palabra & (1 << bit) == 0 {
            return Err(ErrorMarco::DobleLiberacion);
        }
        *palabra &= !(1 << bit);
        Ok(())
    }

    /// Marca como ocupados los marcos `desde..hasta`, por
    /// ejemplo los que usan el firmware o el propio núcleo.
    pub fn reservar(&mut self, desde: usize, hasta: usize) {
        let hasta = hasta.min(64 * P);
        for n in desde..hasta {
            self.mapa[n / 64] |= 1 << (n % 64);
        }
    }

    /// Número de marcos libres.
    pub fn libres(&self) -> usize {
        self.mapa.iter().map(|p| p.count_zeros() as usize).sum()
    }

    /// Lista de marcos libres. Necesita montículo: `alloc`.
    #[cfg(feature = "alloc")]
    pub fn lista_libres(&self) -> alloc::vec::Vec<Marco> {
        (0..64 * P)
            .filter(|n| self.mapa[n / 64] & (1 << (n % 64)) == 0)
            .map(Marco)
            .collect()
    }
}

impl<const P: usize> Default for AsignadorMarcos<P> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asigna_en_orden_y_detecta_doble_liberacion() {
        let mut a = AsignadorMarcos::<1>::new();
        assert_eq!(a.asignar(), Some(Marco(0)));
        assert_eq!(a.asignar(), Some(Marco(1)));
        assert_eq!(a.liberar(Marco(0)), Ok(()));
        assert_eq!(
            a.liberar(Marco(0)),
            Err(ErrorMarco::DobleLiberacion)
        );
        // El hueco liberado se reutiliza primero.
        assert_eq!(a.asignar(), Some(Marco(0)));
        assert_eq!(a.libres(), 62);
    }

    #[test]
    fn se_agota_y_cruza_palabras() {
        let mut a = AsignadorMarcos::<2>::new();
        a.reservar(0, 64);
        assert_eq!(a.asignar(), Some(Marco(64)));
        a.reservar(0, 1000);
        assert_eq!(a.libres(), 0);
        assert_eq!(a.asignar(), None);
        assert_eq!(
            a.liberar(Marco(128)),
            Err(ErrorMarco::FueraDeRango)
        );
    }

    #[test]
    fn direccion_de_un_marco() {
        assert_eq!(Marco(3).direccion(), 0x3000);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn lista_libres_usa_alloc() {
        let mut a = AsignadorMarcos::<1>::new();
        a.reservar(0, 62);
        assert_eq!(a.lista_libres(), [Marco(62), Marco(63)]);
    }
}
