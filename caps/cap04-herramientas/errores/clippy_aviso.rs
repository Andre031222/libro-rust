fn suma(datos: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..datos.len() {
        total += datos[i];
    }
    total
}

fn main() {
    println!("{}", suma(&[1, 2, 3]));
}
