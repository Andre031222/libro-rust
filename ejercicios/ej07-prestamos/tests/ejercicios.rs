//! Pruebas de los ejercicios del capítulo 7.
//! Ejecute `cargo test -p ej07-prestamos` hasta que todas pasen.

use ej07_prestamos::*;

#[test]
fn primera_palabra_de_una_frase() {
    assert_eq!(primera_palabra("hola mundo"), "hola");
}

#[test]
fn primera_palabra_sin_espacios() {
    assert_eq!(primera_palabra("rust"), "rust");
    assert_eq!(primera_palabra(""), "");
}

#[test]
fn primera_palabra_no_copia() {
    let s = String::from("sin copias");
    assert_eq!(primera_palabra(&s).as_ptr(), s.as_ptr());
}

#[test]
fn depurar_quita_negativos_y_duplica() {
    let mut v = vec![3, -1, 0, -7, 5];
    depurar(&mut v);
    assert_eq!(v, vec![6, 0, 10]);
}

#[test]
fn depurar_vector_vacio() {
    let mut v: Vec<i32> = Vec::new();
    depurar(&mut v);
    assert!(v.is_empty());
}

#[test]
fn mas_largo_elige_bien() {
    assert_eq!(mas_largo("ab", "xyz"), "xyz");
    assert_eq!(mas_largo("abc", "xy"), "abc");
    assert_eq!(mas_largo("ab", "cd"), "ab");
}

#[test]
fn palabras_separa_por_espacios() {
    let t = "  uno dos\ttres\n";
    assert_eq!(palabras(t), vec!["uno", "dos", "tres"]);
    assert!(palabras("   ").is_empty());
}

#[test]
fn palabras_son_rebanadas_del_original() {
    let t = String::from("sin copiar nada");
    let rango = t.as_bytes().as_ptr_range();
    for p in palabras(&t) {
        assert!(rango.contains(&p.as_ptr()));
    }
}
