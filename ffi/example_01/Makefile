GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./csrc/hello

clean: 
	$(CARGO_BIN) clean
	rm -f ./csrc/hello

build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -o ./csrc/hello ./csrc/hello.c -Isrc -L. -l:target/release/libexample_01.so