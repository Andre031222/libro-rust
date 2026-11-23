//! Límites de rasgo, `where` e `impl Trait`.
use std::fmt::{Debug, Display};

fn anunciar_mayor<T: Display + PartialOrd>(a: T, b: T) {
    let m = if a >= b { a } else { b };
    println!("el mayor es {m}");
}

fn describir<T, U>(clave: T, valor: U) -> String
where
    T: Display,
    U: Debug,
{
    format!("{clave} = {valor:?}")
}

fn pares(hasta: u32) -> impl Iterator<Item = u32> {
    (0..=hasta).filter(|n| n % 2 == 0)
}

fn main() {
    anunciar_mayor("pera", "manzana");
    println!("{}", describir("lista", vec![1, 2]));
    let suma: u32 = pares(10).sum();
    println!("{suma}");
}
