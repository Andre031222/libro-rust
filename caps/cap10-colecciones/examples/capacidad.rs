//! Longitud y capacidad de un `Vec` mientras crece.

fn main() {
    let mut v: Vec<u64> = Vec::new(); // aún no reserva memoria
    println!("len {:>2}  cap {:>2}", v.len(), v.capacity());
    for i in 0..9 {
        v.push(i);
        println!("len {:>2}  cap {:>2}", v.len(), v.capacity());
    }

    // Si se conoce el tamaño, se reserva de una vez.
    let w: Vec<u64> = Vec::with_capacity(100);
    println!("w: len {}  cap {}", w.len(), w.capacity());
}
