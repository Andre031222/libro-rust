//! Pruebas de los ejercicios del capítulo 18.
//! Ejecute `cargo test -p ej18-proyecto` hasta que pasen.

use cap18_proyecto::{Almacen, ErrorPeticion, Metodo, Peticion};
use ej18_proyecto::*;
use std::path::PathBuf;
use std::{fs, process, thread};

fn get(ruta: &str) -> Peticion {
    Peticion {
        metodo: Metodo::Get,
        ruta: ruta.to_owned(),
        campos: Vec::new(),
        cuerpo: Vec::new(),
    }
}

fn con_datos() -> Almacen {
    let a = Almacen::new();
    a.agregar("puno".into(), 10.0);
    a.agregar("juli".into(), 4.0);
    a.agregar("puno".into(), 13.0);
    a
}

#[test]
fn ruta_resumen() {
    let r = atender_ampliado(&get("/resumen"), &con_datos());
    assert_eq!(r.estado, 200);
    assert_eq!(r.cuerpo, r#"{"cantidad":3,"media":9}"#);
    let vacio = resumen(&Almacen::new());
    assert_eq!(vacio.cuerpo, r#"{"cantidad":0,"media":null}"#);
    // Las rutas anteriores siguen funcionando.
    let r = atender_ampliado(&get("/salud"), &con_datos());
    assert_eq!(r.cuerpo, "ok\n");
}

#[test]
fn estaciones_400_404_y_200() {
    let a = con_datos();
    let r = atender_ampliado(&get("/estaciones/puno"), &a);
    assert_eq!(r.estado, 200);
    let esperado = format!(
        "[{},{}]",
        a.obtener(1).unwrap().a_json(),
        a.obtener(3).unwrap().a_json()
    );
    assert_eq!(r.cuerpo, esperado);
    assert_eq!(por_estacion("ilave", &a).estado, 404);
    assert_eq!(por_estacion("Puno", &a).estado, 400);
    assert_eq!(por_estacion("", &a).estado, 400);
}

#[test]
fn cuerpo_limitado() {
    let mut datos = &b"abcdef"[..];
    assert_eq!(leer_cuerpo(&mut datos, 4, 10).unwrap(), b"abcd");
    let mut datos = &b"abcdef"[..];
    let e = leer_cuerpo(&mut datos, 11, 10).unwrap_err();
    assert!(matches!(e, ErrorPeticion::DemasiadoGrande));
    assert_eq!(datos.len(), 6, "no debe leer nada");
    let mut corto = &b"ab"[..];
    let e = leer_cuerpo(&mut corto, 5, 10).unwrap_err();
    assert!(matches!(e, ErrorPeticion::Incompleta));
    assert!(leer_cuerpo(&mut corto, 0, 0).unwrap().is_empty());
}

#[test]
fn contador_atomico_entre_hilos() {
    let contador = Contador::default();
    thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..1_000 {
                    contador.registrar();
                }
            });
        }
    });
    assert_eq!(contador.total(), 8_000);
    let r = contador.metricas();
    assert_eq!(r.estado, 200);
    assert_eq!(r.cuerpo, r#"{"peticiones":8000}"#);
}

fn archivo(nombre: &str) -> PathBuf {
    let dir = std::env::temp_dir();
    dir.join(format!("ej18-{}-{nombre}.tsv", process::id()))
}

#[test]
fn guardar_y_cargar() {
    let ruta = archivo("ida-vuelta");
    let original = con_datos();
    guardar(&original, &ruta).unwrap();
    let cargado = cargar(&ruta).unwrap();
    assert_eq!(cargado.todas(), original.todas());
    // Los ids nuevos continúan tras los cargados.
    assert_eq!(cargado.agregar("puno".into(), 1.0).id, 4);
    fs::remove_file(&ruta).unwrap();
}

#[test]
fn cargar_sin_archivo_da_almacen_vacio() {
    let ruta = archivo("no-existe");
    assert!(cargar(&ruta).unwrap().is_empty());
}

#[test]
fn cargar_detecta_lineas_rotas() {
    let ruta = archivo("rota");
    let texto = "1\tpuno\t2.5\n2\tPUNO\t1\n";
    fs::write(&ruta, texto).unwrap();
    let e = cargar(&ruta).map(|_| ()).unwrap_err();
    assert!(matches!(e, ErrorPersistencia::Formato { linea: 2 }));
    fs::write(&ruta, "1\tpuno\n").unwrap();
    let e = cargar(&ruta).map(|_| ()).unwrap_err();
    assert!(matches!(e, ErrorPersistencia::Formato { linea: 1 }));
    fs::remove_file(&ruta).unwrap();
}
