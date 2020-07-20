#include <stdio.h>
#include <stdint.h>
#include "example_01.h"

int main(void) {
  char *str = generate("str");
  printf("c side: %s\n", str);
  free_str(str);

  char *ping = generate("ping");
  printf("c side: %s\n", ping);
  free_str(ping);
}