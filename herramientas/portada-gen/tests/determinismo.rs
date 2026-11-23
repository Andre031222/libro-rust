use portada_gen::{Motivo, generar};

#[test]
fn misma_semilla_mismo_svg() {
    assert_eq!(generar(42, Motivo::Grafo), generar(42, Motivo::Grafo));
}

#[test]
fn otra_semilla_otro_svg() {
    assert_ne!(generar(42, Motivo::Grafo), generar(43, Motivo::Grafo));
}

#[test]
fn proporcion_17_por_24() {
    for m in [Motivo::Grafo, Motivo::Oxido, Motivo::Fractal] {
        assert!(generar(7, m).contains(r#"viewBox="0 0 1700 2400""#));
    }
}

#[test]
fn usa_la_paleta_del_libro() {
    for m in [Motivo::Grafo, Motivo::Oxido, Motivo::Fractal] {
        assert!(generar(7, m).contains("#B7410E"));
    }
}
