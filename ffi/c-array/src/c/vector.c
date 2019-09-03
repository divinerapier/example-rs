#include "vector.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

vector vector_create(int unit, int count) {
    void *data = malloc(unit * count);
    vector v;
    v.cap = count;
    v.len = 0;
    v.data = data;
    v.unit = unit;
    return v;
}

void *vector_get(vector *v, int index) {
    if (v == NULL) {
        return NULL;
    }
    if (index > v->len) {
        return NULL;
    }
    return (v->data + (index * v->unit));
}

void *vector_push(vector *v, ...) {
    va_list arguments;
    va_start(arguments, v);

    // for (int x = 0; x < num; x++) {
    //     sum += va_arg(arguments, void);
    // }
    va_end(arguments);
    // Cleans up the list
    return NULL;
}

void *vector_push_arg(vector *v, void *data) {
    if (NULL == data) {
        return v;
    }
    if (v->len == v->cap) {
        // grow
        vector tmp = vector_create(v->unit, v->cap * 2);
        for (int i = 0; i < v->len; i++) {
            vector_push(&tmp, vector_get(v, i));
        }
        vector_destroy(v);
        v->len = tmp.len;
        v->cap = tmp.cap;
        v->unit = tmp.unit;
        v->data = tmp.data;
    }
    memcpy(v->data + (v->len * v->unit), data, v->unit);
    v->len += 1;
    return v;
}

void vector_destroy(vector *v) {
    if (v == NULL) {
        return;
    }
    free(v->data);
    v->len = 0;
    v->cap = 0;
    v->unit = 0;
    v->data = NULL;
    return;
}

int vector_iterate(vector *v, int start, int skip, callback1 cb) {
    if (v == NULL) {
        return 0;
    }
    for (int i = start; i < v->len; i += skip) {
        int result = cb(v->data + i * v->unit);
        if (0 != result) {
            return result;
        }
    }
    return 0;
}
