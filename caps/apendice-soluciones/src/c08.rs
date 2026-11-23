// Ejercicio 8.5: un solo `match` cubre las tres variantes.
#[derive(Debug)]
pub enum Figura {
    Circulo { radio: f64 },
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo { base: f64, altura: f64 },
}

impl Figura {
    pub fn area(&self) -> f64 {
        match self {
            Figura::Circulo { radio } => {
                std::f64::consts::PI * radio * radio
            }
            Figura::Rectangulo { ancho, alto } => ancho * alto,
            Figura::Triangulo { base, altura } => {
                base * altura / 2.0
            }
        }
    }
}
