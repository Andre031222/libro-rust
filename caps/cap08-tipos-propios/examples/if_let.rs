//! if let y let-else: cuando solo interesa un patrón.

fn buscar(v: &[i32], x: i32) -> Option<usize> {
    v.iter().position(|&e| e == x)
}

fn doble_del_primero(v: &[i32]) -> i32 {
    // let-else: si no hay primer elemento, sale con 0
    let Some(p) = v.first() else { return 0 };
    p * 2
}

fn main() {
    if let Some(i) = buscar(&[4, 8, 15], 8) {
        println!("8 está en la posición {i}");
    }
    println!("{}", doble_del_primero(&[]));
}
