//! En qué orden se liberan los valores.

struct Ruido(&'static str);

impl Drop for Ruido {
    fn drop(&mut self) {
        println!("  libero {}", self.0);
    }
}

fn main() {
    let _a = Ruido("a");
    let b = Ruido("b");
    let _c = Ruido("c");
    println!("antes de drop(b)");
    drop(b); // std::mem::drop: libera ahora
    println!("fin de main");
}
