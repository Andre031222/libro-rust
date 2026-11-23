//! Pruebas de los ejercicios del capítulo 9.
//! Ejecute `cargo test -p ej09-errores` hasta que todas pasen.

use ej09_errores::*;

#[test]
fn parsear_numero_con_espacios() {
    assert_eq!(parsear_numero("  -17 "), Ok(-17));
}

#[test]
fn parsear_numero_invalido_es_err() {
    assert!(parsear_numero("12a").is_err());
    assert!(parsear_numero("").is_err());
}

#[test]
fn leer_config_valida() {
    let texto = "puerto = 8080\n\nhost=localhost\n";
    let pares = leer_config(texto).unwrap();
    assert_eq!(
        pares,
        vec![
            ("puerto".to_string(), "8080".to_string()),
            ("host".to_string(), "localhost".to_string()),
        ]
    );
}

#[test]
fn leer_config_sin_igual() {
    let r = leer_config("a=1\nb 2");
    assert_eq!(r, Err(ErrorConfig::SinIgual { linea: 2 }));
}

#[test]
fn leer_config_clave_vacia() {
    let r = leer_config(" = 5");
    assert_eq!(r, Err(ErrorConfig::ClaveVacia { linea: 1 }));
}

#[test]
fn mensajes_de_error_config() {
    let e = ErrorConfig::SinIgual { linea: 3 };
    assert_eq!(e.to_string(), "línea 3: falta '='");
    let e = ErrorConfig::ClaveVacia { linea: 4 };
    assert_eq!(e.to_string(), "línea 4: clave vacía");
}

#[test]
fn sumar_valores_correctos() {
    assert_eq!(sumar_valores("a=1\nb=2\nc=-4").unwrap(), -1);
}

#[test]
fn sumar_valores_propaga_ambos_errores() {
    let e = sumar_valores("a=1\nb").unwrap_err();
    assert_eq!(e.to_string(), "línea 2: falta '='");
    let e = sumar_valores("a=uno").unwrap_err();
    assert_eq!(e.to_string(), "invalid digit found in string");
}

#[test]
fn validar_usuario_correcto() {
    let u = validar(" Ana ", "30", "ana@ejemplo.org").unwrap();
    assert_eq!(u.nombre, "Ana");
    assert_eq!(u.edad, 30);
    assert_eq!(u.correo, "ana@ejemplo.org");
}

#[test]
fn validar_convierte_cada_error() {
    assert_eq!(
        validar("  ", "30", "a@b"),
        Err(ErrorValidacion::Nombre(CampoVacio))
    );
    assert!(matches!(
        validar("Ana", "300", "a@b"),
        Err(ErrorValidacion::Edad(_))
    ));
    assert_eq!(
        validar("Ana", "30", "ana.org"),
        Err(ErrorValidacion::Correo(CorreoInvalido(
            "ana.org".to_string()
        )))
    );
}
