//! Una función genérica usada con tres tipos distintos.

// T: cualquier tipo que se pueda comparar y copiar.
fn maximo<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut mayor = lista[0];
    for &x in lista {
        if x > mayor {
            mayor = x;
        }
    }
    mayor
}

fn main() {
    println!("{}", maximo(&[3, 9, 4]));
    println!("{}", maximo(&[2.5, -1.0, 0.75]));
    println!("{}", maximo(&['r', 'u', 's', 't']));
}
