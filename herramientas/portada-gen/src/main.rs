//! Uso: portada-gen --semilla <u64> --motivo <grafo|oxido|fractal>
//!                  --salida <ruta.svg>

use portada_gen::{Motivo, generar};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let valor = |nombre: &str| {
        args.iter()
            .position(|a| a == nombre)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let semilla =
        valor("--semilla").and_then(|s| s.parse::<u64>().ok());
    let motivo = match valor("--motivo").as_deref() {
        Some("grafo") => Some(Motivo::Grafo),
        Some("oxido") => Some(Motivo::Oxido),
        Some("fractal") => Some(Motivo::Fractal),
        _ => None,
    };
    let (Some(semilla), Some(motivo), Some(salida)) =
        (semilla, motivo, valor("--salida"))
    else {
        eprintln!(
            "uso: portada-gen --semilla <u64> \
             --motivo <grafo|oxido|fractal> --salida <ruta.svg>"
        );
        return ExitCode::FAILURE;
    };
    match std::fs::write(&salida, generar(semilla, motivo)) {
        Ok(()) => {
            println!("portada escrita en {salida}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("no se pudo escribir {salida}: {e}");
            ExitCode::FAILURE
        }
    }
}
