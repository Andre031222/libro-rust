//! Generador pseudoaleatorio determinista y ruido de valor 2D.

/// xorshift64 con semilla mezclada por splitmix64: semillas vecinas
/// dan secuencias muy distintas. Sin dependencias externas.
pub struct Azar(u64);

impl Azar {
    pub fn new(semilla: u64) -> Self {
        Azar(mezclar(semilla).max(1))
    }

    pub fn siguiente(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Número uniforme en [a, b).
    pub fn rango(&mut self, a: f64, b: f64) -> f64 {
        let u =
            (self.siguiente() >> 11) as f64 / (1u64 << 53) as f64;
        a + u * (b - a)
    }

    pub fn indice(&mut self, n: usize) -> usize {
        self.siguiente() as usize % n
    }

    pub fn elegir<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[self.indice(xs.len())]
    }
}

fn mezclar(semilla: u64) -> u64 {
    let mut z = semilla.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Ruido de valor 2D suave en [0, 1), con interpolación quíntica.
pub struct Ruido {
    semilla: u64,
}

impl Ruido {
    pub fn new(semilla: u64) -> Self {
        Ruido {
            semilla: mezclar(semilla ^ 0xA5A5),
        }
    }

    fn valor(&self, x: i64, y: i64) -> f64 {
        let h = mezclar(
            self.semilla
                ^ (x as u64).wrapping_mul(0x9E37_79B9)
                ^ (y as u64).wrapping_mul(0x85EB_CA6B),
        );
        (h >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn en(&self, x: f64, y: f64) -> f64 {
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let s =
            |t: f64| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
        let (sx, sy) = (s(fx), s(fy));
        let (xi, yi) = (x0 as i64, y0 as i64);
        let a = self.valor(xi, yi);
        let b = self.valor(xi + 1, yi);
        let c = self.valor(xi, yi + 1);
        let d = self.valor(xi + 1, yi + 1);
        let arriba = a + (b - a) * sx;
        let abajo = c + (d - c) * sx;
        arriba + (abajo - arriba) * sy
    }

    /// Dos octavas: formas grandes con detalle fino.
    pub fn fractal(&self, x: f64, y: f64) -> f64 {
        (self.en(x, y) * 2.0
            + self.en(x * 2.3 + 17.0, y * 2.3 + 5.0))
            / 3.0
    }
}
