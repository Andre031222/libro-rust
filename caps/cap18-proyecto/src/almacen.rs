//! Almacén en memoria, compartido entre hilos.

use crate::formato::escapar_json;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// Una medición de una estación.
#[derive(Debug, Clone, PartialEq)]
pub struct Medicion {
    pub id: u64,
    pub estacion: String,
    pub valor: f64,
}

impl Medicion {
    /// Representación JSON, escrita a mano.
    pub fn a_json(&self) -> String {
        let estacion = escapar_json(&self.estacion);
        format!(
            r#"{{"id":{},"estacion":"{estacion}","valor":{}}}"#,
            self.id, self.valor
        )
    }
}

/// Lo que protege el cerrojo: los datos y el próximo id.
#[derive(Debug, Default)]
struct Interior {
    ultimo_id: u64,
    datos: Vec<Medicion>,
}

/// Se comparte como `Arc<Almacen>`: el `Mutex` está dentro.
#[derive(Debug, Default)]
pub struct Almacen {
    interior: Mutex<Interior>,
}

impl Almacen {
    /// Almacén vacío.
    pub fn new() -> Self {
        Self::default()
    }

    /// Almacén con datos previos; los ids nuevos siguen al mayor.
    pub fn desde(datos: Vec<Medicion>) -> Self {
        let ultimo_id = datos.iter().map(|m| m.id).max();
        let interior = Interior {
            ultimo_id: ultimo_id.unwrap_or(0),
            datos,
        };
        Self {
            interior: Mutex::new(interior),
        }
    }

    /// Toma el cerrojo. Ninguna sección crítica deja los datos a
    /// medias, así que un cerrojo envenenado se puede recuperar.
    fn cerrojo(&self) -> MutexGuard<'_, Interior> {
        self.interior
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Guarda una medición y la devuelve con su id.
    pub fn agregar(
        &self,
        estacion: String,
        valor: f64,
    ) -> Medicion {
        let mut interior = self.cerrojo();
        interior.ultimo_id += 1;
        let id = interior.ultimo_id;
        let nueva = Medicion {
            id,
            estacion,
            valor,
        };
        interior.datos.push(nueva.clone());
        nueva
    }

    /// Busca una medición por id.
    pub fn obtener(&self, id: u64) -> Option<Medicion> {
        let interior = self.cerrojo();
        interior.datos.iter().find(|m| m.id == id).cloned()
    }

    /// Copia de todas las mediciones, en orden de llegada.
    pub fn todas(&self) -> Vec<Medicion> {
        self.cerrojo().datos.clone()
    }

    /// Número de mediciones.
    pub fn len(&self) -> usize {
        self.cerrojo().datos.len()
    }

    /// `true` si no hay mediciones.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn asigna_ids_consecutivos() {
        let a = Almacen::new();
        assert_eq!(a.agregar("puno".into(), 1.0).id, 1);
        assert_eq!(a.agregar("juli".into(), 2.5).id, 2);
        assert_eq!(a.obtener(2).unwrap().estacion, "juli");
        assert_eq!(a.obtener(3), None);
    }

    #[test]
    fn continua_tras_datos_previos() {
        let previa = Medicion {
            id: 7,
            estacion: "puno".into(),
            valor: 0.5,
        };
        let a = Almacen::desde(vec![previa]);
        assert_eq!(a.agregar("puno".into(), 1.0).id, 8);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn json_de_una_medicion() {
        let m = Medicion {
            id: 1,
            estacion: "puno".into(),
            valor: 12.5,
        };
        let esperado =
            r#"{"id":1,"estacion":"puno","valor":12.5}"#;
        assert_eq!(m.a_json(), esperado);
    }
}
