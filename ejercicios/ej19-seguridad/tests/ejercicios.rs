//! Pruebas de los ejercicios del capítulo 19.
//! Ejecute `cargo test -p ej19-seguridad` hasta que pasen.

use ej19_seguridad::*;
use std::panic;
use std::path::{Path, PathBuf};

const BASE: &str = "/srv/datos";

#[test]
fn ruta_dentro_acepta_y_normaliza() {
    let b = Path::new(BASE);
    let r = |n| ruta_dentro(b, n);
    assert_eq!(
        r("a.txt"),
        Some(PathBuf::from("/srv/datos/a.txt"))
    );
    assert_eq!(r("x/y/z.txt"), Some(b.join("x/y/z.txt")));
    assert_eq!(r("a/../b.txt"), Some(b.join("b.txt")));
    assert_eq!(r("./a/./b"), Some(b.join("a/b")));
}

#[test]
fn ruta_dentro_rechaza_las_fugas() {
    let b = Path::new(BASE);
    for malo in ["", "..", "../x", "a/../../x", "/etc/passwd"] {
        assert_eq!(ruta_dentro(b, malo), None, "{malo:?}");
    }
}

#[test]
fn dimensiones_validas() {
    assert_eq!(dimensiones("800x600"), Ok((800, 600)));
    assert_eq!(dimensiones("1x1"), Ok((1, 1)));
    assert_eq!(dimensiones_fragil("800x600"), (800, 600));
}

#[test]
fn dimensiones_invalidas_no_entran_en_panico() {
    use ErrorDimension::*;
    assert_eq!(dimensiones("800"), Err(SinSeparador));
    assert_eq!(dimensiones("x600"), Err(NoEsNumero));
    assert_eq!(dimensiones("8a0x600"), Err(NoEsNumero));
    assert_eq!(dimensiones("99999999999x1"), Err(NoEsNumero));
    assert_eq!(dimensiones("0x600"), Err(Cero));
    assert_eq!(dimensiones("800x0"), Err(Cero));
}

#[test]
fn longitud_paquete_suma_las_cabeceras() {
    assert_eq!(longitud_paquete(&[]), Some(0));
    assert_eq!(longitud_paquete(&[b"abc", b""]), Some(7));
    let grande = vec![0u8; 65_533];
    assert_eq!(longitud_paquete(&[&grande]), Some(65_535));
}

#[test]
fn longitud_paquete_detecta_el_desbordamiento() {
    let grande = vec![0u8; 65_534];
    assert_eq!(longitud_paquete(&[&grande]), None);
    let enorme = vec![0u8; 70_000];
    assert_eq!(longitud_paquete(&[&enorme]), None);
    let medio = vec![0u8; 40_000];
    assert_eq!(longitud_paquete(&[&medio, &medio]), None);
}

#[test]
fn tlv_corregido_lee_y_rechaza() {
    let e = [1, 2, b'h', b'i', 7, 0];
    let esperado: Vec<(u8, &[u8])> = vec![(1, b"hi"), (7, b"")];
    assert_eq!(analizar_tlv_corregido(&e), Ok(esperado));
    assert_eq!(analizar_tlv_corregido(&[]), Ok(vec![]));
    for malo in [&[1u8][..], &[1, 1], &[1, 3, 0, 0], &[1, 0, 9]] {
        assert_eq!(
            analizar_tlv_corregido(malo),
            Err(ErrorTlv::Truncado),
            "{malo:?}"
        );
    }
}

fn sembrado(e: &[u8]) {
    let _ = analizar_tlv(e);
}

fn corregido(e: &[u8]) {
    let _ = analizar_tlv_corregido(e);
}

fn falla(objetivo: fn(&[u8]), e: &[u8]) -> bool {
    panic::catch_unwind(|| objetivo(e)).is_err()
}

// Una sola prueba, porque el gancho de pánico es global.
#[test]
fn reto_fuzzer_encuentra_reduce_y_confirma() {
    let e = buscar_fallo(sembrado, 2026, 100_000)
        .expect("el fuzzer debe encontrar el error sembrado");
    let original = e.clone();
    let m = reducir(sembrado, e);
    // Comprobaciones en silencio; los asertos, después.
    panic::set_hook(Box::new(|_| {}));
    let falla_original = falla(sembrado, &original);
    let falla_m = falla(sembrado, &m);
    let quitables: Vec<usize> = (0..m.len())
        .filter(|&i| {
            let mut menor = m.clone();
            menor.remove(i);
            falla(sembrado, &menor)
        })
        .collect();
    let _ = panic::take_hook();
    assert!(original.len() <= 8 && falla_original);
    assert!(falla_m, "la entrada reducida debe fallar");
    assert!(m.len() <= original.len());
    assert!(quitables.is_empty(), "no es mínima: {m:?}");
    assert_eq!(buscar_fallo(corregido, 2026, 100_000), None);
}
