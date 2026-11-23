//! Rutas del servicio: de una petición a una respuesta.

use crate::almacen::{Almacen, Medicion};
use crate::formato::analizar_formulario;
use crate::peticion::{Metodo, Peticion};
use crate::respuesta::Respuesta;

/// Decide la respuesta. No toca la red: se prueba sin sockets.
pub fn atender(pet: &Peticion, almacen: &Almacen) -> Respuesta {
    // La consulta (`?...`) no se usa: se descarta.
    let ruta = pet.ruta.split('?').next().unwrap_or_default();
    match (pet.metodo, ruta) {
        (Metodo::Get, "/salud") => Respuesta::texto(200, "ok\n"),
        (Metodo::Get, "/mediciones") => listar(almacen),
        (Metodo::Post, "/mediciones") => crear(pet, almacen),
        (_, "/salud") => no_permitido("GET"),
        (Metodo::Get, _) => {
            match ruta.strip_prefix("/mediciones/") {
                Some(id) => obtener(id, almacen),
                None => Respuesta::error(404, "ruta desconocida"),
            }
        }
        (Metodo::Post, _) => no_permitido_o_404(ruta),
    }
}

fn listar(almacen: &Almacen) -> Respuesta {
    let todas = almacen.todas();
    let partes: Vec<String> =
        todas.iter().map(Medicion::a_json).collect();
    Respuesta::json(200, format!("[{}]", partes.join(",")))
}

fn crear(pet: &Peticion, almacen: &Almacen) -> Respuesta {
    match analizar_formulario(&pet.cuerpo) {
        Ok((estacion, valor)) => {
            let nueva = almacen.agregar(estacion, valor);
            let sitio = format!("/mediciones/{}", nueva.id);
            Respuesta::json(201, nueva.a_json())
                .con_campo("Location", &sitio)
        }
        Err(motivo) => Respuesta::error(400, motivo),
    }
}

fn obtener(id: &str, almacen: &Almacen) -> Respuesta {
    let Ok(id) = id.parse::<u64>() else {
        return Respuesta::error(400, "id debe ser un entero");
    };
    match almacen.obtener(id) {
        Some(m) => Respuesta::json(200, m.a_json()),
        None => Respuesta::error(404, "no existe esa medicion"),
    }
}

fn no_permitido(permitidos: &str) -> Respuesta {
    Respuesta::error(405, "metodo no permitido")
        .con_campo("Allow", permitidos)
}

fn no_permitido_o_404(ruta: &str) -> Respuesta {
    if ruta.starts_with("/mediciones/") {
        no_permitido("GET")
    } else {
        Respuesta::error(404, "ruta desconocida")
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn pedir(m: Metodo, ruta: &str, cuerpo: &str) -> Peticion {
        Peticion {
            metodo: m,
            ruta: ruta.to_owned(),
            campos: Vec::new(),
            cuerpo: cuerpo.as_bytes().to_vec(),
        }
    }

    #[test]
    fn crear_y_listar() {
        let a = Almacen::new();
        let cuerpo = "estacion=puno&valor=12.5";
        let r = atender(
            &pedir(Metodo::Post, "/mediciones", cuerpo),
            &a,
        );
        assert_eq!(r.estado, 201);
        assert_eq!(
            r.extra,
            [("Location", "/mediciones/1".into())]
        );
        let r =
            atender(&pedir(Metodo::Get, "/mediciones", ""), &a);
        assert_eq!(
            r.cuerpo,
            format!("[{}]", a.todas()[0].a_json())
        );
    }

    #[test]
    fn codigos_de_error() {
        let a = Almacen::new();
        let casos = [
            (Metodo::Get, "/mediciones/1", 404),
            (Metodo::Get, "/mediciones/uno", 400),
            (Metodo::Get, "/otra", 404),
            (Metodo::Post, "/salud", 405),
            (Metodo::Post, "/mediciones/1", 405),
            (Metodo::Post, "/otra", 404),
            (Metodo::Post, "/mediciones", 400),
        ];
        for (metodo, ruta, estado) in casos {
            let r = atender(&pedir(metodo, ruta, ""), &a);
            assert_eq!(r.estado, estado, "{metodo:?} {ruta}");
        }
    }
}
