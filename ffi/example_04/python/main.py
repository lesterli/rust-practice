# coding: utf-8

try:
    from cffi import FFI
except ImportError:
    print "pip install cffi"

ffi = FFI()

ffi.cdef("""
    typedef struct c_tuple {
        unsigned int integer;
        bool boolean;
    } c_tuple;
    unsigned int fibonacci(unsigned int index);
    unsigned int count_char(const char *s);
    struct c_tuple handle_tuple(struct c_tuple tup);
    int sum_of_even(const int *ptr, size_t len);
""")

lib = ffi.dlopen("../ffi/target/debug/libexample_04.so")

print "fibonacci(2) from Rust: ", lib.fibonacci(2)
print "fibonacci(4) from Rust: ", lib.fibonacci(4)
print "fibonacci(6) from Rust: ", lib.fibonacci(6)

print 'count_char("hello") from Rust: ', lib.count_char("hello")
print 'count_char("你好") from Rust: ', lib.count_char(u"你好".encode('utf-8'))


py_cdata = ffi.new('c_tuple *')
py_cdata.integer = 100
py_cdata.boolean = True
print('cdata = {0}, {1}'.format(py_cdata.integer, py_cdata.boolean))
new_py_cdata = lib.handle_tuple(py_cdata[0])
print('change cdata = {0}, {1}'.format(new_py_cdata.integer, new_py_cdata.boolean))


array = ffi.new("int[]", [1, 4, 9, 16, 25])
print 'sum_of_even from Rust: ', lib.sum_of_even(array, len(array))

