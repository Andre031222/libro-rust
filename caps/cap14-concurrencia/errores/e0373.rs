use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let hilo = thread::spawn(|| println!("{v:?}"));
    hilo.join().unwrap();
}
