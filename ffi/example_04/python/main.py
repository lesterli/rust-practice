# coding: utf-8

try:
    from cffi import FFI
except ImportError:
    print "pip install cffi"

ffi = FFI()

ffi.cdef("""
    int fibonacci(int);
    int count_char(char *s);
""")

lib = ffi.dlopen("../ffi/target/debug/libexample_04.so")

print "fibonacci(2) from Rust: ", lib.fibonacci(2)
print "fibonacci(4) from Rust: ", lib.fibonacci(4)
print "fibonacci(6) from Rust: ", lib.fibonacci(6)

print 'count_char("hello") from Rust: ', lib.count_char("hello")
print 'count_char("你好") from Rust: ', lib.count_char(u"你好".encode('utf-8'))