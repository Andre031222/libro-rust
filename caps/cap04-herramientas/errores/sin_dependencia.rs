use rand::random;

fn main() {
    let dado: u8 = random::<u8>() % 6 + 1;
    println!("dado: {dado}");
}
