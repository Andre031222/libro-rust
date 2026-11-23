//! Métodos con self, &self y &mut self.

#[derive(Debug)]
struct Cuenta {
    titular: String,
    saldo: i64,
}

impl Cuenta {
    // Función asociada: no recibe self; se llama Cuenta::new.
    fn new(titular: String) -> Self {
        Self { titular, saldo: 0 }
    }
    fn saldo(&self) -> i64 {
        self.saldo // solo lee
    }
    fn depositar(&mut self, monto: i64) {
        self.saldo += monto; // modifica
    }
    fn cerrar(self) -> String {
        self.titular // consume la cuenta
    }
}

fn main() {
    let mut c = Cuenta::new(String::from("Grace"));
    c.depositar(150);
    println!("saldo: {} ({c:?})", c.saldo());
    println!("cerrada la cuenta de {}", c.cerrar());
}
