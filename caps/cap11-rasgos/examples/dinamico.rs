//! Objetos rasgo: canales de aviso de tipos distintos.

trait Notificar {
    fn enviar(&self, msg: &str) -> String;
}

struct Correo(String);
struct Sms(u32);

impl Notificar for Correo {
    fn enviar(&self, msg: &str) -> String {
        format!("correo a {}: {msg}", self.0)
    }
}

impl Notificar for Sms {
    fn enviar(&self, msg: &str) -> String {
        format!("SMS al {}: {msg}", self.0)
    }
}

fn main() {
    let canales: Vec<Box<dyn Notificar>> = vec![
        Box::new(Correo("ana@ejemplo.org".into())),
        Box::new(Sms(987_654_321)),
    ];
    for c in &canales {
        println!("{}", c.enviar("pedido listo"));
    }
    println!("&Sms: {} bytes", size_of::<&Sms>());
    println!("&dyn: {} bytes", size_of::<&dyn Notificar>());
}
