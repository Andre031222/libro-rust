//! Los elementos viajan uno a uno por toda la tubería.

fn main() {
    let v: Vec<i32> = (1..=4)
        .filter(|n| {
            println!("filter {n}");
            n % 2 == 0
        })
        .map(|n| {
            println!("  map {n}");
            n * 10
        })
        .collect();
    println!("{v:?}");
}
