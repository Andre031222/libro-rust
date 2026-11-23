//! Pruebas de los ejercicios del capítulo 10.
//! Ejecute `cargo test -p ej10-colecciones` hasta que pasen.

use ej10_colecciones::*;

#[test]
fn contar_palabras_ignora_mayusculas() {
    let c = contar_palabras("Sol y luna y SOL");
    assert_eq!(c["sol"], 2);
    assert_eq!(c["y"], 2);
    assert_eq!(c["luna"], 1);
    assert_eq!(c.len(), 3);
}

#[test]
fn contar_palabras_texto_vacio() {
    assert!(contar_palabras("   ").is_empty());
}

#[test]
fn cuadrados_solo_de_pares() {
    assert_eq!(
        cuadrados_de_pares(&[1, 2, 3, 4, -6]),
        [4, 16, 36]
    );
    assert!(cuadrados_de_pares(&[]).is_empty());
}

#[test]
fn producto_punto_basico() {
    assert_eq!(
        producto_punto(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]),
        32.0
    );
    assert_eq!(producto_punto(&[1.0, 2.0], &[3.0]), 3.0);
}

#[test]
fn acumulador_guarda_estado() {
    let mut acumular = crear_acumulador();
    assert_eq!(acumular(5), 5);
    assert_eq!(acumular(3), 8);
    assert_eq!(acumular(-10), -2);
}

#[test]
fn acumuladores_independientes() {
    let mut a = crear_acumulador();
    let mut b = crear_acumulador();
    a(100);
    assert_eq!(b(1), 1);
}

#[test]
fn indice_invertido_lineas_sin_repetir() {
    let texto =
        "El gato duerme.\nEl perro y el gato\n\nduerme, gato";
    let ind = indice_invertido(texto);
    assert_eq!(ind["el"], [1, 2]);
    assert_eq!(ind["gato"], [1, 2, 4]);
    assert_eq!(ind["duerme"], [1, 4]);
    assert_eq!(ind["perro"], [2]);
    assert!(!ind.contains_key(""));
}
