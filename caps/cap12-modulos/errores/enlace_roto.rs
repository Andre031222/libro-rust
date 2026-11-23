//! Un enlace intradocumento a un método que no existe.

pub struct Caja;

impl Caja {
    /// Véase [`Caja::sacar`].
    pub fn agregar(&mut self) {}

    /// Retira unidades.
    pub fn retirar(&mut self) {}
}
