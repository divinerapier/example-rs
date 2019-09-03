typedef struct vector {
    int len;
    int cap;
    int unit;
    void *data;
} vector;

typedef int (*callback1)(void *);

vector vector_create(int unit, int count);

void *vector_get(vector *v, int index);

void vector_destroy(vector *);

int vector_iterate(vector *v, int start, int skip, callback1 cb);

void *vector_push(vector *v, ...);