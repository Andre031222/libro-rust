mod precios {
    pub fn precio_final(c: u64) -> u64 {
        redondear(c * 118)
    }

    fn redondear(x: u64) -> u64 {
        (x + 50) / 100
    }
}

fn main() {
    println!("{}", precios::precio_final(1000));
    println!("{}", precios::redondear(250));
}
