//! Existencias de un producto en un almacén de capacidad fija.

use super::precios::precio_final;
use std::io;
use std::path::Path;

/// Errores al crear o modificar unas [`Existencias`].
#[derive(Debug, PartialEq)]
pub enum ErrorExistencias {
    /// Se pidió un almacén de capacidad cero.
    CapacidadCero,
    /// No caben las unidades: solo quedan `libre`.
    SinEspacio { libre: u32 },
    /// No hay tantas unidades: solo hay `hay`.
    Insuficientes { hay: u32 },
}

/// Unidades guardadas de un producto, con un tope fijo.
///
/// Invariante: `0 < capacidad` y `cantidad <= capacidad`.
/// Los campos son privados; fuera de este módulo no se
/// puede construir el tipo con un literal:
///
/// ```compile_fail
/// use cap12_modulos::Existencias;
/// let rota = Existencias { cantidad: 9, capacidad: 1 };
/// ```
#[derive(Debug)]
pub struct Existencias {
    cantidad: u32,
    capacidad: u32,
}

impl Existencias {
    /// Crea un almacén vacío con la `capacidad` dada.
    ///
    /// # Errors
    ///
    /// [`ErrorExistencias::CapacidadCero`] si `capacidad` es 0.
    pub fn new(capacidad: u32) -> Result<Self, ErrorExistencias> {
        if capacidad == 0 {
            return Err(ErrorExistencias::CapacidadCero);
        }
        Ok(Existencias {
            cantidad: 0,
            capacidad,
        })
    }

    /// Unidades guardadas.
    pub fn cantidad(&self) -> u32 {
        self.cantidad
    }

    /// Espacio libre; la invariante impide que la resta
    /// se desborde.
    pub fn libre(&self) -> u32 {
        self.capacidad - self.cantidad
    }

    /// Agrega `n` unidades. Véase [`Existencias::retirar`].
    ///
    /// # Errors
    ///
    /// [`ErrorExistencias::SinEspacio`] si no caben; entonces
    /// las existencias no cambian.
    pub fn agregar(
        &mut self,
        n: u32,
    ) -> Result<(), ErrorExistencias> {
        if n > self.libre() {
            let libre = self.libre();
            return Err(ErrorExistencias::SinEspacio { libre });
        }
        self.cantidad += n;
        Ok(())
    }

    /// Retira `n` unidades.
    ///
    /// # Errors
    ///
    /// [`ErrorExistencias::Insuficientes`] si hay menos de `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cap12_modulos::{ErrorExistencias, Existencias};
    ///
    /// let mut caja = Existencias::new(10)?;
    /// caja.agregar(3)?;
    /// let error = ErrorExistencias::Insuficientes { hay: 3 };
    /// assert_eq!(caja.retirar(5), Err(error));
    /// # Ok::<(), ErrorExistencias>(())
    /// ```
    pub fn retirar(
        &mut self,
        n: u32,
    ) -> Result<(), ErrorExistencias> {
        if n > self.cantidad {
            let hay = self.cantidad;
            return Err(ErrorExistencias::Insuficientes { hay });
        }
        self.cantidad -= n;
        Ok(())
    }

    /// Unidades que tocan a cada una de `tiendas`.
    ///
    /// # Panics
    ///
    /// Si `tiendas` es 0.
    ///
    /// ```should_panic
    /// # use cap12_modulos::Existencias;
    /// let caja = Existencias::new(4).unwrap();
    /// caja.por_tienda(0); // división entre cero
    /// ```
    pub fn por_tienda(&self, tiendas: u32) -> u32 {
        self.cantidad / tiendas
    }

    /// Valor de las existencias con impuesto, en centavos.
    pub fn valor(&self, centavos_unidad: u64) -> u64 {
        precio_final(u64::from(self.cantidad) * centavos_unidad)
    }

    /// Escribe `cantidad/capacidad` en el archivo `ruta`.
    ///
    /// # Errors
    ///
    /// Los mismos de [`std::fs::write`].
    ///
    /// ```no_run
    /// # use cap12_modulos::Existencias;
    /// let caja = Existencias::new(8).unwrap();
    /// caja.guardar("/srv/almacen/caja.txt".as_ref())?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn guardar(&self, ruta: &Path) -> io::Result<()> {
        let texto =
            format!("{}/{}", self.cantidad, self.capacidad);
        std::fs::write(ruta, texto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lee los campos privados para comprobar la invariante.
    fn invariante(e: &Existencias) -> bool {
        0 < e.capacidad && e.cantidad <= e.capacidad
    }

    #[test]
    fn retirar_todo_deja_vacio() -> Result<(), ErrorExistencias> {
        let mut e = Existencias::new(3)?;
        e.agregar(3)?;
        e.retirar(3)?;
        assert_eq!(e.cantidad, 0);
        assert!(invariante(&e));
        Ok(())
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn por_tienda_cero_entra_en_panico() {
        Existencias::new(1).unwrap().por_tienda(0);
    }

    #[test]
    fn agregar_sin_espacio_no_cambia_nada() {
        let mut e = Existencias::new(5).unwrap();
        e.agregar(4).unwrap();
        let error = ErrorExistencias::SinEspacio { libre: 1 };
        assert_eq!(e.agregar(2), Err(error));
        assert_eq!(e.cantidad, 4);
        assert!(invariante(&e));
    }
}
