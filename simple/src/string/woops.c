#include <stdio.h>

int len(char *s) {
    s[0] = '\0';
    return 0;
}

int main(int argc, char **argv) {
    char *arg = argv[1];
    int l = len(arg);
    printf("length of \"%s\" = %d\n", arg, l);
}