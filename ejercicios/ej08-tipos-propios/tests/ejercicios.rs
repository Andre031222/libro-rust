//! Pruebas de los ejercicios del capítulo 8.
//! Ejecute `cargo test -p ej08-tipos-propios` hasta que todas
//! pasen.

use ej08_tipos_propios::*;

#[test]
fn rectangulo_area_y_cuadrado() {
    assert_eq!(Rectangulo::new(3, 4).area(), 12);
    assert_eq!(Rectangulo::cuadrado(5), Rectangulo::new(5, 5));
}

#[test]
fn rectangulo_puede_contener() {
    let grande = Rectangulo::new(10, 8);
    assert!(grande.puede_contener(&Rectangulo::new(9, 7)));
    assert!(!grande.puede_contener(&Rectangulo::new(11, 1)));
    assert!(!grande.puede_contener(&grande));
}

#[test]
fn rectangulo_escalar() {
    let mut r = Rectangulo::new(2, 3);
    r.escalar(3);
    assert_eq!(r, Rectangulo::new(6, 9));
}

#[test]
fn figura_areas() {
    let c = Figura::Circulo { radio: 1.0 };
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-12);
    let r = Figura::Rectangulo {
        ancho: 2.0,
        alto: 3.5,
    };
    assert_eq!(r.area(), 7.0);
    let t = Figura::Triangulo {
        base: 4.0,
        altura: 3.0,
    };
    assert_eq!(t.area(), 6.0);
}

#[test]
fn puerta_transiciones_validas() {
    use Accion::*;
    use Puerta::*;
    assert_eq!(transicion(Abierta, Cerrar), Cerrada);
    assert_eq!(transicion(Cerrada, EcharLlave), ConLlave);
    assert_eq!(transicion(ConLlave, QuitarLlave), Cerrada);
    assert_eq!(transicion(Cerrada, Abrir), Abierta);
}

#[test]
fn puerta_acciones_invalidas_no_cambian() {
    use Accion::*;
    use Puerta::*;
    assert_eq!(transicion(ConLlave, Abrir), ConLlave);
    assert_eq!(transicion(Abierta, EcharLlave), Abierta);
    assert_eq!(transicion(Abierta, Abrir), Abierta);
}

fn num(x: f64) -> Box<Expr> {
    Box::new(Expr::Num(x))
}

#[test]
fn evaluar_expresion_anidada() {
    // (2 + 3) * (10 - 4) / 3 = 10
    let e = Expr::Div(
        Box::new(Expr::Mul(
            Box::new(Expr::Suma(num(2.0), num(3.0))),
            Box::new(Expr::Resta(num(10.0), num(4.0))),
        )),
        num(3.0),
    );
    assert_eq!(evaluar(&e), Some(10.0));
}

#[test]
fn evaluar_division_entre_cero() {
    let e = Expr::Suma(
        num(1.0),
        Box::new(Expr::Div(num(1.0), num(0.0))),
    );
    assert_eq!(evaluar(&e), None);
}
