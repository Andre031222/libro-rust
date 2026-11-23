//! Pruebas de los ejercicios del capítulo 11.
//! Ejecute `cargo test -p ej11-rasgos` hasta que pasen.

use ej11_rasgos::*;

fn cerca(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn area_de_rectangulo_y_circulo() {
    let r = Rectangulo {
        ancho: 3.0,
        alto: 4.5,
    };
    let c = Circulo { radio: 2.0 };
    assert!(cerca(r.area(), 13.5));
    assert!(cerca(c.area(), 4.0 * std::f64::consts::PI));
}

#[test]
fn metodo_por_defecto_usa_area() {
    let r = Rectangulo {
        ancho: 2.0,
        alto: 2.0,
    };
    assert!(r.es_mayor_que(3.9));
    assert!(!r.es_mayor_que(4.0));
}

#[test]
fn mayor_con_varios_tipos() {
    assert_eq!(mayor(3, 8), 8);
    assert_eq!(mayor(-1.5, -2.5), -1.5);
    assert_eq!(mayor("pera", "manzana"), "pera");
    assert_eq!(mayor('a', 'a'), 'a');
}

#[test]
fn mayor_devuelve_a_si_empatan() {
    let a = String::from("x");
    let b = String::from("x");
    let pa = a.as_ptr();
    let m = mayor(a, b);
    assert_eq!(m.as_ptr(), pa);
}

#[test]
fn fraccion_display_y_from() {
    let f = Fraccion { num: 3, den: 4 };
    assert_eq!(f.to_string(), "3/4");
    let g = Fraccion::from(5);
    assert_eq!(g, Fraccion { num: 5, den: 1 });
    assert_eq!(format!("{g}"), "5");
    let h: Fraccion = (-2).into();
    assert_eq!(h.to_string(), "-2");
}

#[test]
fn coleccion_heterogenea() {
    let figuras: Vec<Box<dyn Area>> = vec![
        Box::new(Rectangulo {
            ancho: 2.0,
            alto: 3.0,
        }),
        Box::new(Circulo { radio: 1.0 }),
        Box::new(Rectangulo {
            ancho: 1.0,
            alto: 1.0,
        }),
    ];
    let esperado = 7.0 + std::f64::consts::PI;
    assert!(cerca(area_total(&figuras), esperado));
    assert_eq!(contar_grandes(&figuras, 3.0), 2);
    assert_eq!(area_total(&[]), 0.0);
}

#[test]
fn fibonacci_acotado() {
    let v: Vec<u64> = fibonacci(21).collect();
    assert_eq!(v, [0, 1, 1, 2, 3, 5, 8, 13, 21]);
    assert_eq!(fibonacci(0).collect::<Vec<_>>(), [0]);
    assert_eq!(fibonacci(4).count(), 5);
}

#[test]
fn fibonacci_con_adaptadores() {
    assert_eq!(suma_pares_fibonacci(100), 2 + 8 + 34);
    assert_eq!(suma_pares_fibonacci(4_000_000), 4_613_732);
    let tres: Vec<u64> =
        fibonacci(1000).skip(10).take(3).collect();
    assert_eq!(tres, [55, 89, 144]);
}

#[test]
fn fibonacci_no_desborda() {
    // El último término que cabe en u64 debe aparecer y el
    // iterador debe terminar sin desbordar.
    let ultimo = fibonacci(u64::MAX).last();
    assert_eq!(ultimo, Some(12_200_160_415_121_876_738));
}
