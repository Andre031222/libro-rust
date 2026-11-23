// Este archivo NO compila a propósito: muestra el error E0004.
enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}

fn accion(s: Semaforo) -> &'static str {
    match s {
        Semaforo::Rojo => "detenerse",
        Semaforo::Verde => "avanzar",
    }
}

fn main() {
    println!("{}", accion(Semaforo::Amarillo));
}
