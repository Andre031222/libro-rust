//! Una lista enlazada: cada nodo guarda el siguiente en una caja.

struct Nodo {
    valor: i32,
    siguiente: Option<Box<Nodo>>,
}

fn main() {
    // Se construye de atrás hacia adelante: 3, luego 2, luego 1.
    let mut cabeza: Option<Box<Nodo>> = None;
    for valor in [3, 2, 1] {
        let nodo = Nodo {
            valor,
            siguiente: cabeza,
        };
        cabeza = Some(Box::new(nodo));
    }

    // Se recorre con una referencia que avanza nodo a nodo.
    let mut actual = &cabeza;
    let mut suma = 0;
    while let Some(nodo) = actual {
        print!("{} -> ", nodo.valor);
        suma += nodo.valor;
        actual = &nodo.siguiente;
    }
    println!("fin");
    println!("suma = {suma}");
    let bytes = size_of::<Option<Box<Nodo>>>();
    println!("Option<Box<Nodo>> ocupa {bytes} bytes");
}
