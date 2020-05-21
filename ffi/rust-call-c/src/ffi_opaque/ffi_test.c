#include "ffi_test.h"

struct object {
  int info;
};

/**
 * Returns the size of the object struct.
 */
size_t sizeof_obj(void) {
  return sizeof(struct object);
}

/**
 * Allocates memory for a new object on the heap and returns a pointer to it.
 */
struct object* init(void) {
  struct object* obj = (struct object*)malloc(sizeof_obj());
  obj->info = 0;

  return obj;
}

/**
 * Frees the memory allocated for the object.
 */
void free_object(struct object* obj) {
  free(obj);
}

/**
 * Returns the version of the C API used by this library.
 */
int get_api_version(void) {
  return 0;
}

/**
 * Returns the value of the info field of the given object struct.
 */
int get_info(const struct object* obj) {
  return obj->info;
}

/**
 * Sets the info field of the object struct.
 */
void set_info(struct object* obj, int arg) {
  obj->info = arg;
}