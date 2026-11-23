// Usa las funciones exportadas como si fuera el anfitrión.
use cap20_futuro::{es_primo, mcd};

fn main() {
    println!("mcd(84, 36) = {}", mcd(84, 36));
    let primos: Vec<u32> =
        (1..30).filter(|&n| es_primo(n)).collect();
    println!("primos menores que 30: {primos:?}");
}
