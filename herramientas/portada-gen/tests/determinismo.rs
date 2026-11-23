use portada_gen::{Motivo, generar, generar_pdf};

const TODOS: [Motivo; 4] = [
    Motivo::Grafo,
    Motivo::Oxido,
    Motivo::Fractal,
    Motivo::Flujo,
];

#[test]
fn misma_semilla_mismo_svg() {
    assert_eq!(
        generar(42, Motivo::Grafo),
        generar(42, Motivo::Grafo)
    );
}

#[test]
fn otra_semilla_otro_svg() {
    for m in TODOS {
        assert_ne!(generar(42, m), generar(43, m));
    }
}

#[test]
fn proporcion_17_por_24() {
    for m in TODOS {
        assert!(
            generar(7, m).contains(r#"viewBox="0 0 1700 2400""#)
        );
    }
}

#[test]
fn usa_la_paleta_del_libro() {
    for m in TODOS {
        assert!(generar(7, m).contains("#C83C06"));
    }
}

#[test]
fn flujo_tiene_miles_de_trazos() {
    let svg = generar(7, Motivo::Flujo);
    assert!(svg.matches("<polyline").count() >= 1500);
}

#[test]
fn pdf_es_determinista() {
    for m in TODOS {
        assert_eq!(generar_pdf(5, m), generar_pdf(5, m));
    }
}

#[test]
fn pdf_tiene_cabecera_y_cierre() {
    let pdf = generar_pdf(5, Motivo::Flujo);
    assert!(pdf.starts_with(b"%PDF-1.4"));
    assert!(pdf.ends_with(b"%%EOF\n"));
}

#[test]
fn pdf_mide_17_por_24_cm() {
    let pdf =
        String::from_utf8_lossy(&generar_pdf(5, Motivo::Grafo))
            .into_owned();
    assert!(pdf.contains("/MediaBox [0 0 481.89 680.31]"));
}

#[test]
fn pdf_xref_apunta_a_la_tabla() {
    let pdf = generar_pdf(5, Motivo::Fractal);
    let texto = String::from_utf8_lossy(&pdf).into_owned();
    let i =
        texto.rfind("startxref\n").unwrap() + "startxref\n".len();
    let fin = texto[i..].find('\n').unwrap();
    let pos: usize = texto[i..i + fin].parse().unwrap();
    assert!(pdf[pos..].starts_with(b"xref"));
}

#[test]
fn pdf_offsets_de_objetos_son_exactos() {
    let pdf = generar_pdf(5, Motivo::Oxido);
    let texto = String::from_utf8_lossy(&pdf).into_owned();
    let xref = texto.rfind("\nxref\n").unwrap() + 1;
    // Saltar "xref\n0 N\n" y la entrada libre del objeto 0.
    let lineas: Vec<&str> = texto[xref..]
        .lines()
        .skip(3)
        .map(str::trim_end)
        .take_while(|l| l.ends_with(" n"))
        .collect();
    assert_eq!(lineas.len(), 4, "el PDF tiene 4 objetos");
    for (k, l) in lineas.iter().enumerate() {
        let pos: usize = l[..10].parse().unwrap();
        let esperado = format!("{} 0 obj", k + 1);
        assert!(
            pdf[pos..].starts_with(esperado.as_bytes()),
            "obj {}",
            k + 1
        );
    }
}
