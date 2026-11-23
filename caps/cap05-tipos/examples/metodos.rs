//! Cuatro maneras explícitas de tratar el desbordamiento.

fn main() {
    let a: u8 = 250;
    let b: u8 = 10;

    println!("checked:     {:?}", a.checked_add(b));
    println!("wrapping:    {}", a.wrapping_add(b));
    println!("saturating:  {}", a.saturating_add(b));
    println!("overflowing: {:?}", a.overflowing_add(b));

    // Si cabe, todas dan el resultado exacto.
    println!("checked:     {:?}", a.checked_add(5));
}
