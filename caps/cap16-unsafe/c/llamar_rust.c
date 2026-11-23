// Un programa en C que llama a una función escrita en Rust.
#include <stdio.h>

// Declaración de la función que exporta el crate.
int c16_sumar(int a, int b);

int main(void) {
    printf("c16_sumar(2, 40) = %d\n", c16_sumar(2, 40));
    return 0;
}
