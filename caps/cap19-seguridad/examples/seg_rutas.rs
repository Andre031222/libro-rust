use cap19_seguridad::rutas::{ruta_ingenua, ruta_segura};
use std::path::Path;

fn main() {
    let base = Path::new("/srv/informes");
    // Nombres que podría enviar un usuario en una petición
    let pedidos =
        ["marzo.txt", "../../etc/passwd", "/etc/passwd"];
    for nombre in pedidos {
        println!("{nombre}");
        let r = ruta_ingenua(base, nombre);
        println!("  ingenua: {}", r.display());
        match ruta_segura(base, nombre) {
            Ok(r) => println!("  segura:  {}", r.display()),
            Err(e) => println!("  segura:  rechazada ({e:?})"),
        }
    }
}
