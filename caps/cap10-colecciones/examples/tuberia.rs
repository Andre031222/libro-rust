//! Adaptadores perezosos y consumidores.

fn main() {
    let notas = [12, 17, 8, 15, 20, 9];

    // Ningún adaptador hace trabajo hasta que collect lo pide.
    let aprobadas: Vec<i32> = notas
        .iter()
        .filter(|&&n| n >= 11)
        .map(|n| n * 5) // escala de 0 a 100
        .collect();
    println!("{aprobadas:?}");

    for (i, n) in notas.iter().enumerate().take(2) {
        println!("alumno {i}: {n}");
    }

    let pesos = [1, 1, 2, 2, 3, 3];
    let suma: i32 =
        notas.iter().zip(pesos).map(|(n, p)| n * p).sum();
    let peso: i32 = pesos.iter().sum();
    println!(
        "promedio ponderado = {:.2}",
        suma as f64 / peso as f64
    );

    let maxima = notas.iter().fold(0, |m, &n| m.max(n));
    println!("máxima = {maxima}");
}
