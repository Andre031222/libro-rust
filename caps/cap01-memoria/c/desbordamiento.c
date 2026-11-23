// Desbordamiento de búfer en la pila: strcpy no conoce el
// tamaño del destino y escribe más allá de sus 8 bytes.
#include <stdio.h>
#include <string.h>

int main(void) {
    char clave[8];
    int acceso = 0;  // vecino en la pila

    strcpy(clave, "contraseña-larga");  // 18 bytes en 8
    printf("acceso = %d\n", acceso);
    return 0;
}
