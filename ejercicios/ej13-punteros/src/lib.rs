//! Ejercicios del capítulo 13: punteros inteligentes.
//!
//! Reemplace cada `todo!()` por su solución y ejecute
//! `cargo test -p ej13-punteros`.

// Parámetros y campos aún no se usan mientras haya `todo!()`.
#![allow(unused_variables, dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Un nodo de la pila: un valor y la caja del nodo de abajo.
pub struct Nodo {
    pub valor: i32,
    pub abajo: Option<Box<Nodo>>,
}

/// Ejercicio 13.4. Una pila (LIFO) hecha con `Option<Box<Nodo>>`.
pub struct Stack {
    cima: Option<Box<Nodo>>,
    largo: usize,
}

impl Stack {
    /// Crea una pila vacía.
    pub fn new() -> Self {
        Stack {
            cima: None,
            largo: 0,
        }
    }

    /// Apila `valor` encima de todo. Pista: `Option::take`.
    pub fn push(&mut self, valor: i32) {
        todo!()
    }

    /// Quita y devuelve el valor de la cima, o `None`.
    pub fn pop(&mut self) -> Option<i32> {
        todo!()
    }

    /// Devuelve el valor de la cima sin quitarlo.
    pub fn peek(&self) -> Option<i32> {
        todo!()
    }

    /// Número de elementos.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// `true` si la pila no tiene elementos.
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

/// Ejercicio 13.4 (segunda parte). Libera los nodos uno a uno, sin
/// la recursión del `Drop` automático de las cajas encadenadas.
impl Drop for Stack {
    fn drop(&mut self) {
        // Vacíe la pila con un bucle: saque cada nodo con `take`.
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

/// Ejercicio 13.5. Devuelve `n` punteros nuevos al mismo texto
/// que `original`, sin copiar el texto. Use `Rc::clone`.
pub fn repartir(
    original: &Rc<String>,
    n: usize,
) -> Vec<Rc<String>> {
    todo!()
}

/// Ejercicio 13.5 (segunda parte). Número de dueños de `valor`.
pub fn duenos(valor: &Rc<String>) -> usize {
    todo!()
}

/// Ejercicio 13.6. Un contador que varios dueños comparten.
pub struct Contador {
    valor: Rc<RefCell<i32>>,
}

impl Contador {
    /// Crea un contador en cero.
    pub fn new() -> Self {
        Contador {
            valor: Rc::new(RefCell::new(0)),
        }
    }

    /// Devuelve otro `Contador` que comparte el mismo valor.
    pub fn compartir(&self) -> Contador {
        todo!()
    }

    /// Suma uno al valor compartido. Observe que recibe `&self`.
    pub fn incrementar(&self) {
        todo!()
    }

    /// Lee el valor compartido.
    pub fn valor(&self) -> i32 {
        todo!()
    }
}

impl Default for Contador {
    fn default() -> Self {
        Self::new()
    }
}

/// Ejercicio 13.7. Al liberarse, anota su nombre en el registro.
pub struct Registrador {
    pub nombre: String,
    pub registro: Rc<RefCell<Vec<String>>>,
}

impl Drop for Registrador {
    fn drop(&mut self) {
        // Añada `self.nombre` al final del registro.
    }
}

/// Reto 13.8. Nodo de un árbol: los hijos son dueños (`Rc`) y el
/// padre se referencia sin ser dueño (`Weak`).
pub struct NodoArbol {
    pub valor: i32,
    pub padre: RefCell<Weak<NodoArbol>>,
    pub hijos: RefCell<Vec<Rc<NodoArbol>>>,
}

/// Reto 13.8. Crea un nodo sin padre ni hijos.
pub fn nuevo_nodo(valor: i32) -> Rc<NodoArbol> {
    todo!()
}

/// Reto 13.8. Cuelga `hijo` de `padre`: el padre guarda un `Rc`
/// al hijo y el hijo, un `Weak` al padre.
pub fn agregar_hijo(padre: &Rc<NodoArbol>, hijo: &Rc<NodoArbol>) {
    todo!()
}

/// Reto 13.8. Valores desde `hoja` hasta la raíz, ambos incluidos.
pub fn camino_a_raiz(hoja: &Rc<NodoArbol>) -> Vec<i32> {
    todo!()
}
