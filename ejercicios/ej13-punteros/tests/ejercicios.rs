//! Pruebas de los ejercicios del capítulo 13.
//! Ejecute `cargo test -p ej13-punteros` hasta que pasen.

use ej13_punteros::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn stack_es_lifo() {
    let mut p = Stack::new();
    assert!(p.is_empty());
    p.push(1);
    p.push(2);
    p.push(3);
    assert_eq!(p.len(), 3);
    assert_eq!(p.peek(), Some(3));
    assert_eq!(p.pop(), Some(3));
    assert_eq!(p.pop(), Some(2));
    assert_eq!(p.len(), 1);
    assert_eq!(p.pop(), Some(1));
    assert_eq!(p.pop(), None);
    assert!(p.is_empty());
}

#[test]
fn repartir_comparte_sin_copiar() {
    let texto = Rc::new(String::from("compartido"));
    let copias = repartir(&texto, 3);
    assert_eq!(copias.len(), 3);
    assert_eq!(duenos(&texto), 4);
    assert!(copias.iter().all(|c| Rc::ptr_eq(c, &texto)));
    drop(copias);
    assert_eq!(duenos(&texto), 1);
}

#[test]
fn contador_compartido() {
    let a = Contador::new();
    let b = a.compartir();
    a.incrementar();
    b.incrementar();
    b.incrementar();
    assert_eq!(a.valor(), 3);
    assert_eq!(b.valor(), 3);
    let c = Contador::new();
    assert_eq!(c.valor(), 0);
}

fn reg(n: &str, r: &Rc<RefCell<Vec<String>>>) -> Registrador {
    Registrador {
        nombre: n.to_string(),
        registro: Rc::clone(r),
    }
}

#[test]
fn drop_registra_el_orden() {
    let registro = Rc::new(RefCell::new(Vec::new()));
    {
        let _a = reg("a", &registro);
        {
            let _b = reg("b", &registro);
            let _c = reg("c", &registro);
        }
        let _d = reg("d", &registro);
    }
    assert_eq!(*registro.borrow(), ["c", "b", "d", "a"]);
}

#[test]
fn arbol_camino_a_raiz() {
    let raiz = nuevo_nodo(1);
    let medio = nuevo_nodo(2);
    let hoja = nuevo_nodo(3);
    agregar_hijo(&raiz, &medio);
    agregar_hijo(&medio, &hoja);
    assert_eq!(camino_a_raiz(&hoja), [3, 2, 1]);
    assert_eq!(camino_a_raiz(&raiz), [1]);
    assert_eq!(Rc::strong_count(&medio), 2);
    assert_eq!(Rc::weak_count(&medio), 1);
}

#[test]
fn arbol_padre_no_es_dueno() {
    let hoja = nuevo_nodo(3);
    {
        let raiz = nuevo_nodo(1);
        agregar_hijo(&raiz, &hoja);
        assert_eq!(camino_a_raiz(&hoja), [3, 1]);
    }
    // La raíz se liberó: el Weak de la hoja ya no la alcanza.
    assert!(hoja.padre.borrow().upgrade().is_none());
    assert_eq!(camino_a_raiz(&hoja), [3]);
}
