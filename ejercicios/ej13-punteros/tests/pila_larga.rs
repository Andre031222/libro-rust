//! Prueba aparte del ejercicio 13.4: si el `Drop` de `Stack` es
//! recursivo, el desborde de la pila de llamadas aborta el
//! proceso, y por eso esta prueba vive en su propio binario.

use ej13_punteros::Stack;

#[test]
fn stack_grande_no_desborda_al_liberarse() {
    let mut p = Stack::new();
    for i in 0..1_000_000 {
        p.push(i);
    }
    assert_eq!(p.peek(), Some(999_999));
    assert_eq!(p.len(), 1_000_000);
} // aquí se libera: un millón de nodos
