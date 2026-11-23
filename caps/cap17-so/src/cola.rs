//! Cola circular de capacidad fija, sin montículo.

/// Cola FIFO de hasta `N` elementos guardados en un arreglo.
pub struct ColaCircular<T: Copy, const N: usize> {
    datos: [Option<T>; N],
    inicio: usize, // índice del elemento más antiguo
    largo: usize,
}

impl<T: Copy, const N: usize> ColaCircular<T, N> {
    pub const fn new() -> Self {
        ColaCircular {
            datos: [None; N],
            inicio: 0,
            largo: 0,
        }
    }

    /// Encola al final. Si está llena, devuelve el valor.
    pub fn push(&mut self, x: T) -> Result<(), T> {
        if self.largo == N {
            return Err(x);
        }
        let fin = (self.inicio + self.largo) % N;
        self.datos[fin] = Some(x);
        self.largo += 1;
        Ok(())
    }

    /// Saca el elemento más antiguo.
    pub fn pop(&mut self) -> Option<T> {
        let x = self.datos[self.inicio].take()?;
        self.inicio = (self.inicio + 1) % N;
        self.largo -= 1;
        Some(x)
    }

    pub fn len(&self) -> usize {
        self.largo
    }

    pub fn is_empty(&self) -> bool {
        self.largo == 0
    }
}

impl<T: Copy, const N: usize> Default for ColaCircular<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_con_vuelta() {
        let mut c = ColaCircular::<u8, 3>::new();
        assert_eq!(c.push(1), Ok(()));
        assert_eq!(c.push(2), Ok(()));
        assert_eq!(c.push(3), Ok(()));
        assert_eq!(c.push(4), Err(4)); // llena
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.push(4), Ok(())); // da la vuelta
        assert_eq!(c.len(), 3);
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), None);
        assert!(c.is_empty());
    }
}
