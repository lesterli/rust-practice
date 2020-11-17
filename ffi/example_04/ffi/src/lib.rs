use std::os::raw::{c_int};

#[no_mangle]
pub extern "C" fn fibonacci(index: c_int) -> c_int {
    return if index <= 2 { 1 } else { fibonacci(index - 1) + fibonacci(index - 2) };
}