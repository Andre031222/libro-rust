use std::rc::Rc;
use std::thread;

fn main() {
    let dato = Rc::new(5);
    let copia = Rc::clone(&dato);
    let hilo = thread::spawn(move || println!("{copia}"));
    hilo.join().unwrap();
}
