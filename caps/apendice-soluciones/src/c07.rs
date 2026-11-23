// Ejercicio 7.5: `retain` cambia la longitud del vector, algo
// que una rebanada `&mut [i32]` no permite.
pub fn depurar(v: &mut Vec<i32>) {
    v.retain(|&x| x >= 0);
    for x in v.iter_mut() {
        *x *= 2;
    }
}
