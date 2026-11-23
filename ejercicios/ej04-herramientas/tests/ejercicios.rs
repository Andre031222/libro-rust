//! Pruebas de los ejercicios del capítulo 4.
//! Ejecute `cargo test -p ej04-herramientas` hasta que todas
//! pasen.

use ej04_herramientas::*;

#[test]
fn bisiestos_comunes() {
    assert!(es_bisiesto(2024));
    assert!(!es_bisiesto(2026));
}

#[test]
fn bisiestos_de_siglo() {
    assert!(!es_bisiesto(1900));
    assert!(es_bisiesto(2000));
}

#[test]
fn vocales_con_tilde_y_mayusculas() {
    assert_eq!(contar_vocales("Rust es rápido"), 5);
    assert_eq!(contar_vocales("ÁRBOL pingüino"), 6);
    assert_eq!(contar_vocales("xyz"), 0);
}

fn cadenas(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn suma_de_numeros() {
    let args = cadenas(&["10", "-3", "5"]);
    assert_eq!(sumar_argumentos(&args), Some(12));
    assert_eq!(sumar_argumentos(&[]), Some(0));
}

#[test]
fn suma_con_texto_devuelve_none() {
    let args = cadenas(&["10", "diez"]);
    assert_eq!(sumar_argumentos(&args), None);
}
