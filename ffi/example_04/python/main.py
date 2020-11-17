try:
    from cffi import FFI
except ImportError:
    print "pip install cffi"

ffi = FFI()
lib = ffi.dlopen("../ffi/target/debug/libexample_04.so")
ffi.cdef("""
    int fibonacci(int);
""")

print "fibonacci(2) from Rust: ", lib.fibonacci(2)
print "fibonacci(4) from Rust: ", lib.fibonacci(4)
print "fibonacci(6) from Rust: ", lib.fibonacci(6)
