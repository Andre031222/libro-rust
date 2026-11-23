//! La optimización de nicho que std garantiza para Option.

use std::mem::size_of;

fn main() {
    // Una referencia nunca es nula: None usa ese valor imposible.
    println!("&u64             {}", size_of::<&u64>());
    println!("Option<&u64>     {}", size_of::<Option<&u64>>());
    println!("Box<u64>         {}", size_of::<Box<u64>>());
    println!(
        "Option<Box<u64>> {}",
        size_of::<Option<Box<u64>>>()
    );
    // u64 admite cualquier patrón de bits: no hay nicho libre.
    println!("u64              {}", size_of::<u64>());
    println!("Option<u64>      {}", size_of::<Option<u64>>());
}
