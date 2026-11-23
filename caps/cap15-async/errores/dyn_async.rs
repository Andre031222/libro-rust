trait Sensor {
    async fn leer(&self) -> u32;
}

fn usar(s: &dyn Sensor) {
    let _ = s.leer();
}

fn main() {}
