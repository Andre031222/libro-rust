//! `Option`: «no hay valor»; `Result`: «falló y por qué».

fn main() {
    let notas = [15, 18, 12];

    // Option: buscar no es un error, el valor puede no estar.
    println!("{:?}", notas.get(1)); // Some(18)
    println!("{:?}", notas.get(7)); // None

    // Result: convertir texto puede fallar por una causa.
    println!("{:?}", "18".parse::<u8>());
    println!("{:?}", "300".parse::<u8>());

    // De Option a Result: ok_or le da una causa a la ausencia.
    let r: Result<&i32, &str> =
        notas.get(7).ok_or("no hay nota 7");
    println!("{r:?}");

    // unwrap_or da un valor por defecto, sin pánico.
    println!("{}", notas.get(7).unwrap_or(&0));
}
