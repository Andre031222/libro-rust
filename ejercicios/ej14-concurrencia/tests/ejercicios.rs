//! Pruebas de los ejercicios del capítulo 14.
//! Ejecute `cargo test -p ej14-concurrencia` hasta que pasen.
//! Ninguna depende del orden en que se ejecutan los hilos.

use ej14_concurrencia::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;

#[test]
fn suma_paralela_coincide_con_la_secuencial() {
    let datos: Vec<i64> = (-500..=1_000).collect();
    let esperado: i64 = datos.iter().sum();
    for hilos in [0, 1, 2, 3, 7, 16, 5_000] {
        assert_eq!(suma_paralela(&datos, hilos), esperado);
    }
    assert_eq!(suma_paralela(&[], 4), 0);
}

#[test]
fn contador_con_mutex_no_pierde_incrementos() {
    let contador = contar_con_mutex(8, 10_000);
    assert_eq!(*contador.lock().unwrap(), 80_000);
    assert_eq!(Arc::strong_count(&contador), 1);
    assert_eq!(*contar_con_mutex(0, 5).lock().unwrap(), 0);
}

#[test]
fn consumidor_recibe_todo_ordenado() {
    let recibido = producir_y_consumir(3, 4);
    let esperado =
        vec![0, 1, 2, 3, 100, 101, 102, 103, 200, 201, 202, 203];
    assert_eq!(recibido, esperado);
    assert_eq!(producir_y_consumir(5, 50).len(), 250);
    assert!(producir_y_consumir(0, 10).is_empty());
}

#[test]
fn contar_pares_con_atomico() {
    let datos: Vec<u32> = (1..=10_001).collect();
    for hilos in [1, 2, 4, 9] {
        assert_eq!(contar_pares(&datos, hilos), 5_000);
    }
    assert_eq!(contar_pares(&[], 3), 0);
}

#[test]
fn generador_de_ids_no_repite() {
    let generador = GeneradorIds::new();
    let ids: Vec<usize> = thread::scope(|s| {
        let hilos: Vec<_> = (0..4)
            .map(|_| {
                s.spawn(|| {
                    (0..1_000)
                        .map(|_| generador.siguiente())
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        hilos
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });
    let unicos: HashSet<usize> = ids.iter().copied().collect();
    assert_eq!(unicos.len(), 4_000);
    assert_eq!(unicos.iter().min(), Some(&1));
    assert_eq!(unicos.iter().max(), Some(&4_000));
}

#[test]
fn grupo_ejecuta_todas_las_tareas() {
    let total = Arc::new(AtomicU64::new(0));
    {
        let grupo = GrupoTrabajadores::new(4);
        assert_eq!(grupo.len(), 4);
        assert!(!grupo.is_empty());
        for i in 1..=100 {
            let total = Arc::clone(&total);
            grupo.ejecutar(move || {
                total.fetch_add(i, Ordering::Relaxed);
            });
        }
    } // al liberar el grupo, se esperan todas las tareas
    assert_eq!(total.load(Ordering::Relaxed), 5_050);
    assert_eq!(Arc::strong_count(&total), 1);
}

#[test]
fn grupo_devuelve_resultados_por_canal() {
    let (tx, rx) = mpsc::channel();
    let grupo = GrupoTrabajadores::new(3);
    for n in 0..20_u64 {
        let tx = tx.clone();
        grupo.ejecutar(move || tx.send(n * n).unwrap());
    }
    drop(tx);
    let mut cuadrados: Vec<u64> = rx.iter().collect();
    cuadrados.sort();
    let esperado: Vec<u64> = (0..20).map(|n| n * n).collect();
    assert_eq!(cuadrados, esperado);
}
