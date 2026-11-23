//! Pruebas de los ejercicios del capítulo 6.
//! Ejecute `cargo test -p ej06-propiedad` hasta que todas pasen.

use ej06_propiedad::*;

#[test]
fn primera_palabra_de_una_frase() {
    assert_eq!(primera_palabra_len(&"hola mundo".to_string()), 4);
}

#[test]
fn primera_palabra_de_cadena_vacia() {
    assert_eq!(primera_palabra_len(&String::new()), 0);
}

#[test]
fn concatenar_toma_a_y_presta_b() {
    assert_eq!(concatenar("Ru".to_string(), "st"), "Rust");
}

#[test]
fn duplicar_devuelve_dos_copias() {
    assert_eq!(duplicar(vec![1, 2]), (vec![1, 2], vec![1, 2]));
}

#[test]
fn mover_mayor_en_empate_devuelve_a() {
    assert_eq!(
        mover_mayor("ab".to_string(), "cd".to_string()),
        "ab"
    );
}

#[test]
fn mover_mayor_devuelve_la_mas_larga() {
    assert_eq!(
        mover_mayor("a".to_string(), "xyz".to_string()),
        "xyz"
    );
}
