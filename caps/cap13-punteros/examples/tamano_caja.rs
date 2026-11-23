//! Una caja mide lo mismo sea cual sea su contenido.

fn main() {
    // Un arreglo de un megabyte, guardado en el montículo.
    let grande: Box<[u8; 1_000_000]> = Box::new([7; 1_000_000]);
    let otra = grande; // mueve 8 bytes, no un megabyte
    println!("primer byte: {}", otra[0]);
    println!("Box<[u8; 1000000]>: {} bytes", size_of_val(&otra));

    // Tipos de tamaño desconocido al compilar: str y [i32].
    let texto: Box<str> = "hola".into();
    let numeros: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    println!("Box<str>: {} bytes", size_of_val(&texto));
    println!("Box<[i32]>: {} bytes", size_of_val(&numeros));
}
