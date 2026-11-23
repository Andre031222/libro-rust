// Ejercicio 17.5: cola circular que, llena, descarta la tecla
// más antigua para guardar la nueva.
pub struct ColaTeclado<const N: usize> {
    datos: [u8; N],
    inicio: usize,
    largo: usize,
}

impl<const N: usize> ColaTeclado<N> {
    pub const fn new() -> Self {
        ColaTeclado {
            datos: [0; N],
            inicio: 0,
            largo: 0,
        }
    }

    pub fn push(&mut self, b: u8) -> Option<u8> {
        if N == 0 {
            return Some(b);
        }
        let mut descartada = None;
        if self.largo == N {
            descartada = self.pop();
        }
        let fin = (self.inicio + self.largo) % N;
        self.datos[fin] = b;
        self.largo += 1;
        descartada
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.largo == 0 {
            return None;
        }
        let b = self.datos[self.inicio];
        self.inicio = (self.inicio + 1) % N;
        self.largo -= 1;
        Some(b)
    }
}

impl<const N: usize> Default for ColaTeclado<N> {
    fn default() -> Self {
        Self::new()
    }
}
