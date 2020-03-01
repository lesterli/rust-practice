// in `woops.c`

#include <stdio.h>

int len(char *s) {
    int l = 0;
    while (s[l]) {
        l++;
    }
    return l;
}

int main(int argc, char **argv) {
    char *arg = argv[1];
    int l = len(arg);
    printf("length of \"%s\" = %d\n", arg, l);
}