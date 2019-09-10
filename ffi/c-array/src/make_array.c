#include <stdlib.h>

int make_array(int **array, int *length) {
    if (array == NULL || length == NULL) {
        return 1;
    }
    *length = 5;
    *array = (int *)malloc(5 * sizeof(int));
    for (int i = 0; i < *length; i++) {
        (*array)[i] = i;
    }
    return 0;
}