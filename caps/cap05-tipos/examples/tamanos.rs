//! char frente a u8, y cuánto ocupa cada uno.

use std::mem::size_of;

fn main() {
    let letra: char = 'ñ';
    let byte: u8 = b'n'; // un byte ASCII
    println!("char: {} B", size_of::<char>());
    println!("u8:   {} B", size_of::<u8>());
    println!("'{letra}' = U+{:04X}", letra as u32);
    println!("'{letra}' ocupa {} B en UTF-8", letra.len_utf8());
    println!("'a' ocupa {} B en UTF-8", 'a'.len_utf8());
    println!("b'n' = {byte}");
}
