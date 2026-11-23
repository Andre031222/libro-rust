//! Pruebas de los ejercicios del capítulo 16.
//! Ejecute `cargo test -p ej16-unsafe` hasta que pasen.

use ej16_unsafe::*;
use std::rc::Rc;

#[test]
fn partir_en_da_mitades_independientes() {
    let mut v = [1, 2, 3, 4, 5];
    let (a, b) = partir_en(&mut v, 2);
    assert_eq!(a, [1, 2]);
    assert_eq!(b, [3, 4, 5]);
    a[1] = 20;
    b[2] = 50;
    assert_eq!(v, [1, 20, 3, 4, 50]);
}

#[test]
fn partir_en_extremos_y_otros_tipos() {
    let mut v = vec![String::from("a"), String::from("b")];
    let (a, b) = partir_en(&mut v, 0);
    assert!(a.is_empty());
    assert_eq!(b.len(), 2);
    let (a, b) = partir_en(&mut v, 2);
    assert_eq!(a.len(), 2);
    assert!(b.is_empty());
    let mut vacio: [u8; 0] = [];
    let (a, b) = partir_en(&mut vacio, 0);
    assert!(a.is_empty() && b.is_empty());
}

#[test]
#[should_panic]
fn partir_en_rechaza_mitad_grande() {
    let mut v = [1, 2, 3];
    partir_en(&mut v, 4);
}

#[test]
fn longitud_c_cuenta_bytes() {
    assert_eq!(longitud_c(c""), 0);
    assert_eq!(longitud_c(c"rust"), 4);
    assert_eq!(longitud_c(c"año"), 4);
}

#[test]
fn longitud_texto_rechaza_nulos() {
    assert_eq!(longitud_texto("hola"), Some(4));
    assert_eq!(longitud_texto(""), Some(0));
    assert_eq!(longitud_texto("ho\0la"), None);
}

#[test]
fn ultimo_maneja_vacio() {
    assert_eq!(ultimo(&[]), None);
    assert_eq!(ultimo(&[7]), Some(7));
    assert_eq!(ultimo(&[1, 2, 3]), Some(3));
}

#[test]
fn bufer_es_fifo_y_da_la_vuelta() {
    let mut b = BuferCircular::new(3);
    assert!(b.is_empty());
    assert_eq!(b.push(1), Ok(()));
    assert_eq!(b.push(2), Ok(()));
    assert_eq!(b.push(3), Ok(()));
    assert!(b.is_full());
    assert_eq!(b.push(4), Err(4));
    assert_eq!(b.pop(), Some(1));
    assert_eq!(b.push(4), Ok(())); // reutiliza la casilla 0
    assert_eq!(b.peek(), Some(&2));
    assert_eq!(b.len(), 3);
    assert_eq!(b.pop(), Some(2));
    assert_eq!(b.pop(), Some(3));
    assert_eq!(b.pop(), Some(4));
    assert_eq!(b.pop(), None);
    assert!(b.is_empty());
}

#[test]
fn bufer_con_cadenas_muchas_vueltas() {
    let mut b = BuferCircular::new(2);
    for i in 0..10 {
        b.push(i.to_string()).unwrap();
        assert_eq!(b.pop(), Some(i.to_string()));
    }
    assert!(b.is_empty());
}

#[test]
fn bufer_libera_lo_que_queda() {
    let testigo = Rc::new(());
    {
        let mut b = BuferCircular::new(4);
        for _ in 0..4 {
            b.push(Rc::clone(&testigo)).unwrap();
        }
        drop(b.pop());
        assert_eq!(Rc::strong_count(&testigo), 4);
    }
    // Al soltar el búfer se liberan los tres que quedaban.
    assert_eq!(Rc::strong_count(&testigo), 1);
}

#[test]
#[should_panic(expected = "capacidad cero")]
fn bufer_rechaza_capacidad_cero() {
    let _b: BuferCircular<i32> = BuferCircular::new(0);
}
