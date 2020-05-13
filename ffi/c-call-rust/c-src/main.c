#include "c_call_rust.h"
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

int main(void) {
    call_from_rust();

    int my_array[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    int total = sum(my_array, 10);
    printf("The total is %d\n", total);
    return 0;
}  
