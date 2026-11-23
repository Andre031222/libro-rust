//! `Box<dyn Error>`: un tipo para errores de distinto origen.
use std::error::Error;

fn precio_total(
    cant: &str,
    precio: &str,
) -> Result<f64, Box<dyn Error>> {
    let c: u32 = cant.parse()?; // ParseIntError
    let p: f64 = precio.parse()?; // ParseFloatError
    Ok(f64::from(c) * p)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("total: {}", precio_total("3", "2.5")?);
    println!("total: {}", precio_total("3", "dos")?);
    println!("esta línea no se ejecuta");
    Ok(())
}
