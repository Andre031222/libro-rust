//! Pruebas de los ejercicios del capítulo 1.
//! Ejecute `cargo test -p ej01-memoria` hasta que todas pasen.

use ej01_memoria::*;

#[test]
fn leer_dentro_de_limites() {
    assert_eq!(leer(&[4, 5, 6], 2), Some(6));
}

#[test]
fn leer_fuera_de_limites() {
    assert_eq!(leer(&[4, 5, 6], 3), None);
    assert_eq!(leer(&[], 0), None);
}

#[test]
fn suma_sin_desbordar() {
    assert_eq!(suma_segura(200, 55), Some(255));
}

#[test]
fn suma_que_desborda() {
    assert_eq!(suma_segura(200, 56), None);
}

#[test]
fn truncar_ascii() {
    assert_eq!(truncar_utf8("contraseña", 5), "contr");
}

#[test]
fn truncar_no_parte_la_enie() {
    // «ñ» ocupa los bytes 8 y 9: cortar en 9 la partiría.
    assert_eq!(truncar_utf8("contraseña", 9), "contrase");
    assert_eq!(truncar_utf8("contraseña", 10), "contraseñ");
}

#[test]
fn truncar_mas_largo_que_el_texto() {
    assert_eq!(truncar_utf8("hola", 50), "hola");
}
