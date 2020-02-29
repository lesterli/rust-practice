// in `print.c`

#include <stdio.h> // for printf

int main(int argc, char **argv) {
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        printf("%s\n", arg);
    }

    return 0;
}