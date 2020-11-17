## example_04

在 Python 中调用 Rust 代码的示例。

* Python FFI 库有 [ctypes](https://docs.python.org/3/library/ctypes.html) 和 [cffi](https://cffi.readthedocs.io/en/latest/) 库

### 示例说明

* `src` 目录： Rust 源码
* `ffi` 目录： Rust 源码导出的 FFI 代码
* `python` 目录： Python 源码
* 本示例采用的是 `cffi` 库

### 运行步骤

1. 在 `ffi` 目录下，执行 `cargo build` 生成 Rust 源码的动态库(Linux平台，`libexample_04.so`)。
2. 在 `python` 目录下，运行 `python main.py`。



