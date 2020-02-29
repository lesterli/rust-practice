// in `print.c`

#include <stdio.h> // for printf

int main(int argc, char **argv) {
    printf("argv = %p\n", argv); // new!
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        printf("argv[%d] = %p\n", i, argv[i]); // new!
        printf("%s\n", arg);
    }

    return 0;
}