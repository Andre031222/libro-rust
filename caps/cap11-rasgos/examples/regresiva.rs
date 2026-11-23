//! Un iterador propio: cuenta regresiva desde `n` hasta 1.

struct Regresiva {
    actual: u32,
}

impl Iterator for Regresiva {
    type Item = u32; // tipo asociado: lo que entrega `next`

    fn next(&mut self) -> Option<Self::Item> {
        if self.actual == 0 {
            return None;
        }
        self.actual -= 1;
        Some(self.actual + 1)
    }
}

fn main() {
    let v: Vec<u32> = Regresiva { actual: 5 }.collect();
    println!("{v:?}");
    let suma: u32 = Regresiva { actual: 10 }
        .filter(|n| n % 3 == 0)
        .map(|n| n * n)
        .sum();
    println!("{suma}");
}
