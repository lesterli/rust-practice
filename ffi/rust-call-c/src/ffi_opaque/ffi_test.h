#ifndef FFI_TEST_H
#define FFI_TEST_H

#include <stdlib.h>

struct object;

struct object* init(void);
void free_object(struct object*);
int get_api_version(void);
int get_info(const struct object*);
void set_info(struct object*, int);
size_t sizeof_obj(void);

#endif /* FFI_TEST_H */