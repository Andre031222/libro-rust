//! Tuplas y arreglos.

fn main() {
    // Una tupla agrupa valores de tipos distintos.
    let punto: (i32, f64, char) = (3, 2.5, 'x');
    let (a, b, c) = punto; // desestructuración
    println!("a = {a}, b = {b}, c = {c}, punto.1 = {}", punto.1);

    // Un arreglo: longitud fija, un solo tipo.
    let primos: [u32; 5] = [2, 3, 5, 7, 11];
    let ceros = [0u8; 4]; // cuatro ceros
    println!("primos[0] = {}, len = {}", primos[0], primos.len());
    println!("ceros = {ceros:?}, get(9) = {:?}", primos.get(9));
}
