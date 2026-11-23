# Código del libro de Rust

Ejemplos, ejercicios y herramientas del libro *Óxido. Rust para escribir software que no se rompe*, de Richar Andre Vilca-Solorzano (LaCima, FINESI-UNAP).

El texto del libro **no** está en este repositorio; aquí solo vive el código que el libro cita, para que puedas ejecutarlo y modificarlo.

## Requisitos

- Rust 1.96 o posterior (`rustup` instala la versión fijada en `rust-toolchain.toml`).

## Organización

| Carpeta | Contenido |
| --- | --- |
| `caps/capNN-tema/` | ejemplos del capítulo NN |
| `ejercicios/ejNN-tema/` | ejercicios del capítulo NN con pruebas automáticas |
| `figuras/` | figuras propias del libro y su registro de créditos |
| `herramientas/` | verificador del manuscrito y generador de la portada |

## Uso

Ejecutar un ejemplo:

```bash
cargo run -p cap06-propiedad --example movimiento
```

Resolver los ejercicios: reemplaza cada `todo!()` y corre las pruebas hasta que pasen.

```bash
cargo test -p ej06-propiedad
```

## Licencia

El código se distribuye bajo la licencia MIT o Apache-2.0, a elección del usuario (`LICENSE-MIT`, `LICENSE-APACHE`). El texto y las figuras del libro tienen todos los derechos reservados.

Excepción: `caps/cap17-so/linux/rust_minimal_extracto.rs` es un extracto breve de `samples/rust/rust_minimal.rs` del núcleo Linux (Rust for Linux Contributors) y conserva su licencia original, GPL-2.0.
