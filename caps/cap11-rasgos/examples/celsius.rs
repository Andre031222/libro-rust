//! `Display`, `From` y `Default` para un tipo propio.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Celsius(f64);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} °C", self.0)
    }
}

impl From<f64> for Celsius {
    fn from(grados: f64) -> Self {
        Celsius(grados)
    }
}

fn main() {
    let a = Celsius::from(21.46);
    let b: Celsius = 21.46.into(); // `Into` sale de `From`
    let c = Celsius::default();
    println!("{a} | {b:?} | {c}");
    println!("iguales: {}", a == b);
    let texto: String = a.to_string(); // `ToString` gratis
    println!("{texto:?}");
}
