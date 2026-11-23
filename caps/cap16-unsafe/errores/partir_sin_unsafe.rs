fn partir_mut(
    v: &mut [i32],
    mitad: usize,
) -> (&mut [i32], &mut [i32]) {
    (&mut v[..mitad], &mut v[mitad..])
}

fn main() {
    let mut v = [1, 2, 3, 4];
    let (a, b) = partir_mut(&mut v, 2);
    a[0] = b[0];
}
