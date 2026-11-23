trait Duplicar {
    fn duplicar(&self) -> Self;
}

impl Duplicar for i32 {
    fn duplicar(&self) -> Self {
        self * 2
    }
}

fn main() {
    let _x: Box<dyn Duplicar> = Box::new(5);
}
