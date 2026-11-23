use cap19_seguridad::mensajes::{carga, carga_fragil};

fn main() {
    // El segundo mensaje anuncia 255 bytes y trae 5
    let cola: [&[u8]; 3] =
        [b"RS\x02hi", b"RS\xffcorto", b"RS\x03fin"];

    println!("-- versión corregida --");
    for m in cola {
        match carga(m) {
            Ok(c) => println!("atendido: {}", c.escape_ascii()),
            Err(e) => println!("rechazado: {e:?}"),
        }
    }

    println!("-- versión frágil --");
    for m in cola {
        let c = carga_fragil(m).unwrap_or_default();
        println!("atendido: {}", c.escape_ascii());
    }
    println!("fin de la cola"); // nunca se imprime
}
