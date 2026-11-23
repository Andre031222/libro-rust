//! Pruebas de los ejercicios del capítulo 12.
//! Ejecute `cargo test -p ej12-modulos` hasta que pasen.

use ej12_modulos::texto::es_palindromo;
use ej12_modulos::{ErrorPorcentaje, Porcentaje, porcentaje_de};

#[test]
fn palindromos_con_signos_y_mayusculas() {
    assert!(es_palindromo("Anita lava la tina"));
    assert!(es_palindromo("¡Oso!"));
    assert!(es_palindromo(""));
    assert!(!es_palindromo("Rust"));
}

#[test]
fn porcentaje_respeta_el_rango() -> Result<(), ErrorPorcentaje> {
    assert_eq!(Porcentaje::new(0)?.valor(), 0);
    assert_eq!(Porcentaje::new(100)?.valor(), 100);
    assert_eq!(
        Porcentaje::new(101),
        Err(ErrorPorcentaje::FueraDeRango)
    );
    Ok(())
}

#[test]
fn complemento_y_aplicar() -> Result<(), ErrorPorcentaje> {
    let p = Porcentaje::new(18)?;
    assert_eq!(p.complemento().valor(), 82);
    assert_eq!(p.aplicar(1000), 180);
    assert_eq!(p.aplicar(99), 17);
    assert_eq!(Porcentaje::new(100)?.aplicar(u32::MAX), u32::MAX);
    Ok(())
}

#[test]
fn porcentaje_de_parte_y_total() {
    assert_eq!(porcentaje_de(1, 3).map(|p| p.valor()), Ok(33));
    assert_eq!(porcentaje_de(7, 7).map(|p| p.valor()), Ok(100));
    assert_eq!(
        porcentaje_de(0, 0),
        Err(ErrorPorcentaje::TotalCero)
    );
    assert_eq!(
        porcentaje_de(5, 4),
        Err(ErrorPorcentaje::FueraDeRango)
    );
    let grande = porcentaje_de(u32::MAX, u32::MAX);
    assert_eq!(grande.map(|p| p.valor()), Ok(100));
}

#[test]
fn desde_texto_acepta_solo_la_forma_correcta() {
    let leer = |t| Porcentaje::desde_texto(t).map(|p| p.valor());
    assert_eq!(leer("45%"), Ok(45));
    assert_eq!(leer("0%"), Ok(0));
    assert_eq!(leer("101%"), Err(ErrorPorcentaje::FueraDeRango));
    assert_eq!(leer("45"), Err(ErrorPorcentaje::Formato));
    assert_eq!(leer(" 45%"), Err(ErrorPorcentaje::Formato));
    assert_eq!(leer("x%"), Err(ErrorPorcentaje::Formato));
}
