// in `print.c`
//lyys-MacBook-Pro:src lyy$ ./print "ready" "set" "go"
//./printreadys
//readysetgoTE
//setgoTERM_PRO
//goTERM_PROGRAM
//$ # note: "-g 1" means "show groups of one byte",
//$ # xxd defaults to "-g 2".
// lyys-MacBook-Pro:src lyy$ ./print "ready" "set" "go" | xxd -g 1
//00000000: 2e 2f 70 72 69 6e 74 00 72 65 61 64 79 00 73 0a  ./print.ready.s.
//00000010: 72 65 61 64 79 00 73 65 74 00 67 6f 00 54 45 0a  ready.set.go.TE.
//00000020: 73 65 74 00 67 6f 00 54 45 52 4d 5f 50 52 4f 0a  set.go.TERM_PRO.
//00000030: 67 6f 00 54 45 52 4d 5f 50 52 4f 47 52 41 4d 0a  go.TERM_PROGRAM.

#include <stdio.h> // for printf

int main(int argc, char **argv) {
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        // note: the loop condition is gone, we just loop forever.
        // well, until a 'break' at least.
        for (int j = 0;; j++) {
            char character = arg[j];
            // technically, we ought to use '\0' rather than just 0,
            // but even `gcc -Wall -Wextra -Wpedantic` doesn't chastise
            // us, so let's just go with it.
            if (character == 0) {
                break;
            }
            printf("%c", character);
        }
        printf("\n");
    }

    return 0;
}