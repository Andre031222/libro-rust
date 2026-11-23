use cap19_seguridad::fuzz::{fuzzear, minimizar};
use cap19_seguridad::mensajes::{carga, carga_fragil};

// El objetivo solo debe no entrar en pánico; el resultado
// no importa.
fn fragil(e: &[u8]) {
    let _ = carga_fragil(e);
}

fn corregida(e: &[u8]) {
    let _ = carga(e);
}

fn main() {
    let n = 1_000_000;
    match fuzzear(fragil, 42, n) {
        Some((i, e)) => {
            println!(
                "fallo en la entrada {i}: {}",
                e.escape_ascii()
            );
            let m = minimizar(fragil, e);
            println!("entrada mínima: {}", m.escape_ascii());
        }
        None => println!("ningún fallo en {n} entradas"),
    }
    match fuzzear(corregida, 42, n) {
        Some((i, _)) => println!("la corregida falla en {i}"),
        None => {
            println!("corregida: ningún fallo en {n} entradas")
        }
    }
}
