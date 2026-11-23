//! Rutas construidas con un nombre que llega del usuario.

use std::path::{Component, Path, PathBuf};

/// Versión vulnerable: une `nombre` a `base` sin revisarlo.
pub fn ruta_ingenua(base: &Path, nombre: &str) -> PathBuf {
    base.join(nombre)
}

/// Motivo por el que se rechaza un nombre.
#[derive(Debug, PartialEq, Eq)]
pub enum RutaInvalida {
    Vacia,
    SaleDeLaBase,
}

/// Versión corregida: solo acepta componentes normales, así
/// que el resultado nunca sale de `base`.
pub fn ruta_segura(
    base: &Path,
    nombre: &str,
) -> Result<PathBuf, RutaInvalida> {
    let relativa = Path::new(nombre);
    if relativa.as_os_str().is_empty() {
        return Err(RutaInvalida::Vacia);
    }
    for parte in relativa.components() {
        match parte {
            Component::Normal(_) => {}
            // `..`, `.`, la raíz `/` o un prefijo como `C:`
            _ => return Err(RutaInvalida::SaleDeLaBase),
        }
    }
    Ok(base.join(relativa))
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const BASE: &str = "/srv/informes";

    #[test]
    fn la_version_ingenua_escapa() {
        let base = Path::new(BASE);
        let r = ruta_ingenua(base, "../../etc/passwd");
        assert!(
            r.components().any(|c| c == Component::ParentDir)
        );
        // Un nombre absoluto reemplaza a la base entera.
        let r = ruta_ingenua(base, "/etc/passwd");
        assert_eq!(r, PathBuf::from("/etc/passwd"));
    }

    #[test]
    fn la_version_segura_rechaza() {
        let base = Path::new(BASE);
        for malo in ["../x", "a/../../x", "/etc/passwd", "./a"] {
            assert_eq!(
                ruta_segura(base, malo),
                Err(RutaInvalida::SaleDeLaBase),
                "{malo}"
            );
        }
        assert_eq!(
            ruta_segura(base, ""),
            Err(RutaInvalida::Vacia)
        );
    }

    #[test]
    fn la_version_segura_acepta() {
        let base = Path::new(BASE);
        let r = ruta_segura(base, "2026/marzo.txt").unwrap();
        assert_eq!(
            r,
            PathBuf::from("/srv/informes/2026/marzo.txt")
        );
        assert!(r.starts_with(base));
    }
}
