//! Un enum cuyas variantes llevan datos distintos.

enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
    Color(u8, u8, u8),
}

fn describir(m: &Mensaje) -> String {
    match m {
        Mensaje::Salir => "salir".to_string(),
        Mensaje::Mover { x, y } => format!("mover a ({x}, {y})"),
        Mensaje::Escribir(t) => format!("escribir «{t}»"),
        Mensaje::Color(r, g, b) => format!("color {r},{g},{b}"),
    }
}

fn main() {
    let mensajes = [
        Mensaje::Mover { x: 3, y: -1 },
        Mensaje::Escribir(String::from("hola")),
        Mensaje::Color(183, 65, 14),
        Mensaje::Salir,
    ];
    for m in &mensajes {
        println!("{}", describir(m));
    }
}
