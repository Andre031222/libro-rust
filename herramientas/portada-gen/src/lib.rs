//! Generador del arte de la portada: un programa en Rust que dibuja
//! la portada del libro de Rust. Produce SVG (para Canva) y PDF
//! vectorial (para componer la portada en LaTeX), sin texto. El
//! resultado depende solo de la semilla, así que cualquier portada se
//! puede regenerar idéntica.

mod azar;
mod escena;
mod motivos;

pub use escena::{ALTO, ANCHO};

use azar::Azar;
use escena::Escena;

/// Motivos disponibles para el arte central.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motivo {
    /// Nodos de propiedad unidos por flechas de préstamo.
    Grafo,
    /// Capas de manchas que imitan óxido sobre metal.
    Oxido,
    /// Hexágonos recursivos, como un caparazón.
    Fractal,
    /// Miles de trazos guiados por un campo de ruido.
    Flujo,
}

fn escena(semilla: u64, motivo: Motivo) -> Escena {
    let mut azar = Azar::new(semilla);
    let mut e = Escena::default();
    match motivo {
        Motivo::Grafo => motivos::grafo(&mut e, &mut azar),
        Motivo::Oxido => motivos::oxido(&mut e, &mut azar),
        Motivo::Fractal => motivos::fractal(&mut e, &mut azar),
        Motivo::Flujo => {
            motivos::flujo(&mut e, &mut azar, semilla)
        }
    }
    e
}

/// Genera el SVG completo de la portada.
pub fn generar(semilla: u64, motivo: Motivo) -> String {
    escena(semilla, motivo).svg()
}

/// Genera la misma portada como PDF vectorial de 17 × 24 cm.
pub fn generar_pdf(semilla: u64, motivo: Motivo) -> Vec<u8> {
    escena(semilla, motivo).pdf()
}
