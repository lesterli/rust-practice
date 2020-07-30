#include <stdio.h>
#include <float.h>
#include "example_02.h"

int main(void) {
  char version[] = "v1";
  printf("C call Rust Result: %d\n", handle_result(version));
  *(version+1) = '2';
  printf("C call Rust Result: %d\n", handle_result(version));
  char *n_version = NULL;
  printf("C call Rust Result error: %d\n", handle_result(n_version));

  float x = 2.0, y = 3.0;
  printf("C call Rust Option value: %d\n", handle_option(x, y));
  x = 2.0, y = 0.0;
  printf("C call Rust Option None: %d\n", handle_option(x, y));

  printf("C call Rust panic: %d\n", no_panic());
}