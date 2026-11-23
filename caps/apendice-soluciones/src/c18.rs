use cap18_proyecto::formato::validar_estacion;
use cap18_proyecto::{Almacen, Medicion, Respuesta};
use std::sync::atomic::{AtomicU64, Ordering};

// Ejercicio 18.5: primero se valida, luego se busca.
pub fn por_estacion(
    nombre: &str,
    almacen: &Almacen,
) -> Respuesta {
    if let Err(motivo) = validar_estacion(nombre) {
        return Respuesta::error(400, motivo);
    }
    let partes: Vec<String> = almacen
        .todas()
        .iter()
        .filter(|m| m.estacion == nombre)
        .map(Medicion::a_json)
        .collect();
    if partes.is_empty() {
        return Respuesta::error(404, "estacion sin mediciones");
    }
    Respuesta::json(200, format!("[{}]", partes.join(",")))
}

// Ejercicio 18.7: un contador que comparten todos los hilos.
#[derive(Debug, Default)]
pub struct Contador {
    total: AtomicU64,
}

impl Contador {
    pub fn registrar(&self) {
        self.total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn metricas(&self) -> Respuesta {
        let n = self.total.load(Ordering::Relaxed);
        Respuesta::json(200, format!(r#"{{"peticiones":{n}}}"#))
    }
}
