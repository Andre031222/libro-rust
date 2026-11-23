// Este archivo NO compila a propósito: muestra el error E0597.
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // aquí se libera x
    println!("{r}");
}
