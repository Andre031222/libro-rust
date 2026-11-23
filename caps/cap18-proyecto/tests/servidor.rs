//! Pruebas de integración: un servidor real en 127.0.0.1:0.

use cap18_proyecto::{Almacen, Apagador, Servidor};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn arrancar() -> (SocketAddr, Apagador, JoinHandle<()>) {
    let almacen = Arc::new(Almacen::new());
    let servidor = Servidor::iniciar("127.0.0.1:0", 2, almacen)
        .expect("abrir un puerto libre")
        .con_espera(Duration::from_millis(300));
    let dir = servidor.direccion().unwrap();
    let apagador = servidor.apagador().unwrap();
    (dir, apagador, thread::spawn(move || servidor.ejecutar()))
}

/// Envía `texto` y lee la respuesta hasta que el servidor cierra.
fn pedir(dir: SocketAddr, texto: &str) -> String {
    let mut cliente = TcpStream::connect(dir).unwrap();
    let espera = Some(Duration::from_secs(5));
    cliente.set_read_timeout(espera).unwrap();
    cliente.write_all(texto.as_bytes()).unwrap();
    let mut respuesta = String::new();
    cliente.read_to_string(&mut respuesta).unwrap();
    respuesta
}

#[test]
fn crea_y_consulta_una_medicion() {
    let (dir, apagador, hilo) = arrancar();
    let r = pedir(
        dir,
        "POST /mediciones HTTP/1.1\r\nHost: prueba\r\n\
         Content-Length: 24\r\n\r\nestacion=puno&valor=12.5",
    );
    let json = r#"{"id":1,"estacion":"puno","valor":12.5}"#;
    assert!(r.starts_with("HTTP/1.1 201 Created\r\n"), "{r}");
    assert!(r.contains("Location: /mediciones/1\r\n"));
    assert!(r.ends_with(json));
    let r = pedir(dir, "GET /mediciones/1 HTTP/1.1\r\n\r\n");
    assert!(r.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(r.ends_with(json));
    apagador.apagar();
    hilo.join().unwrap();
}

#[test]
fn responde_errores_sin_caerse() {
    let (dir, apagador, hilo) = arrancar();
    let casos = [
        ("GET /nada HTTP/1.1\r\n\r\n", "404 Not Found"),
        ("BASURA\r\n\r\n", "400 Bad Request"),
        ("DELETE / HTTP/1.1\r\n\r\n", "501 Not Implemented"),
        (
            "POST /mediciones HTTP/1.1\r\n\
             Content-Length: 999999\r\n\r\n",
            "413 Content Too Large",
        ),
        ("GET /salud HTT", "408 Request Timeout"),
    ];
    for (peticion, estado) in casos {
        let r = pedir(dir, peticion);
        assert!(
            r.starts_with(&format!("HTTP/1.1 {estado}")),
            "{r}"
        );
    }
    // Después de los errores el servicio sigue sano.
    assert!(
        pedir(dir, "GET /salud HTTP/1.1\r\n\r\n")
            .ends_with("ok\n")
    );
    apagador.apagar();
    hilo.join().unwrap();
}
