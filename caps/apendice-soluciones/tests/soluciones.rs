//! Comprueba las soluciones del apéndice A.

use apendice_soluciones::apoyo::*;
use apendice_soluciones::*;
use cap18_proyecto::Almacen;
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[test]
fn capitulos_1_a_7() {
    assert_eq!(c01::suma_segura(200, 55), Some(255));
    assert_eq!(c01::suma_segura(200, 56), None);
    assert_eq!(c04::contar_vocales("Ágil ÜÑ xyz"), 3);
    assert_eq!(c05::tamano_bufer(1920, 1080, 4), Some(8_294_400));
    assert_eq!(c05::tamano_bufer(65_536, 65_536, 1), None);
    let a = String::with_capacity(16) + "ho";
    let p = a.as_ptr();
    let r = c06::concatenar(a, "la");
    assert_eq!((r.as_str(), r.as_ptr()), ("hola", p));
    let m = c06::mover_mayor("ab".into(), "cd".into());
    assert_eq!(m, "ab");
    let mut v = vec![3, -1, 0, -7, 5];
    c07::depurar(&mut v);
    assert_eq!(v, [6, 0, 10]);
}

#[test]
fn capitulos_8_a_12() {
    let t = c08::Figura::Triangulo {
        base: 4.0,
        altura: 3.0,
    };
    assert_eq!(t.area(), 6.0);
    let cfg = c09::leer_config("a = 1\n\nb=2").unwrap();
    assert_eq!(cfg[1], ("b".to_string(), "2".to_string()));
    let e = c09::leer_config("a=1\nsin igual").unwrap_err();
    assert_eq!(e.to_string(), "línea 2: falta '='");
    assert_eq!(
        c09::leer_config(" =3"),
        Err(ErrorConfig::ClaveVacia { linea: 1 })
    );
    assert_eq!(c10::cuadrados_de_pares(&[1, 2, 3, 4]), [4, 16]);
    assert_eq!(
        c10::producto_punto(&[1.0, 2.0], &[3.0, 4.0]),
        11.0
    );
    assert_eq!(c11::mayor("b".to_string(), "a".to_string()), "b");
    let figuras: Vec<Box<dyn Area>> = vec![
        Box::new(Rectangulo {
            ancho: 2.0,
            alto: 3.0,
        }),
        Box::new(Circulo { radio: 1.0 }),
    ];
    let total = c11::area_total(&figuras);
    assert!((total - (6.0 + std::f64::consts::PI)).abs() < 1e-9);
    assert_eq!(c11::contar_grandes(&figuras, 5.0), 1);
    assert_eq!(size_of::<Box<Circulo>>(), size_of::<usize>());
    assert_eq!(
        size_of::<Box<dyn Area>>(),
        2 * size_of::<usize>()
    );
    let p = c12::Porcentaje::new(30).unwrap();
    assert_eq!(p.complemento().valor(), 70);
    assert_eq!(p.aplicar(u32::MAX), 1_288_490_188);
    assert_eq!(
        c12::Porcentaje::new(101),
        Err(ErrorPorcentaje::FueraDeRango)
    );
}

#[test]
fn capitulo_13() {
    let t = Rc::new(String::from("texto"));
    let copias = c13::repartir(&t, 3);
    assert_eq!(c13::duenos(&t), 4);
    drop(copias);
    assert_eq!(c13::duenos(&t), 1);
    let registro = Rc::new(RefCell::new(Vec::new()));
    let nuevo = |n: &str| c13::Registrador {
        nombre: n.to_string(),
        registro: Rc::clone(&registro),
    };
    {
        let _a = nuevo("a");
        {
            let _b = nuevo("b");
            let _c = nuevo("c");
        }
        let _d = nuevo("d");
    }
    assert_eq!(*registro.borrow(), ["c", "b", "d", "a"]);
}

#[test]
fn capitulos_14_y_15() {
    let c = c14::contar_con_mutex(4, 1000);
    assert_eq!(Arc::strong_count(&c), 1);
    assert_eq!(*c.lock().unwrap(), 4000);
    let datos: Vec<u32> = (1..=101).collect();
    assert_eq!(c14::contar_pares(&datos, 4), 50);
    assert_eq!(c14::contar_pares(&[], 0), 0);
    static IDS: c14::GeneradorIds = c14::GeneradorIds::new();
    let hilos: Vec<_> = (0..4)
        .map(|_| thread::spawn(|| IDS.siguiente()))
        .collect();
    let mut ids: Vec<_> =
        hilos.into_iter().map(|h| h.join().unwrap()).collect();
    ids.sort();
    assert_eq!(ids, [1, 2, 3, 4]);
    assert_eq!(c15::ejecutar_todas(&[3, 0, 5]), [9, 0, 25]);
}

#[test]
fn capitulos_16_a_19() {
    let c = CString::new("hola").unwrap();
    assert_eq!(c16::longitud_c(&c), 4);
    assert_eq!(c16::longitud_texto("año"), Some(4));
    assert_eq!(c16::longitud_texto("a\0b"), None);
    let mut cola = c17::ColaTeclado::<2>::new();
    assert_eq!(cola.push(1), None);
    assert_eq!(cola.push(2), None);
    assert_eq!(cola.push(3), Some(1));
    assert_eq!(
        (cola.pop(), cola.pop(), cola.pop()),
        (Some(2), Some(3), None)
    );
    let a = Almacen::new();
    a.agregar("puno".into(), 10.0);
    assert_eq!(c18::por_estacion("puno", &a).estado, 200);
    assert_eq!(c18::por_estacion("ilave", &a).estado, 404);
    assert_eq!(c18::por_estacion("Puno", &a).estado, 400);
    let contador = Arc::new(c18::Contador::default());
    let hilos: Vec<_> = (0..8)
        .map(|_| {
            let c = Arc::clone(&contador);
            thread::spawn(move || c.registrar())
        })
        .collect();
    hilos.into_iter().for_each(|h| h.join().unwrap());
    assert_eq!(contador.metricas().cuerpo, r#"{"peticiones":8}"#);
    assert_eq!(c19::dimensiones("800x600"), Ok((800, 600)));
    assert_eq!(
        c19::dimensiones("800"),
        Err(ErrorDimension::SinSeparador)
    );
    assert_eq!(
        c19::dimensiones("80ax6"),
        Err(ErrorDimension::NoEsNumero)
    );
    assert_eq!(
        c19::dimensiones("0x6"),
        Err(ErrorDimension::Cero)
    );
}
