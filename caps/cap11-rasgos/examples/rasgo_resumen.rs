//! Un rasgo con un método obligatorio y otro por defecto.

trait Resumen {
    fn autor(&self) -> String; // obligatorio
    fn resumir(&self) -> String {
        format!("(Leer más de {}...)", self.autor())
    }
}

struct Articulo(String); // nombre del medio
struct Mensaje(String); // nombre de usuario

impl Resumen for Articulo {
    fn autor(&self) -> String {
        self.0.clone()
    }
}

impl Resumen for Mensaje {
    fn autor(&self) -> String {
        format!("@{}", self.0)
    }
}

fn main() {
    let a = Articulo("La Gaceta".into());
    let m = Mensaje("ferris".into());
    println!("{}", a.resumir());
    println!("{}", m.resumir());
}
