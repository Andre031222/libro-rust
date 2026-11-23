use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let almacen = Arc::new(Mutex::new(Vec::new()));
    for i in 0..3 {
        thread::spawn(move || {
            almacen.lock().unwrap().push(i);
        });
    }
}
