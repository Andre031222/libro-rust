//! Hilos con ámbito: prestar datos locales sin `move` ni `Arc`.

use std::thread;

fn main() {
    let datos: Vec<u64> = (1..=1_000).collect();
    let (izq, der) = datos.split_at(datos.len() / 2);
    let mut maximo = 0;

    let total = thread::scope(|s| {
        let a = s.spawn(|| izq.iter().sum::<u64>());
        let b = s.spawn(|| der.iter().sum::<u64>());
        s.spawn(|| maximo = *datos.iter().max().unwrap());
        a.join().unwrap() + b.join().unwrap()
    }); // aquí ya terminaron los tres hilos

    println!("total = {total}, máximo = {maximo}");
    println!("datos sigue disponible: {} elementos", datos.len());
}
