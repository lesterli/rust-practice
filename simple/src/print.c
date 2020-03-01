// in `print.c`

#include <stdio.h> // printf
#include <stdint.h> // uint8_t

void print_spaced(char *s) {
    // start at the beginning
    int i = 0;

    while (1) {
        // we're going to be shifting bytes around,
        // so treat them like unsigned 8-bit values
        uint8_t c = s[i];
        if (c == 0) {
            // reached null terminator, stop printing
            break;
        }

        // length of the sequence, ie., number of bytes
        // that encode a single Unicode scalar value
        int len = 1;
        if (c >> 5 == 0b110) {
            len = 2;
        } else if (c >> 4 == 0b1110) {
            len = 3;
        } else if (c >> 3 == 0b11110) {
            len = 4;
        }

        // print the entire UTF-8-encoded Unicode scalar value
        for (; len > 0; len--) {
            printf("%c", s[i]);
            i++;
        }
        // print space separator
        printf(" ");
    }
}

int main(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        print_spaced(argv[i]);
        printf("\n");
    }

    return 0;
}