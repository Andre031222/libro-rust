#![no_std]

pub fn pares(n: u32) -> Vec<u32> {
    let v: Vec<u32> = (0..n).filter(|x| x % 2 == 0).collect();
    println!("{} pares", v.len());
    v
}
