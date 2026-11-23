// Dos errores clásicos de C que el compilador no detiene.
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *s1 = malloc(5);
    strcpy(s1, "hola");
    char *s2 = s1;   // dos punteros al mismo bloque

    free(s1);
    s2[0] = 'H';     // uso después de liberar (use-after-free)
    free(s2);        // doble liberación (double free)
    return 0;
}
