// Fragmento del ejercicio 16.6: le faltan los comentarios
// de seguridad y tiene un caso que provoca UB.
pub fn ultimo(v: &[i32]) -> i32 {
    let i = v.len().wrapping_sub(1);
    unsafe { *v.get_unchecked(i) }
}
