//! Pruebas de los ejercicios del capítulo 15.
//! Ejecute `cargo test -p ej15-async` hasta que pasen.

use ej15_async::*;
use std::cell::RefCell;
use std::future::{Future, pending};
use std::pin::pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

/// Sondea a mano hasta `Ready` y cuenta los sondeos.
fn contar_sondeos<F: Future>(f: F) -> (F::Output, u32) {
    let mut cx = Context::from_waker(Waker::noop());
    let mut f = pin!(f);
    let mut veces = 0;
    loop {
        veces += 1;
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return (v, veces);
        }
    }
}

#[test]
fn listo_tras_cero_esta_listo_al_primer_sondeo() {
    assert_eq!(contar_sondeos(ListoTras::new(0)), (0, 1));
}

#[test]
fn listo_tras_tres_necesita_cuatro_sondeos() {
    assert_eq!(contar_sondeos(ListoTras::new(3)), (3, 4));
}

#[test]
#[should_panic(expected = "después de Ready")]
fn listo_tras_no_se_sondea_dos_veces_al_final() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut f = pin!(ListoTras::new(0));
    let _ = f.as_mut().poll(&mut cx);
    let _ = f.as_mut().poll(&mut cx);
}

#[test]
fn listo_tras_funciona_con_bloquear() {
    assert_eq!(bloquear(ListoTras::new(5)), 5);
}

#[test]
fn cuadrado_lento_espera_y_calcula() {
    assert_eq!(contar_sondeos(cuadrado_lento(3)), (9, 4));
}

#[test]
fn ejecutar_todas_conserva_el_orden() {
    assert_eq!(ejecutar_todas(&[3, 0, 2]), vec![9, 0, 4]);
    assert!(ejecutar_todas(&[]).is_empty());
}

#[test]
fn unir_avanza_los_dos_a_la_vez() {
    let f = unir(ListoTras::new(2), ListoTras::new(5));
    // Juntos tardan max(2, 5) + 1 sondeos.
    assert_eq!(contar_sondeos(f), ((2, 5), 6));
}

#[test]
fn unir_con_bloquear() {
    let f = unir(ListoTras::new(4), ListoTras::new(0));
    assert_eq!(bloquear(f), (4, 0));
}

#[test]
fn round_robin_alterna_las_tareas() {
    let mut rr = RoundRobin::default();
    let a = rr.lanzar(async {
        ListoTras::new(2).await;
    });
    let b = rr.lanzar(async {});
    let c = rr.lanzar(async {
        ListoTras::new(1).await;
    });
    assert_eq!((a, b, c), (0, 1, 2));
    assert_eq!(rr.ejecutar(), vec![0, 1, 2, 0, 2, 0]);
}

#[test]
fn round_robin_registra_el_trabajo() {
    let log = Rc::new(RefCell::new(String::new()));
    let mut rr = RoundRobin::default();
    for letra in ['x', 'y'] {
        let log = Rc::clone(&log);
        rr.lanzar(async move {
            for _ in 0..2 {
                log.borrow_mut().push(letra);
                ListoTras::new(1).await;
            }
        });
    }
    rr.ejecutar();
    assert_eq!(*log.borrow(), "xyxy");
}

#[test]
fn round_robin_no_sondea_sin_aviso() {
    let mut rr = RoundRobin::default();
    rr.lanzar(pending::<()>()); // nunca llama a wake
    rr.lanzar(async {});
    assert_eq!(rr.ejecutar(), vec![0, 1]);
}
