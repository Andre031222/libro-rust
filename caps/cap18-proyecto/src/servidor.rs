//! El servidor: acepta conexiones y las reparte entre hilos.

use crate::almacen::Almacen;
use crate::error::ErrorPeticion;
use crate::grupo::GrupoHilos;
use crate::peticion::leer_peticion;
use crate::respuesta::Respuesta;
use crate::rutas;
use std::io;
use std::net::{
    SocketAddr, TcpListener, TcpStream, ToSocketAddrs,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// Espera máxima de lectura y escritura por conexión.
pub const ESPERA: Duration = Duration::from_secs(5);

/// Un servidor ligado a una dirección, aún sin atender.
pub struct Servidor {
    escucha: TcpListener,
    almacen: Arc<Almacen>,
    hilos: usize,
    espera: Duration,
    registro: bool,
    apagar: Arc<AtomicBool>,
}

impl Servidor {
    /// Abre el puerto. Con el puerto 0, el sistema elige uno.
    pub fn iniciar(
        dir: impl ToSocketAddrs,
        hilos: usize,
        almacen: Arc<Almacen>,
    ) -> io::Result<Self> {
        Ok(Self {
            escucha: TcpListener::bind(dir)?,
            almacen,
            hilos: hilos.max(1),
            espera: ESPERA,
            registro: false,
            apagar: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Cambia la espera máxima por conexión (no puede ser 0).
    pub fn con_espera(mut self, espera: Duration) -> Self {
        self.espera = espera.max(Duration::from_millis(1));
        self
    }

    /// Escribe una línea por petición en la salida de error.
    pub fn con_registro(mut self) -> Self {
        self.registro = true;
        self
    }

    /// La dirección real, útil si se pidió el puerto 0.
    pub fn direccion(&self) -> io::Result<SocketAddr> {
        self.escucha.local_addr()
    }

    /// Un mando para detener el servidor desde otro hilo.
    pub fn apagador(&self) -> io::Result<Apagador> {
        let bandera = Arc::clone(&self.apagar);
        Ok(Apagador {
            bandera,
            dir: self.direccion()?,
        })
    }

    /// Atiende conexiones hasta que se use el apagador.
    pub fn ejecutar(self) {
        let grupo = GrupoHilos::new(self.hilos);
        for conexion in self.escucha.incoming() {
            if self.apagar.load(Ordering::SeqCst) {
                break;
            }
            let Ok(flujo) = conexion else {
                // Sin descriptores (EMFILE), accept falla al
                // instante: una pausa evita girar en vacío.
                std::thread::sleep(Duration::from_millis(10));
                continue;
            };
            let almacen = Arc::clone(&self.almacen);
            let (espera, registro) = (self.espera, self.registro);
            grupo.ejecutar(move || {
                atender_conexion(
                    flujo, &almacen, espera, registro,
                );
            });
        }
    } // aquí se libera `grupo`, que espera a sus hilos
}

/// Lee una petición, la responde y cierra la conexión.
fn atender_conexion(
    mut flujo: TcpStream,
    almacen: &Almacen,
    espera: Duration,
    registro: bool,
) {
    let limites = flujo
        .set_read_timeout(Some(espera))
        .and_then(|()| flujo.set_write_timeout(Some(espera)));
    if limites.is_err() {
        return;
    }
    let (resumen, respuesta) = match leer_peticion(&mut flujo) {
        Ok(pet) => {
            let r = rutas::atender(&pet, almacen);
            (format!("{} {}", pet.metodo.nombre(), pet.ruta), r)
        }
        // La conexión ya no sirve: no hay a quién responder.
        Err(ErrorPeticion::Io(_)) => return,
        Err(e) => {
            let r = Respuesta::error(e.estado(), &e.to_string());
            (e.to_string(), r)
        }
    };
    if registro {
        eprintln!("{resumen} -> {}", respuesta.estado);
    }
    let _ = respuesta.escribir(&mut flujo);
}

/// Detiene un servidor en marcha.
#[derive(Debug, Clone)]
pub struct Apagador {
    bandera: Arc<AtomicBool>,
    dir: SocketAddr,
}

impl Apagador {
    /// Marca el apagado y despierta a `incoming` conectándose.
    pub fn apagar(&self) {
        self.bandera.store(true, Ordering::SeqCst);
        // En Windows no se conecta a 0.0.0.0: use 127.0.0.1.
        let _ = TcpStream::connect(self.dir);
    }
}
