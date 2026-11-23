// Ejercicio 15.5: `.await` cede el control hasta que
// `ListoTras` termina; `bloquear` sondea cada futuro.
use crate::apoyo::{ListoTras, bloquear};

pub async fn cuadrado_lento(x: u32) -> u32 {
    ListoTras::new(x).await;
    x * x
}

pub fn ejecutar_todas(valores: &[u32]) -> Vec<u32> {
    valores
        .iter()
        .map(|&x| bloquear(cuadrado_lento(x)))
        .collect()
}
