use crate::apoyo::Area;

// Ejercicio 11.5: mueve `a` o `b` al resultado; no copia nada.
pub fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if b > a { b } else { a }
}

// Ejercicio 11.7: despacho dinámico sobre figuras mezcladas.
pub fn area_total(figuras: &[Box<dyn Area>]) -> f64 {
    figuras.iter().map(|f| f.area()).sum()
}

pub fn contar_grandes(
    figuras: &[Box<dyn Area>],
    umbral: f64,
) -> usize {
    figuras.iter().filter(|f| f.es_mayor_que(umbral)).count()
}
