//lyys-MacBook-Pro:src lyy$ ./print "élément"
//� � l � � m e n t 
//lyys-MacBook-Pro:src lyy$ ./print "élément"
//élément
//lyys-MacBook-Pro:src lyy$ ./print "élément" | xxd -g 1
//00000000: c3 a9 6c c3 a9 6d 65 6e 74 0a                    ..l..ment.

#include <stdio.h> // printf

int main(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        char *arg = argv[i];
        for (int j = 0;; j++) {
            char character = arg[j];
            if (character == 0) {
                break;
            }
            // notice the space following `%c`
            printf("%c", character);
        }
        printf("\n");
    }

    return 0;
}