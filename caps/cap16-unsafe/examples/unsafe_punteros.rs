// Crear punteros crudos es seguro; desreferenciarlos, no.
fn main() {
    let mut x = 5;
    let p = &raw mut x; // *mut i32
    // SAFETY: `p` apunta a `x`, que sigue viva y alineada,
    // y ninguna referencia a `x` está activa.
    let leido = unsafe {
        *p += 1;
        *p
    };
    println!("p = {p:?}, *p = {leido}");
    // Compila, pero leer `*suelto` sería UB.
    let suelto = 0x1234 as *const i32;
    println!("suelto = {suelto:?} (no se lee)");
}
