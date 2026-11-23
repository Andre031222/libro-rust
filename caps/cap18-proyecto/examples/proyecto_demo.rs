// El servicio en un puerto libre y tres peticiones.
use cap18_proyecto::{Almacen, Servidor};
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::{sync::Arc, thread};
fn pedir(dir: SocketAddr, peticion: &str) -> Result<()> {
    let mut cliente = TcpStream::connect(dir)?;
    cliente.write_all(peticion.as_bytes())?;
    let mut r = String::new();
    cliente.read_to_string(&mut r)?;
    let (cabecera, cuerpo) =
        r.split_once("\r\n\r\n").unwrap_or_default();
    let estado = cabecera.lines().next().unwrap_or_default();
    println!("{estado}\n    {cuerpo}");
    Ok(())
}
fn main() -> Result<()> {
    let almacen = Arc::new(Almacen::new());
    let servidor = Servidor::iniciar("127.0.0.1:0", 2, almacen)?;
    let (dir, apagador) =
        (servidor.direccion()?, servidor.apagador()?);
    let hilo = thread::spawn(move || servidor.ejecutar());
    let post = "POST /mediciones HTTP/1.1\r\n\
        Content-Length: 24\r\n\r\nestacion=puno&valor=12.5";
    pedir(dir, post)?;
    pedir(dir, "GET /mediciones HTTP/1.1\r\n\r\n")?;
    pedir(dir, "GET /mediciones/7 HTTP/1.1\r\n\r\n")?;
    apagador.apagar();
    let _ = hilo.join();
    Ok(())
}
