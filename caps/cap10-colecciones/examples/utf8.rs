//! Un `String` guarda bytes UTF-8, no letras de tamaño fijo.

fn main() {
    let s = String::from("año");
    println!("bytes: {}", s.len()); // 4: la ñ ocupa dos
    println!("caracteres: {}", s.chars().count());
    println!("{:?}", s.bytes().collect::<Vec<u8>>());

    // Rebanar por bytes: válido solo en límites de carácter.
    println!("{}", &s[0..3]); // "añ"
    println!("{:?}", s.get(0..2)); // None: corta la ñ

    // Para recorrer letras se usa chars, no índices.
    for (i, c) in s.char_indices() {
        println!("byte {i}: {c}");
    }
}
