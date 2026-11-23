use std::cell::RefCell;
use std::rc::Rc;

// Ejercicio 13.5: cada clon suma un dueño, no copia el texto.
pub fn repartir(
    original: &Rc<String>,
    n: usize,
) -> Vec<Rc<String>> {
    (0..n).map(|_| Rc::clone(original)).collect()
}

pub fn duenos(valor: &Rc<String>) -> usize {
    Rc::strong_count(valor)
}

// Ejercicio 13.7: el destructor deja constancia del orden.
pub struct Registrador {
    pub nombre: String,
    pub registro: Rc<RefCell<Vec<String>>>,
}

impl Drop for Registrador {
    fn drop(&mut self) {
        self.registro.borrow_mut().push(self.nombre.clone());
    }
}
