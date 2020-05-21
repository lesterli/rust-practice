CC=gcc
CFLAGS=-Wall -Werror
OBJ=libffi_test.o

libffi_test.so: $(OBJ)
	gcc -shared -o $@ $(OBJ)

$(OBJ): ffi_test.c ffi_test.h
	$(CC) -c -o $(OBJ) -fpic $< $(CFLAGS)