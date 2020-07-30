GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./csrc/main

clean: 
	$(CARGO_BIN) clean
	rm -f ./csrc/main

build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -o ./csrc/main ./csrc/main.c -Isrc -L. -l:target/release/libexample_02.so