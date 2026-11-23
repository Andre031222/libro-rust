// Ejercicio 10.5: dos cálculos sin bucles explícitos.
pub fn cuadrados_de_pares(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&x| x % 2 == 0).map(|x| x * x).collect()
}

pub fn producto_punto(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
