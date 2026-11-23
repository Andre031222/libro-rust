//! `Result` obliga a decidir qué hacer con cada resultado.

fn main() {
    for texto in ["42", "4x", ""] {
        // parse devuelve Result<i32, ParseIntError>
        match texto.parse::<i32>() {
            Ok(n) => println!("{texto:?} -> Ok({n})"),
            Err(e) => println!("{texto:?} -> Err: {e}"),
        }
    }
}
