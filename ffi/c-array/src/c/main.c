#include "vector.h"

int main() {
    vector v = vector_create(sizeof(int), 10);

    vector_push(&v, 1);

    vector_destroy(&v);
    return 0;
}