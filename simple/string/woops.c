// in `woops.c`

#include <stdio.h>
#include <ctype.h>

void uppercase(char *s) {
    // this is peak C right there
    do {
        *s = toupper(*s);
    } while (*s++);
}

int main(int argc, char **argv) {
    char *arg = argv[1];

    char *upp = arg;
    uppercase(upp);

    printf("upp = %s\n", upp);
}