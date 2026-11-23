//! ESQUELETO ILUSTRATIVO de un núcleo. No es parte del
//! workspace: para arrancarlo hacen falta un objetivo sin
//! sistema operativo, un cargador de arranque y un emulador.

#![no_std] // sin la biblioteca estándar
#![no_main] // sin `main`: nadie va a llamarla

use core::panic::PanicInfo;

/// Punto de entrada. El cargador de arranque salta aquí con
/// la CPU ya en modo largo y una pila preparada.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Aquí empezaría el núcleo: pantalla, interrupciones,
    // tabla de páginas, asignador de marcos...
    loop {
        core::hint::spin_loop();
    }
}

/// Sin `std`, nadie maneja los pánicos: lo hace el núcleo.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
