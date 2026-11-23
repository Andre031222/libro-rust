mod tienda {
    // Privada: la ven `tienda` y sus descendientes.
    fn comision() -> u32 {
        2
    }
    pub mod caja {
        pub fn cobrar(monto: u32) -> u32 {
            monto + super::comision() // sube a `tienda`
        }
    }
    pub fn abrir() -> u32 {
        self::caja::cobrar(10) // parte del módulo actual
    }
}

// Atajo válido en este ámbito; la ruta parte de la raíz.
use crate::tienda::caja::cobrar;

fn main() {
    println!("al abrir: {}", tienda::abrir());
    println!("al cobrar 5: {}", cobrar(5));
}
