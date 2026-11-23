//! Pruebas de los ejercicios del capítulo 5.
//! Ejecute `cargo test -p ej05-tipos` hasta que todas pasen.

use ej05_tipos::*;

#[test]
fn conversion_de_puntos_fijos() {
    assert_eq!(celsius_a_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_a_fahrenheit(100.0), 212.0);
    assert_eq!(celsius_a_fahrenheit(-40.0), -40.0);
}

#[test]
fn clasificacion_por_tramos() {
    assert_eq!(clasificar(5.0), "frío");
    assert_eq!(clasificar(10.0), "templado");
    assert_eq!(clasificar(30.0), "caluroso");
}

#[test]
fn bufer_que_cabe() {
    assert_eq!(tamano_bufer(1920, 1080, 4), Some(8_294_400));
}

#[test]
fn bufer_que_desborda() {
    assert_eq!(tamano_bufer(65_536, 65_536, 1), None);
    assert_eq!(tamano_bufer(40_000, 40_000, 4), None);
}

#[test]
fn maximo_con_repetidos() {
    assert_eq!(maximo_y_posicion([3, 9, -1, 9, 2]), (9, 1));
    assert_eq!(maximo_y_posicion([-5, -2, -8, -3, -9]), (-2, 1));
}

#[test]
fn fizzbuzz_hasta_quince() {
    let esperado = [
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz",
        "Buzz", "11", "Fizz", "13", "14", "FizzBuzz",
    ];
    assert_eq!(fizzbuzz(15), esperado);
    assert!(fizzbuzz(0).is_empty());
}
