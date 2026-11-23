//! Binario `mediciones`: arranca el servicio.
//! Uso: `mediciones [DIRECCION]` (por defecto 127.0.0.1:8080).

use cap18_proyecto::{Almacen, Servidor};
use std::process::ExitCode;
use std::sync::Arc;
use std::{env, thread};

fn main() -> ExitCode {
    let dir = env::args().nth(1);
    let dir = dir.as_deref().unwrap_or("127.0.0.1:8080");
    let hilos =
        thread::available_parallelism().map_or(4, |n| n.get());
    let almacen = Arc::new(Almacen::new());
    match Servidor::iniciar(dir, hilos, almacen) {
        Ok(servidor) => {
            eprintln!("mediciones: {dir}, {hilos} hilos");
            servidor.con_registro().ejecutar();
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("mediciones: no se pudo abrir {dir}: {e}");
            ExitCode::FAILURE
        }
    }
}
