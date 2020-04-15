GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./c-src/main

clean: 
	$(CARGO_BIN) clean
	rm -f ./c-src/main

build:
	$(CARGO_BIN) build
	$(GCC_BIN) -o ./c-src/main ./c-src/main.c -Isrc -L ./target/debug -lc_call_rust