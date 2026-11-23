// Compila y suele "funcionar", pero lee fuera del arreglo:
// es comportamiento indefinido. Miri lo detecta.
fn main() {
    let v = [1, 2, 3];
    let p = v.as_ptr();
    // SAFETY (falsa): se supone que hay un valor en `p.add(3)`,
    // pero el arreglo solo tiene índices 0, 1 y 2.
    let x = unsafe { *p.add(3) };
    println!("{x}");
}
